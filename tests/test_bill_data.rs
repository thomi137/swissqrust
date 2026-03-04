/*
 * Copyright (c) 2026 Thomas Prosser
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */

use std::fs;
use swiss_qrust::qr_bill::QrBill;

mod common;
use common::*;
use swiss_qrust::BillData;
use swiss_qrust::input::InputBill;

#[test]
fn test_address_data() {
    let address = crdt_address();
    assert_eq!(address.city, "Bern");
    assert_eq!(address.plz, "3000");
}

#[test]
fn test_bill_data() {
    let expected = include_str!("data/expected/Nr. 1 Datenschema englisch.txt");
    let bill = bill_data();
    let qr_bill = QrBill::new(&bill).unwrap();

    assert_eq!(qr_bill.create_qr_text().unwrap(), expected);
}

#[test]
fn test_bill_data_bill_info_as() {
    let expected = include_str!("data/expected/Nr. 5 Datenschema englisch.txt");

    let bill = bill_data_bill_info_as();
    let qr_bill = QrBill::new(&bill).unwrap();

    assert_eq!(qr_bill.create_qr_text().unwrap(), expected);

}

#[test]
fn test_bill_data_scor_ref() {
    let expected = include_str!("data/expected/Nr. 17 Datenschema englisch.txt");

    let bill = bill_data_scor_ref();
    let qr_bill = QrBill::new(&bill).unwrap();

    assert_eq!(qr_bill.create_qr_text().unwrap(), expected);
}

#[test]
fn test_bill_data_non_ref() {
    let expected = include_str!("data/expected/Nr. 33 Datenschema englisch.txt");

    let bill = bill_data_non_ref();
    let qr_bill = QrBill::new(&bill).unwrap();

    assert_eq!(qr_bill.create_qr_text().unwrap(), expected);
}

#[test]
fn test_correct_toml_read() {
    let content = fs::read_to_string("./tests/data/valid_input/normal_slip_valid.toml").unwrap();
    let input: InputBill = toml::from_str(&content).unwrap();

    assert_eq!(input.iban, "CH9300762011623852957");
    assert_eq!(input.currency, "CHF");
    assert_eq!(input.amount, Some("199.95".to_string()));
    assert_eq!(input.reference, Some("210000000003139471430009017".to_string()));
    assert_eq!(input.unstructured_message, Some("Invoice 2026-01".to_string()));
}

#[test]
fn test_input_to_qr_data() {
    let content = fs::read_to_string("./tests/data/valid_input/normal_slip_valid.toml").unwrap();
    let expected = fs::read_to_string("./tests/data/valid_input/normal_slip_valid.txt").unwrap();

    let input: InputBill = toml::from_str(&content).unwrap();
    let bill = BillData::try_from(input).unwrap();
    let qr_bill = QrBill::new(&bill).unwrap();

    assert_eq!(qr_bill.create_qr_text().unwrap(), expected);
}

#[test]
fn test_input_to_qr_data_no_street() {
    let content = fs::read_to_string("./tests/data/valid_input/normal_slip_valid_no_street.toml").unwrap();
    let expected = fs::read_to_string("./tests/data/valid_input/normal_slip_valid_no_street.txt").unwrap();

    let input: InputBill = toml::from_str(&content).unwrap();
    let bill = BillData::try_from(input).unwrap();
    let qr_bill = QrBill::new(&bill).unwrap();

    assert_eq!(qr_bill.create_qr_text().unwrap(), expected);
}