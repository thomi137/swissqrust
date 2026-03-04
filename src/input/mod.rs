/*
 * Copyright (c) 2026 Thomas Prosser
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */
use std::str::FromStr;
use serde::Deserialize;
use crate::address::{AddressError, InputAddress};
use crate::{BillError, Country, Currency};

#[derive(Debug, Deserialize)]
pub struct InputBill {
    pub iban: String,
    pub creditor_address: InputAddress,
    pub debtor_address: Option<InputAddress>,
    pub country: String,
    pub currency: String,
    pub amount: Option<String>,
    pub reference: Option<String>,
    pub unstructured_message: Option<String>,
    pub bill_information: Option<String>,
    pub alternative_schemes: Option<[Option<String>; 2]>,
}

#[derive(Debug, Deserialize)]
pub struct PartyInput {
    pub name: String,
    pub street: String,
    pub house_no: String,
    pub postal_code: String,
    pub city: String,
    pub country: String,
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

