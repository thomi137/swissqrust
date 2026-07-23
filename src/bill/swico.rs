/*
 * Copyright (c) 2026 Thomas Prosser
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */

//! Swico S1 "Billing information" (Rechnungsinformationen): the structured
//! billing information some Swiss accounting software puts into the
//! QR-bill's `StrdBkgInf` field, per Implementation Guidelines 4.3.3 and
//! Annex D. Not part of the SIX standardisation itself - Swico publishes
//! and owns the S1 syntax (http://swiss-qr-invoice.org/downloads/qr-bill-s1-syntax-de.pdf).
//!
//! Field tags and encoding mirror the reference implementation at
//! https://github.com/manuelbl/SwissQRBill (SwicoBillInformation /
//! SwicoS1Encoder / SwicoS1Decoder).

use chrono::{Days, NaiveDate};
use crate::BillError;
use crate::input::InputSwicoBillInformation;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct SwicoBillInformation {
    pub invoice_number: Option<String>,
    pub invoice_date: Option<NaiveDate>,
    pub customer_reference: Option<String>,
    pub vat_number: Option<String>,
    /// Date the goods/service were supplied. Mutually exclusive with
    /// `vat_start_date` / `vat_end_date`.
    pub vat_date: Option<NaiveDate>,
    pub vat_start_date: Option<NaiveDate>,
    pub vat_end_date: Option<NaiveDate>,
    /// VAT rate (percent) if a single rate applies to the whole invoice.
    /// Mutually exclusive with `vat_rate_details`.
    pub vat_rate: Option<f64>,
    /// VAT rate/net-amount tuples, if different rates apply to different
    /// line items.
    pub vat_rate_details: Vec<RateDetail>,
    /// VAT rate/tax-amount tuples for imported goods.
    pub vat_import_taxes: Vec<RateDetail>,
    /// Discount/deadline-in-days tuples (e.g. 2% within 10 days, due in 30).
    pub payment_conditions: Vec<PaymentCondition>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RateDetail {
    pub rate: f64,
    pub amount: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PaymentCondition {
    pub discount: f64,
    pub days: u32,
}

impl SwicoBillInformation {
    /// Encodes this bill information as `StrdBkgInf` text (Swico S1
    /// syntax), or `None` if nothing is set.
    pub fn encode_as_text(&self) -> Option<String> {
        let mut out = String::from("//S1");

        if let Some(v) = &self.invoice_number {
            out.push_str("/10/");
            out.push_str(&escape(v));
        }
        if let Some(d) = self.invoice_date {
            out.push_str("/11/");
            out.push_str(&s1_date(d));
        }
        if let Some(v) = &self.customer_reference {
            out.push_str("/20/");
            out.push_str(&escape(v));
        }
        if let Some(v) = &self.vat_number {
            out.push_str("/30/");
            out.push_str(&escape(v));
        }

        if let Some(d) = self.vat_date {
            out.push_str("/31/");
            out.push_str(&s1_date(d));
        } else if let (Some(start), Some(end)) = (self.vat_start_date, self.vat_end_date) {
            out.push_str("/31/");
            out.push_str(&s1_date(start));
            out.push_str(&s1_date(end));
        }

        if let Some(rate) = self.vat_rate {
            out.push_str("/32/");
            out.push_str(&s1_number(rate));
        } else if !self.vat_rate_details.is_empty() {
            out.push_str("/32/");
            out.push_str(&rate_detail_list(&self.vat_rate_details));
        }

        if !self.vat_import_taxes.is_empty() {
            out.push_str("/33/");
            out.push_str(&rate_detail_list(&self.vat_import_taxes));
        }

        if !self.payment_conditions.is_empty() {
            out.push_str("/40/");
            out.push_str(&condition_list(&self.payment_conditions));
        }

        (out.len() > "//S1".len()).then_some(out)
    }

    /// Decodes `StrdBkgInf` text encoded per Swico S1 syntax. As much data
    /// as possible is decoded; malformed tuples are silently skipped
    /// (matching the reference decoder). Returns `None` if `text` isn't a
    /// Swico S1 payload at all.
    pub fn decode_text(text: &str) -> Option<Self> {
        let body = text.strip_prefix("//S1/")?;
        let parts = split_unescaped(body);

        let mut info = SwicoBillInformation::default();
        for pair in parts.chunks_exact(2) {
            let (tag, value) = (pair[0].as_str(), pair[1].as_str());
            if value.is_empty() {
                continue;
            }
            if let Ok(tag) = tag.parse::<u32>() {
                decode_element(&mut info, tag, value);
            }
        }
        Some(info)
    }

    /// The invoice date plus the number of days of the payment condition
    /// with a 0% discount (the "net" deadline), if both are present.
    pub fn due_date(&self) -> Option<NaiveDate> {
        let invoice_date = self.invoice_date?;
        let net_condition = self.payment_conditions.iter().find(|c| c.discount == 0.0)?;
        invoice_date.checked_add_days(Days::new(net_condition.days as u64))
    }
}

impl TryFrom<InputSwicoBillInformation> for SwicoBillInformation {
    type Error = BillError;

    fn try_from(input: InputSwicoBillInformation) -> Result<Self, Self::Error> {
        Ok(SwicoBillInformation {
            invoice_number: input.invoice_number,
            invoice_date: parse_input_date(input.invoice_date)?,
            customer_reference: input.customer_reference,
            vat_number: input.vat_number,
            vat_date: parse_input_date(input.vat_date)?,
            vat_start_date: parse_input_date(input.vat_start_date)?,
            vat_end_date: parse_input_date(input.vat_end_date)?,
            vat_rate: input.vat_rate,
            vat_rate_details: input
                .vat_rate_details
                .unwrap_or_default()
                .into_iter()
                .map(|d| RateDetail { rate: d.rate, amount: d.amount })
                .collect(),
            vat_import_taxes: input
                .vat_import_taxes
                .unwrap_or_default()
                .into_iter()
                .map(|d| RateDetail { rate: d.rate, amount: d.amount })
                .collect(),
            payment_conditions: input
                .payment_conditions
                .unwrap_or_default()
                .into_iter()
                .map(|c| PaymentCondition { discount: c.discount, days: c.days })
                .collect(),
        })
    }
}

/// Parses an optional "YYYY-MM-DD" input date.
fn parse_input_date(date: Option<String>) -> Result<Option<NaiveDate>, BillError> {
    date.map(|s| {
        NaiveDate::parse_from_str(&s, "%Y-%m-%d").map_err(|_| BillError::InvalidSwicoDate(s.clone()))
    })
    .transpose()
}

fn decode_element(info: &mut SwicoBillInformation, tag: u32, value: &str) {
    match tag {
        10 => info.invoice_number = Some(value.to_string()),
        11 => info.invoice_date = parse_s1_date(value),
        20 => info.customer_reference = Some(value.to_string()),
        30 => info.vat_number = Some(value.to_string()),
        31 => set_vat_dates(info, value),
        32 => set_vat_rate(info, value),
        33 => info.vat_import_taxes = parse_rate_detail_list(value),
        40 => info.payment_conditions = parse_condition_list(value),
        _ => {} // unknown tags are ignored, per spec Annex D
    }
}

fn set_vat_dates(info: &mut SwicoBillInformation, value: &str) {
    match value.len() {
        6 => {
            if let Some(date) = parse_s1_date(value) {
                info.vat_date = Some(date);
                info.vat_start_date = None;
                info.vat_end_date = None;
            }
        }
        12 => {
            let (start, end) = value.split_at(6);
            if let (Some(start), Some(end)) = (parse_s1_date(start), parse_s1_date(end)) {
                info.vat_start_date = Some(start);
                info.vat_end_date = Some(end);
                info.vat_date = None;
            }
        }
        _ => {}
    }
}

fn set_vat_rate(info: &mut SwicoBillInformation, value: &str) {
    if !value.contains(':') && !value.contains(';') {
        if let Some(rate) = parse_decimal(value) {
            info.vat_rate = Some(rate);
            info.vat_rate_details = Vec::new();
        }
    } else {
        info.vat_rate_details = parse_rate_detail_list(value);
        info.vat_rate = None;
    }
}

fn parse_rate_detail_list(value: &str) -> Vec<RateDetail> {
    value
        .split(';')
        .filter_map(|entry| {
            let (rate, amount) = entry.split_once(':')?;
            Some(RateDetail { rate: parse_decimal(rate)?, amount: parse_decimal(amount)? })
        })
        .collect()
}

fn parse_condition_list(value: &str) -> Vec<PaymentCondition> {
    value
        .split(';')
        .filter_map(|entry| {
            let (discount, days) = entry.split_once(':')?;
            Some(PaymentCondition { discount: parse_decimal(discount)?, days: days.parse().ok()? })
        })
        .collect()
}

fn rate_detail_list(list: &[RateDetail]) -> String {
    list.iter()
        .map(|d| format!("{}:{}", s1_number(d.rate), s1_number(d.amount)))
        .collect::<Vec<_>>()
        .join(";")
}

fn condition_list(list: &[PaymentCondition]) -> String {
    list.iter()
        .map(|c| format!("{}:{}", s1_number(c.discount), c.days))
        .collect::<Vec<_>>()
        .join(";")
}

fn escape(text: &str) -> String {
    text.replace('\\', "\\\\").replace('/', "\\/")
}

/// Splits `text` at unescaped slashes, undoing the backslash escaping.
/// Mirrors the reference decoder's placeholder trick: `\\` and `\/` are
/// swapped for characters outside the QR-bill character set before
/// splitting, so an escaped slash never looks like a separator.
fn split_unescaped(text: &str) -> Vec<String> {
    text.replace("\\\\", "\u{2601}")
        .replace("\\/", "\u{2605}")
        .split('/')
        .map(|s| s.replace('\u{2605}', "/").replace('\u{2601}', "\\"))
        .collect()
}

fn s1_date(date: NaiveDate) -> String {
    date.format("%y%m%d").to_string()
}

/// Parses a Swico S1 date. 6 digits (yyMMdd) per spec; 10 and 12 digits
/// (with a time-of-day suffix) are accepted too - not per spec, but seen in
/// production, per the reference implementation.
fn parse_s1_date(text: &str) -> Option<NaiveDate> {
    match text.len() {
        6 => NaiveDate::parse_from_str(text, "%y%m%d").ok(),
        10 => NaiveDate::parse_from_str(text, "%y%m%d%H%M").ok(),
        12 => NaiveDate::parse_from_str(text, "%y%m%d%H%M%S").ok(),
        _ => None,
    }
}

/// Formats like Java's `DecimalFormat("0.###")`: up to 3 decimal places,
/// trailing zeros (and a bare trailing dot) dropped.
fn s1_number(n: f64) -> String {
    let rounded = (n * 1000.0).round() / 1000.0;
    let mut s = format!("{:.3}", rounded);
    while s.ends_with('0') {
        s.pop();
    }
    if s.ends_with('.') {
        s.pop();
    }
    if s.is_empty() || s == "-" {
        s = "0".to_string();
    }
    s
}

fn parse_decimal(s: &str) -> Option<f64> {
    s.parse::<f64>().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    // Test vectors from Implementation Guidelines Annex D, Table 31
    // ("Billing information of Swico, examples").

    #[test]
    fn decodes_spec_example_1() {
        let info = SwicoBillInformation::decode_text(
            "//S1/10/10201409/11/190512/20/1400.000-53/30/106017086/31/180508/32/7.7/40/2:10;0:30",
        )
        .unwrap();

        assert_eq!(info.invoice_number.as_deref(), Some("10201409"));
        assert_eq!(info.invoice_date, Some(date(2019, 5, 12)));
        assert_eq!(info.customer_reference.as_deref(), Some("1400.000-53"));
        assert_eq!(info.vat_number.as_deref(), Some("106017086"));
        assert_eq!(info.vat_date, Some(date(2018, 5, 8)));
        assert_eq!(info.vat_rate, Some(7.7));
        assert_eq!(
            info.payment_conditions,
            vec![
                PaymentCondition { discount: 2.0, days: 10 },
                PaymentCondition { discount: 0.0, days: 30 },
            ]
        );
        assert_eq!(info.due_date(), Some(date(2019, 6, 11)));
    }

    #[test]
    fn decodes_spec_example_2_vat_date_range_and_rate_details() {
        let info = SwicoBillInformation::decode_text(
            "//S1/10/10104/11/180228/30/395856455/31/180226180227/32/3.7:400.19;7.7:553.39;0:14/40/0:30",
        )
        .unwrap();

        assert_eq!(info.vat_start_date, Some(date(2018, 2, 26)));
        assert_eq!(info.vat_end_date, Some(date(2018, 2, 27)));
        assert_eq!(info.vat_date, None);
        assert_eq!(
            info.vat_rate_details,
            vec![
                RateDetail { rate: 3.7, amount: 400.19 },
                RateDetail { rate: 7.7, amount: 553.39 },
                RateDetail { rate: 0.0, amount: 14.0 },
            ]
        );
    }

    #[test]
    fn decodes_spec_example_3_vat_import_tax() {
        let info = SwicoBillInformation::decode_text(
            "//S1/10/4031202511/11/180107/20/61257233.4/30/105493567/32/8:49.82/33/2.5:14.85/40/0:30",
        )
        .unwrap();

        assert_eq!(info.vat_rate, None);
        assert_eq!(info.vat_rate_details, vec![RateDetail { rate: 8.0, amount: 49.82 }]);
        assert_eq!(info.vat_import_taxes, vec![RateDetail { rate: 2.5, amount: 14.85 }]);
    }

    #[test]
    fn decodes_spec_example_4_escaped_slash() {
        let info = SwicoBillInformation::decode_text(
            r"//S1/10/X.66711\/8824/11/200712/20/MW-2020-04/30/107978798/32/2.5:117.22/40/3:5;1.5:20;1:40;0:60",
        )
        .unwrap();

        assert_eq!(info.invoice_number.as_deref(), Some("X.66711/8824"));
        assert_eq!(
            info.payment_conditions,
            vec![
                PaymentCondition { discount: 3.0, days: 5 },
                PaymentCondition { discount: 1.5, days: 20 },
                PaymentCondition { discount: 1.0, days: 40 },
                PaymentCondition { discount: 0.0, days: 60 },
            ]
        );
    }

    #[test]
    fn round_trips_encode_and_decode() {
        let original = SwicoBillInformation {
            invoice_number: Some("X.66711/8824".to_string()),
            invoice_date: Some(date(2020, 7, 12)),
            customer_reference: Some("MW-2020-04".to_string()),
            vat_number: Some("107978798".to_string()),
            vat_rate: Some(2.5),
            payment_conditions: vec![
                PaymentCondition { discount: 3.0, days: 5 },
                PaymentCondition { discount: 0.0, days: 60 },
            ],
            ..Default::default()
        };

        let encoded = original.encode_as_text().unwrap();
        assert_eq!(encoded, r"//S1/10/X.66711\/8824/11/200712/20/MW-2020-04/30/107978798/32/2.5/40/3:5;0:60");

        let decoded = SwicoBillInformation::decode_text(&encoded).unwrap();
        assert_eq!(decoded, original);
    }

    #[test]
    fn empty_bill_information_encodes_to_none() {
        assert_eq!(SwicoBillInformation::default().encode_as_text(), None);
    }

    #[test]
    fn non_swico_text_does_not_decode() {
        assert_eq!(SwicoBillInformation::decode_text("Invoice 2026-01"), None);
    }
}
