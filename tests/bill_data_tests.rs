/*
 * Copyright (c) 2026 Thomas Prosser
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */

use swiss_qrust::bill::{Address, BillData};
use swiss_qrust::qr_bill::QrBill;
use swiss_qrust::{Currency, QRCountry, ReferenceType};

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
        None,
        [None, None]
    ).unwrap()
}

fn bill_data_bill_info_as() -> BillData {
    let creditor_address = crdt_address();
    let debtor_address = dbt_address();
    let amount = String::from("121.00");
    let iban = "CH2231989000007611146";
    return BillData::new(
        iban.to_string(),
        creditor_address,
        Some(debtor_address),
        QRCountry::CH,
        Currency::CHF,
        Some(amount),
        ReferenceType::QrRef(String::from("000003701588132583136809972")),
        Some(String::from("Premium calculation July 2020")),
        Some(String::from("//S1/10/10201409/11/200630/20/140.000-53/30/102673831/31/200630/32/7.7/33/7.7:9.30/40/0:30")),
        [Some(String::from("eBill/B/sarah.beispiel@einfach-zahlen.ch")), None],
    ).unwrap();
}

