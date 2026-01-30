use std::fmt;

/// Damm Table for testing QR Reference against mod-10
const MOD_10: [u8; 10] = [0, 9, 4, 6, 8, 2, 7, 1, 3, 5];

/// IBAN Errors
#[derive(Debug, PartialEq)]
pub enum IbanError{
    IncorrectLength{expected: usize, actual: usize},
    IncorrectCountryCode,
    InvalidCharacter,
}

/// Reference number errors for both QR-IBAN and SCOR (ISO11649)
#[derive(Debug, PartialEq)]
pub enum ReferenceError {
    InvalidQrChar(char),
    InvalidQrChecksum,
    InvalidIso11649Length,
    InvalidIso11649Prefix,
    InvalidIso11649Char(char),
    InvalidIso11649Checksum,
}
impl std::error::Error for ReferenceError {}

impl fmt::Display for ReferenceError {
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
        ReferenceError::InvalidQrChar(c) => {
            write!(f, "Invalid QR reference character: '{}'", c)
        }
        ReferenceError::InvalidQrChecksum => write!(f, "QR reference checksum failed"),
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
    }
}
}

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
///let result = is_valid_iban(IBAN).unwrap();
///assert!(!result, "Expected '{}' to be invalid, but got true", IBAN);
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
pub fn is_valid_iban(iban: &str) -> Result<bool, IbanError>  {

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
    Ok(remainder == 1)
}

/// QR Reference Number
///
/// Valid QR Reference:
/// ```
/// use swiss_qrust::validators::is_valid_qr_reference;
/// const REF: &str = "21 00000 00003 13947 14300 09017";
/// assert!(is_valid_qr_reference(REF).is_ok());
/// ```
/// TODO: Needs more checks. A Reference "0" passes this
pub fn is_valid_qr_reference(reference: &str) ->  Result<(), String> {
    let mut carry: u8 = 0;

    let reference: String = reference
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect();

    for ch in reference.chars() {
        if !ch.is_ascii_digit() {
            return Err(format!("Invalid character: {}", ch));
        }
        let digit = ch as u8 - b'0';
        carry = MOD_10[((carry + digit) % 10) as usize];
    }
    Ok(())
}

pub fn is_valid_iso11649_reference(reference: &str) ->  Result<(), ReferenceError> {

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

    if !mod97(rearranged) {
        return Err(ReferenceError::InvalidIso11649Checksum);
    }

    Ok(())
}

pub fn mod97<I>(chars: I) -> bool
where
    I: IntoIterator<Item = char>,
{
    let mut remainder: u32 = 0;

    for ch in chars {
        let ch = ch.to_ascii_uppercase();
        let value = match ch {
            '0'..='9' => ch as u32 - '0' as u32,
            'A'..='Z' => ch as u32 - 'A' as u32 + 10,
            _ => return false, // invalid character
        };

        remainder = if value < 10 {
            (remainder * 10 + value) % 97
        } else {
            (remainder * 100 + value) % 97
        };
    }

    remainder == 1
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_qr_ref(){
    assert_eq!(is_valid_qr_reference("0").is_ok(), true);
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
}
