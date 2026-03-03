/*
 * Copyright (c) 2026 Thomas Prosser
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */

use swiss_qrust::qr_bill::QrBill;

mod common;
use common::*;

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

