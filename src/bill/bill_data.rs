/*
 * Copyright (c) 2026 Thomas Prosser 
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */
use std::fmt::{Display, Formatter};
use std::fmt;
use once_cell::sync::Lazy;
use regex::Regex;
use thiserror::Error;
use crate::Address;
use crate::bill::reference_type::ReferenceType;
use crate::utils::remove_whitespace;
use crate::validators::*;

pub static AMOUNT_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^\d{1,9}\.\d{2}$").unwrap());

#[derive(Debug, Error)]
pub enum BillError{

    #[error(transparent)]
    ReferenceError(#[from] ReferenceError),
    #[error(transparent)]
    IbanError(#[from] IbanError),
    #[error(transparent)]
    SPSCharsetError(#[from] SPSCharsetError),
    #[error("Amount does not match amount specification")]
    InvalidAmount,
    #[error("QR-IBAN requires a QR reference (QRR)")]
    QrIbanRequiresQrReference,
    
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub enum Currency {
    #[default]
    CHF,
    EUR,
}
impl Display for Currency {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Currency::CHF => f.write_str("CHF"),
            Currency::EUR => f.write_str("EUR"),
        }
    }
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub enum QRCountry {
    #[default]
    CH,
    LI
}
impl Display for QRCountry {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            QRCountry::CH => f.write_str("CH"),
            QRCountry::LI => f.write_str("LI"),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BillData {
    pub iban: String,
    pub creditor_address: Address,
    pub debtor_address: Option<Address>,
    pub country: QRCountry,
    pub currency: Currency,
    pub amount: String,
    pub reference_type: ReferenceType,
    pub unstructured_message: Option<String>,
    pub bill_information: Option<String>,
} impl BillData {
    pub fn new (
        iban: String,
        creditor_address: Address,
        debtor_address: Option<Address>,
        country: QRCountry,
        currency: Currency,
        amount: String,
        reference_type: ReferenceType,
        unstructured_message: Option<String>,
        bill_information: Option<String>,
    ) -> Result<Self, BillError> {

        if !AMOUNT_REGEX.is_match(&amount) {
            return Err(BillError::InvalidAmount);
        }

        is_valid_iban(&iban)?;

        Ok(BillData{
            iban,
            creditor_address,
            debtor_address,
            country,
            currency,
            amount,
            reference_type,
            unstructured_message,
            bill_information,
        })
    }
}