/*
 * Copyright (c) 2026. Thomas Prosser
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */

use std::fmt;
use std::fmt::{Display, Formatter};
use thiserror::Error;
use crate::{utils, Country};
use crate::utils::{is_in_extended_sps_charset,
                   qr_ref_is_numeric,
                   remove_whitespace
};


/// Charset Error
#[derive(Debug, PartialEq)]
pub struct SPSCharsetError {
        invalid: char,
        index: usize,
}

impl Display for SPSCharsetError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
        SPSCharsetError { invalid, index } => {
                write!(f, "Found invalid character {} at {}", invalid, index)
            }
        }
    }
}

impl std::error::Error for SPSCharsetError {}

/// IBAN Errors
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IbanError{
    IncorrectLength{expected: usize, actual: usize},
    IncorrectCountryCode,
    InvalidCharacter,
    InvalidIban,
}
impl Display for IbanError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            IbanError::IncorrectLength { expected, actual } => { 
                write!(f, "Incorrect length expected {} got {}", expected, actual)
            },
            IbanError::IncorrectCountryCode => {
                write!(f, "Incorrect country code. Must be CH or LI")
            },
            IbanError::InvalidCharacter => {
                f.write_str("Invalid character")
            },
            IbanError::InvalidIban => {
                f.write_str("Invalid Iban")
            }
        }
    }
}

impl std::error::Error for IbanError {}

#[derive(Debug, Error)]
pub enum CountryValidationError {

    #[error("Invalid country code")]
    InvalidCountryCode(String),

    #[error("country code is withdrawn")]
    Withdrawn,

    #[error("country code is reserved")]
    Reserved,
}


/// Reference number errors for both QR-IBAN and SCOR (ISO11649)
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ReferenceError {
    InvalidQrChar,
    InvalidQrChecksum,
    InvalidQrLength { expected: usize, actual: usize },
    InvalidIso11649Length,
    InvalidIso11649Prefix,
    InvalidIso11649Char(char),
    InvalidIso11649Checksum,
    InvalidReference,
}

impl fmt::Display for ReferenceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReferenceError::InvalidQrChar => {
                write!(f, "Invalid QR reference character")
            }
            ReferenceError::InvalidQrChecksum => write!(f, "QR reference checksum failed"),
            ReferenceError::InvalidQrLength { expected: e, actual: a } => write!(f, "Invalid QR length, must be {}, is {}", e, a),
            ReferenceError::InvalidIso11649Length => {
                write!(f, "ISO 11649 reference must be 5–25 characters long")
            }
            ReferenceError::InvalidIso11649Prefix => {
                write!(f, "ISO 11649 reference must start with 'RF'")
            }
            ReferenceError::InvalidIso11649Char(c) => {
                write!(f, "ISO11649 reference invalid character: '{}'", c)
            }
            ReferenceError::InvalidIso11649Checksum => {
                write!(f, "ISO11649 reference checksum failed")
            }
            ReferenceError::InvalidReference => {
                write!(f, "Invalid reference")
            }
        }
    }
}

impl std::error::Error for ReferenceError {}

/// Validates an IBAN
///
/// Valid IBAN:
///```
/// use swiss_qrust::validators::is_valid_iban;
/// const  IBAN: &str = "CH93 0076 2011 6238 5295 7";
/// assert!(is_valid_iban(IBAN).is_ok());
///```
///
/// Invalid IBAN:
///```
///use swiss_qrust::validators::is_valid_iban;
///
///const IBAN: &str = "CH44 0871 0000 0033 1272 0007";
///let result = is_valid_iban(IBAN);
///assert!(result.is_err(), "Expected '{}' to be invalid, but got true", IBAN);
///```
///
/// Another invalid IBAN:
///```
/// pub const CNT_ERR: &str = "IBAN must pertain to Switzerland or Liechtenstein";
/// use swiss_qrust::validators::{is_valid_iban, IbanError};
///
/// const IBAN: &str = "GB33BUKB20201555555555";
/// let err = is_valid_iban(IBAN).unwrap_err();
/// assert_eq!(err, IbanError::IncorrectCountryCode );
/// ```
pub fn is_valid_iban(iban: &str) -> Result<(), IbanError>  {

    let iban: String = iban.chars()
        .filter(|c| !c.is_whitespace())
        .collect();

    let first_two: String = iban
        .to_ascii_uppercase()
        .chars()
        .take(2)
        .collect();

    if !(first_two == "CH" || first_two == "LI") {
        return Err(IbanError::IncorrectCountryCode);
    }

    if iban.len() < 15 || iban.len() > 34 {
        return Err(IbanError::IncorrectLength{
            expected: 21,
            actual: iban.len(),
        })}

    let rearranged = iban[4..]
        .chars()
        .chain(iban[..4].chars());

    let mut remainder: u32 = 0;

    for ch in rearranged {
        match ch.to_ascii_uppercase() {
            '0'..='9' => {
                remainder = (remainder * 10 + (ch as u32 - '0' as u32)) % 97;
            }
            'A'..='Z' => {
                let value = ch.to_ascii_uppercase() as u32 - 'A' as u32 + 10;
                remainder = (remainder * 100 + value) % 97;
            }
            _ => return Err(IbanError::InvalidCharacter),
        }
    }

    if remainder != 1 {
        return Err(IbanError::InvalidIban)
    }

    Ok(())
}

