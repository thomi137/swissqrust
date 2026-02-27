/*
 * Copyright (c) 2026 Thomas Prosser 
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */
use std::fmt::{Display, Formatter};
use std::fmt;
use once_cell::sync::Lazy;
use qrcodegen::QrCode;
use regex::Regex;
use thiserror::Error;
use crate::Address;
use crate::bill::reference_type::ReferenceType;
use crate::support::validators::*;

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
#[derive(Clone, Eq, PartialEq)]
pub struct BillData {
    pub iban: String,
    pub creditor_address: Address,
    pub debtor_address: Option<Address>,
    pub country: QRCountry,
    pub currency: Currency,
    pub amount: Option<String>,
    pub reference_type: ReferenceType,
    pub unstructured_message: Option<String>,
    pub additional_information: Option<String>,
    pub qr_code: Option<QrCode>,
} impl BillData {
    pub fn new (
        iban: String,
        creditor_address: Address,
        debtor_address: Option<Address>,
        country: QRCountry,
        currency: Currency,
        amount: Option<String>,
        reference_type: ReferenceType,
        unstructured_message: Option<String>,
        additional_information: Option<String>,
    ) -> Result<Self, BillError> {

        if let Some(ref amt) = amount {
            if !AMOUNT_REGEX.is_match(amt) {
                return Err(BillError::InvalidAmount);
            }
        }

        is_valid_iban(&iban)?;

        // TODO: This should have happened before here.
        let iban =
            iban
                .chars()
                .filter(|s| !s.is_whitespace())
                .collect();

        Ok(BillData{
            iban,
            creditor_address,
            debtor_address,
            country,
            currency,
            amount,
            reference_type,
            unstructured_message,
            additional_information,
            qr_code: None,
        })
    }
}
