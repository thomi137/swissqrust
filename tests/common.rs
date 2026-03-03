/*
 * Copyright (c) 2026 Thomas Prosser
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */
use swiss_qrust::{Address, BillData, Currency, QRCountry, ReferenceType};

pub fn load_test_file(path: &str) -> String {
    let full_path = format!("tests/data/{}", path);
    std::fs::read_to_string(full_path)
        .expect("Failed to read test file")
}

pub fn  crdt_address() -> Address {
    return Address::new(
        "Health insurance fit&kicking",
        Some("Am Wasser"),
        Some("1"),
        "3000",
        "Bern",
        "CH"
    ).unwrap()
}

pub fn dbt_address() -> Address {
    Address::new(
        "Sarah Beispiel",
        Some("Mustergasse"),
        Some("1"),
        "3600",
        "Thun",
        "CH").unwrap()
}

pub fn bill_data() -> BillData {
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

pub fn bill_data_bill_info_as() -> BillData {
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

pub fn bill_data_scor_ref() -> BillData {
    let creditor_address = crdt_address();
    let debtor_address = dbt_address();
    let amount = String::from("211.00");
    let iban = "CH5800791123000889012";
    let ref_type = ReferenceType::infer("RF240191230100405JSH0438").unwrap();

    BillData::new(
        iban.to_string(),
        creditor_address,
        Some(debtor_address),
        QRCountry::CH,
        Currency::CHF,
        Some(amount),
        ref_type,
        Some(String::from("Premium calculation July 2020")),
        None,
        [None, None],
    ).unwrap()
}

