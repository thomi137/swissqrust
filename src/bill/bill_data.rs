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
use crate::{Address, Country};
use crate::address::AddressError;
use crate::bill::reference_type::ReferenceType;
use crate::input::InputBill;
use crate::qr_bill::{encode_text_to_qr_code, QrBill};
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
    AddressError(#[from] AddressError),
    #[error(transparent)]
    SPSCharsetError(#[from] SPSCharsetError),
    #[error("Invalid currency")]
    InvalidCurrency,
    #[error("Amount does not match amount specification")]
    InvalidAmount,
    #[error("QR-IBAN requires a QR reference (QRR)")]
    QrIbanRequiresQrReference
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
    pub currency: Currency,
    pub amount: Option<String>,
    pub reference_type: ReferenceType,
    pub unstructured_message: Option<String>,
    pub bill_information: Option<String>,
    pub qr_code: Option<QrCode>,
    pub alternative_schemes: [Option<String>; 2],
} impl BillData {
    pub fn new (
        iban: String,
        creditor_address: Address,
        debtor_address: Option<Address>,
        currency: Currency,
        amount: Option<String>,
        reference_type: ReferenceType,

        // Belongs to additional Information
        unstructured_message: Option<String>,

        // Belongs to additional Information, but not to QR code
        bill_information: Option<String>,

        // Belongs to Further Information
        alternative_schemes: [Option<String>;2],
    ) -> Result<Self, BillError> {

        if let Some(ref amt) = amount
            && !AMOUNT_REGEX.is_match(amt) {
                return Err(BillError::InvalidAmount);
        }

        is_valid_iban(&iban)?;

        // TODO: This should have happened before here.
        let iban =
            iban
                .chars()
                .filter(|s| !s.is_whitespace())
                .collect();

        let mut bill =  BillData{
            iban,
            creditor_address,
            debtor_address,
            currency,
            amount,
            reference_type,
            unstructured_message,
            bill_information,
            qr_code: None,
            alternative_schemes,
        };


        // TODO Decompose out of this struct.
         bill.qr_code = QrBill::new(&bill)
            .and_then(|b| b.create_qr_text())
            .and_then(|txt| encode_text_to_qr_code(&txt))
            .ok();

        Ok(bill)
    }
}

impl TryFrom<InputBill> for BillData {
    type Error = BillError;

    fn try_from(input: InputBill) -> Result<Self, Self::Error> {

        let currency = input.currency.parse()?;

        let creditor_address = Address::try_from(input.creditor_address)?;
        let debtor_address =
            match input.debtor_address {
                Some(addr) => Some(Address::try_from(addr)?),
                None => None,
            };
        let alternative_schemes = input
            .alternative_schemes
            .unwrap_or([None, None]);

        let reference_type =
            ReferenceType::infer(input.reference.unwrap_or("".to_string()).as_str())?;

        BillData::new(
            input.iban,
            creditor_address,
            debtor_address,
            currency,
            input.amount,
            reference_type,
            input.unstructured_message,
            input.bill_information,
            alternative_schemes,
        )
    }
}