/// QR Reference Number
///
/// Valid QR Reference:
/// ```
/// use swiss_qrust::validators::is_valid_qr_reference;
/// const REF: &str = "21 00000 00003 13947 14300 0901 7";
/// assert!(is_valid_qr_reference(REF).is_ok());
/// ```
pub fn is_valid_qr_reference(reference: &str) -> Result<(), ReferenceError> {
    let mut reference = reference.to_owned();
    remove_whitespace(&mut reference);

    if !qr_ref_is_numeric(&reference) {
        return Err(ReferenceError::InvalidQrChar);
    }

    if reference.len() != 27 {
        return Err(ReferenceError::InvalidQrLength {
            expected: 27,
            actual: reference.len(),
        });
    }

    if reference == "000000000000000000000000000" {
        return Err(ReferenceError::InvalidQrChecksum);
    }

    if !utils::mod10(&reference) {
        return Err(ReferenceError::InvalidQrChecksum);
    }

    Ok(())
}

pub fn is_valid_sps_charset(s: &str) -> Result<(), SPSCharsetError> {

    for (i, ch) in s.chars().enumerate() {
        if !is_in_extended_sps_charset(ch as u32) {
            return Err(SPSCharsetError {
                invalid: ch,
                index: i,
            })
        }
    }

    Ok(())
}

pub fn is_valid_iso11649_reference(reference: &str) ->  Result<(), ReferenceError> {

    let mut reference = String::from(reference);
    remove_whitespace(&mut reference);

    if reference.len() < 5 || reference.len() > 25 {
        return Err(ReferenceError::InvalidIso11649Length);
    }

    if !reference.starts_with("RF") {
        return Err(ReferenceError::InvalidIso11649Prefix);
    }

    for c in reference.chars() {
        if !c.is_ascii_alphanumeric() {
            return Err(ReferenceError::InvalidIso11649Char(c));
        }
    }

    let rearranged = reference[4..]
        .chars()
        .chain(reference[..4].chars());

    if !utils::mod97(rearranged) {
        return Err(ReferenceError::InvalidIso11649Checksum);
    }

    Ok(())
}

pub fn is_valid_iso_3661_1_country(country: &str) -> Result<Country, CountryValidationError> {

    let country: Country = country
        .to_ascii_uppercase()
        .parse()
        .map_err(|_| CountryValidationError::InvalidCountryCode(country.to_string()))?;

    Ok(country)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_qr(){
        let qr = "21 00000 00003 13947 14300 0901 7";
        assert!(is_valid_qr_reference(qr).is_ok());
    }

    #[test]
    fn test_invalid_length_qr_ref(){
        assert_eq!(is_valid_qr_reference("0").unwrap_err(), ReferenceError::InvalidQrLength{expected: 27, actual: 1});
    }

    #[test]
    fn valid_iso11649_references() {
        let valid = [
            "RF18539007547034",
            "RF49N73GBST73AKL38ZX",
            "RF08B3700321",
            "RF19N8BG33KQ9HSS7BG",
        ];

        for r in valid {
            assert!(
                is_valid_iso11649_reference(r).is_ok(),
                "Expected '{}' to be valid",
                r
            );
        }
    }
    #[test]
    fn invalid_iso11649_prefix() {
        let err = is_valid_iso11649_reference("AB18539007547034")
            .unwrap_err();

        assert_eq!(err, ReferenceError::InvalidIso11649Prefix);
    }

    #[test]
    fn valid_iso11649_with_spaces() {
        let valid = "RF08 B370 0321";

        assert!(
            is_valid_iso11649_reference(valid).is_ok(),
            "Expected '{}' to be valid", valid
        );
    }

    #[test]
    fn valid_iso11649_with_lowercase() {
        let valid = "RF 44 all lower case";

        assert!(
            is_valid_iso11649_reference(valid).is_ok(),
            "Expected '{}' to be valid", valid
        );
    }

    #[test]
    fn invalid_iso11649_too_short() {
        let err = is_valid_iso11649_reference("RF04");

        assert_eq!(err, Err(ReferenceError::InvalidIso11649Length));
    }

    #[test]
    fn invalid_iso11649_too_long() {
        let err = is_valid_iso11649_reference("RF04GHJ74CV9B4DFH99RXPLMMQ43JKL0");

        assert_eq!(err.unwrap_err(), ReferenceError::InvalidIso11649Length);
    }

    #[test]
    fn valid_sps_string() {
        assert!(is_valid_sps_charset("Șipi Müller € AG").is_ok())
    }

    #[test]
    fn invalid_sps_string() {
        let err = is_valid_sps_charset("Hello 🤣").unwrap_err();
        assert_eq!(err.invalid, '🤣');
        assert_eq!(err.index, 6);
    }

    #[test]
    fn is_valid_iso_country() {
        let country = "AF";
        assert!(is_valid_iso_3661_1_country(country).is_ok());
    }

    #[test]
    fn is_valid_iso_country_lowercase() {
        let country = "fr";
        assert!(is_valid_iso_3661_1_country(country).is_ok());
    }

    #[test]
    fn is_invalid_iso_country() {
        let country = "SU";
        assert!(is_valid_iso_3661_1_country(country).is_err());
    }
}
