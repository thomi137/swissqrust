/*
 * Copyright (c) 2026 Thomas Prosser
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */

use std::str::FromStr;
use serde::Deserialize;
use crate::address::InputAddress;
use crate::{BillError, Currency};

#[derive(Debug, Deserialize)]
pub struct InputBill {
    pub iban: String,
    pub creditor_address: InputAddress,
    pub debtor_address: Option<InputAddress>,
    pub currency: String,
    pub amount: Option<String>,
    pub reference: Option<String>,
    pub unstructured_message: Option<String>,
    pub bill_information: Option<String>,
    /// Structured alternative to `bill_information`: encoded to Swico S1
    /// text on conversion. Providing both is an error.
    pub swico_bill_information: Option<InputSwicoBillInformation>,
    pub alternative_schemes: Option<[Option<String>; 2]>,
}

/// Dates are plain "YYYY-MM-DD" strings here (parsed in `TryFrom`) since
/// `chrono` is used without its `serde` feature.
#[derive(Debug, Deserialize)]
pub struct InputSwicoBillInformation {
    pub invoice_number: Option<String>,
    pub invoice_date: Option<String>,
    pub customer_reference: Option<String>,
    pub vat_number: Option<String>,
    pub vat_date: Option<String>,
    pub vat_start_date: Option<String>,
    pub vat_end_date: Option<String>,
    pub vat_rate: Option<f64>,
    pub vat_rate_details: Option<Vec<InputRateDetail>>,
    pub vat_import_taxes: Option<Vec<InputRateDetail>>,
    pub payment_conditions: Option<Vec<InputPaymentCondition>>,
}

#[derive(Debug, Deserialize)]
pub struct InputRateDetail {
    pub rate: f64,
    pub amount: f64,
}

#[derive(Debug, Deserialize)]
pub struct InputPaymentCondition {
    pub discount: f64,
    pub days: u32,
}

impl FromStr for Currency {
    type Err = BillError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "CHF" => Ok(Currency::CHF),
            "EUR" => Ok(Currency::EUR),
            _ => Err(BillError::InvalidCurrency),
        }
    }
}
