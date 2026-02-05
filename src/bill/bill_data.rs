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

/// This is all in Millimeters and all in DIN A4

/// A4 Width in mm
pub const A4_PORTRAIT_WIDTH: u16 = 210;

/// A4 Height in mm
pub const A4_PORTRAIT_HEIGHT: u16 = 297;

/// Width of a QR Slip is incidentally the same as the whole paper slip 🤣
pub const QR_BILL_SLIP_WIDTH: u16 = 210;

/// Now the height of the slip itself
pub const QR_BILL_HEIGHT: u16 = 105;

/// Then the width with scissors symbol and all... Surprise
pub const QR_BILL_WITH_ALL_WIDTH: u16 = 210;

/// Takes 5 mm more with all the scissors svg.
pub const QR_BILL_WITH_HORI_LINE_HEIGHT: u16 = 110;

/// QR Code Width
pub const QR_CODE_WIDTH: u16 = 46;

/// QR Code Height
pub const QR_CODE_HEIGHT: u16 = 46;

/// QR Code Padding
pub const QR_CODE_PADDING: u16 = 56;

/// QR Width in mm
pub const QR_CODE_OVERALL_WIDTH: u16 = 148;

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
pub enum Country {
    #[default]
    CH,
    LI
}
impl Display for Country {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Country::CH => f.write_str("CH"),
            Country::LI => f.write_str("LI"),
        }
    }
}

struct BillData {
    creditor_address: Address,
    debtor_address: Option<Address>,
    country: Country,
    currency: Currency,
    amount: String,
    reference_type: ReferenceType,
    unstructured_message: Option<String>,
    bill_information: Option<String>,
} impl BillData {
    pub fn new (
        creditor_address: Address,
        debtor_address: Option<Address>,
        country: Country,
        currency: Currency,
        amount: String,
        reference_type: ReferenceType,
        unstructured_message: Option<String>,
        bill_information: Option<String>,
    ) -> Result<Self, BillError> {

        if !AMOUNT_REGEX.is_match(&amount) {
            return Err(BillError::InvalidAmount);
        }

        Ok(BillData{
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