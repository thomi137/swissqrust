/*
 * Copyright (c) 2026 Thomas Prosser
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */

use anyhow::Result;
use crate::{Address, BillData, Currency, QRCountry, ReferenceType};

pub fn buid_bill()  -> Result<BillData> {
    let  creditor = Address::new(
        "Health insurance fit&kicking",
        Some("Am Wasser"),
        Some("1"),
        "3000",
        "Bern",
        "CH"
    )?;

    let debtor = Address::new(
        "Sarah Beispiel",
        Some("Mustergasse"),
        Some("1"),
        "3600",
        "Thun",
        "CH"
    )?;

    let bill_data = BillData::new(
        "CH64 3196 1000 0044 2155 7".to_string(),
        creditor,
        None,//Some(debtor),
        QRCountry::CH,
        Currency::CHF,
        Some(String::from("32111.00")),
        ReferenceType::infer("000008207791225857421286694")?,
        Some(String::from("Premium calculation July 2020")),
        None
    );

    Ok(bill_data?)
}