/*
 * Copyright (c) 2026 Thomas Prosser
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */

use swiss_qrust::bill::{Address, BillData};
use swiss_qrust::{BillError, Currency, QRCountry, ReferenceType};
use swiss_qrust::qr_bill::QrBill;

#[test]
fn test_address_data() {
    let address = crdt_address();
    assert_eq!(address.city, "Bern");
    assert_eq!(address.plz, "3000");
}

#[test]
fn test_bill_data() {
    let expected = include_str!("fixtures/Nr. 1 Datenschema englisch.txt");
    let bill = bill_data();
    let qr_bill = QrBill::new(&bill).unwrap();

    assert_eq!(qr_bill.create_qr_text().unwrap(), expected);
}


fn  crdt_address() -> Address {
    return Address::new(
        "Health insurance fit&kicking",
        Some("Am Wasser"),
        Some("1"),
        "3000",
        "Bern",
        "CH"
    ).unwrap()
}

fn dbt_address() -> Address {
    Address::new(
        "Sarah Beispiel",
        Some("Mustergasse"),
        Some("1"),
        "3600",
        "Thun",
        "CH").unwrap()
}

fn bill_data() -> BillData {
    let creditor_address = crdt_address();
    let debtor_address = dbt_address();
    let amount = String::from("111.00");
    let iban = "CH64 3196 1000 0044 2155 7";
    BillData::new(
        iban.to_string(),
        creditor_address,
        Some(debtor_address),
        QRCountry::CH,
        Currency::CHF,
        Some(amount),
        ReferenceType::QrRef(String::from("000008207791225857421286694")),
        Some(String::from("Premium calculation July 2020")),
        None
    ).unwrap()
}


fn bill_data_no_debtor_no_amount() -> BillData {
    let creditor_address = crdt_address();
    let debtor_address = dbt_address();
    let amount = String::from("111.00");
    let iban = "CH6631996000002544373";
    return BillData::new(
        iban.to_string(),
        creditor_address,
        None,
        QRCountry::CH,
        Currency::CHF,
        Some(amount),
        ReferenceType::QrRef(String::from("000008207791225857421286694")),
        Some(String::from("Premium calculation July 2020")),
        None
    ).unwrap();
}

