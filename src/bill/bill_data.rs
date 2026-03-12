/*
 * Copyright (c) 2026 Thomas Prosser 
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fmt;
use once_cell::sync::Lazy;
use regex::Regex;
use thiserror::Error;
use crate::Address;
use crate::address::AddressError;
use crate::bill::reference_type::ReferenceType;
use crate::input::InputBill;
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

        let bill =  BillData{
            iban,
            creditor_address,
            debtor_address,
            currency,
            amount,
            reference_type,
            unstructured_message,
            bill_information,
            alternative_schemes,
        };
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

/// Generates Bill Data for use in a main function which is testing this library:
/// 1. Create a new Address
/// 2. Create a new BillData
///
/// Note: This is not a real bill, it is just a test case. Validating against the
/// SIX Swiss Bank Master will fail as the IBAN is not valid.
///
/// # Returns
/// `Result<BillData>` - Bill Data Result for use as a test case
///
/// # Example
/// ```
/// # use swiss_qrust::*;
/// let bill_data = build_bill::build_bill().unwrap();
/// assert_eq!(bill_data.creditor_address.name, "Health insurance fit&kicking");
/// assert_eq!(bill_data.reference_type, ReferenceType::QrRef("000008207791225857421286694".to_string()));
/// ```
pub fn build_bill()  -> Result<BillData, BillError> {
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
        Some(debtor),
        Currency::CHF,
        None, //Some(String::from("32111.00")),
        ReferenceType::infer("000008207791225857421286694")?,
        Some(String::from("Premium calculation July 2020")),
        None,
        [None, None],
    );

    Ok(bill_data?)
}