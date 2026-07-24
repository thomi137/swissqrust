/*
 * Copyright (c) 2026 Thomas Prosser
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */

//! `bill::swico` has good coverage of the S1 encode/decode logic itself,
//! but nothing previously exercised the actual wiring a real caller (the
//! web frontend included) goes through: `InputBill.swico_bill_information`
//! -> `TryFrom<InputBill>` -> `BillData.bill_information`. These use raw
//! JSON (not `InputBill` struct literals) so a field-name mismatch between
//! `web/app.js`'s payload and `InputSwicoBillInformation`'s Deserialize
//! impl would fail here the same way it would in the browser.
//!
//! Swico spec (S1 syntax, Generelle Regeln p.4): "Ein Tag ohne Daten darf
//! weggelassen werden" - every one of /10/ /11/ /20/ /30/ /31/ /32/ /33/
//! /40/ is independently optional. In particular, a business without VAT
//! must still be able to produce Swico billing info without a /32/ tag.

use std::convert::TryFrom;
use swiss_qrust::{BillData, BillError, InputBill};

fn bill_json(swico_bill_information: &str) -> String {
    format!(
        r#"{{
            "iban": "CH93 0076 2011 6238 5295 7",
            "currency": "CHF",
            "amount": "100.00",
            "reference": null,
            "unstructured_message": null,
            "creditor_address": {{
                "name": "Robert Schneider AG",
                "street": "Rue du Lac",
                "house_num": "1268",
                "plz": "2501",
                "city": "Biel",
                "country": "CH"
            }},
            "debtor_address": null,
            "swico_bill_information": {swico_bill_information},
            "alternative_schemes": null
        }}"#
    )
}

#[test]
fn swico_without_a_vat_rate_still_produces_output() {
    // A business without VAT: invoice number, customer reference and a
    // discount, but no /32/ tag at all.
    let json = bill_json(
        r#"{
            "invoice_number": "10201409",
            "invoice_date": null,
            "customer_reference": "1400.000-53",
            "vat_number": null,
            "vat_date": null,
            "vat_start_date": null,
            "vat_end_date": null,
            "vat_rate": null,
            "vat_rate_details": null,
            "vat_import_taxes": null,
            "payment_conditions": [{"discount": 2.0, "days": 10}]
        }"#,
    );

    let input: InputBill = serde_json::from_str(&json).unwrap();
    let bill = BillData::try_from(input).unwrap();

    assert_eq!(
        bill.bill_information.as_deref(),
        Some("//S1/10/10201409/20/1400.000-53/40/2:10"),
        "no VAT rate should just omit /32/, not drop the whole payload"
    );
}

#[test]
fn swico_with_only_payment_conditions_still_produces_output() {
    // The minimal case: nothing but payment terms - no invoice number, no
    // customer reference, no VAT anything.
    let json = bill_json(
        r#"{
            "invoice_number": null,
            "invoice_date": null,
            "customer_reference": null,
            "vat_number": null,
            "vat_date": null,
            "vat_start_date": null,
            "vat_end_date": null,
            "vat_rate": null,
            "vat_rate_details": null,
            "vat_import_taxes": null,
            "payment_conditions": [{"discount": 0.0, "days": 30}]
        }"#,
    );

    let input: InputBill = serde_json::from_str(&json).unwrap();
    let bill = BillData::try_from(input).unwrap();

    assert_eq!(bill.bill_information.as_deref(), Some("//S1/40/0:30"));
}

#[test]
fn swico_toggled_on_with_every_field_empty_produces_no_billing_info() {
    // Matches web/app.js: toggling Swico "on" always sends a
    // swico_bill_information *object*, even before the user has typed
    // anything into it - every field inside is null.
    let json = bill_json(
        r#"{
            "invoice_number": null,
            "invoice_date": null,
            "customer_reference": null,
            "vat_number": null,
            "vat_date": null,
            "vat_start_date": null,
            "vat_end_date": null,
            "vat_rate": null,
            "vat_rate_details": null,
            "vat_import_taxes": null,
            "payment_conditions": null
        }"#,
    );

    let input: InputBill = serde_json::from_str(&json).unwrap();
    let bill = BillData::try_from(input).unwrap();

    assert_eq!(bill.bill_information, None);
}

#[test]
fn providing_both_bill_information_and_swico_is_rejected() {
    let json = format!(
        r#"{{
            "iban": "CH93 0076 2011 6238 5295 7",
            "currency": "CHF",
            "amount": "100.00",
            "reference": null,
            "unstructured_message": null,
            "creditor_address": {{
                "name": "Robert Schneider AG",
                "street": "Rue du Lac",
                "house_num": "1268",
                "plz": "2501",
                "city": "Biel",
                "country": "CH"
            }},
            "debtor_address": null,
            "bill_information": "some free-text billing info",
            "swico_bill_information": {{
                "invoice_number": "10201409",
                "invoice_date": null,
                "customer_reference": null,
                "vat_number": null,
                "vat_date": null,
                "vat_start_date": null,
                "vat_end_date": null,
                "vat_rate": null,
                "vat_rate_details": null,
                "vat_import_taxes": null,
                "payment_conditions": null
            }},
            "alternative_schemes": null
        }}"#
    );

    let input: InputBill = serde_json::from_str(&json).unwrap();
    let err = BillData::try_from(input).unwrap_err();

    assert!(matches!(err, BillError::AmbiguousBillingInformation));
}
