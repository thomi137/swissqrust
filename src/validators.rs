//! Damm Table
const MOD_10: [u8; 10] = [0, 9, 4, 6, 8, 2, 7, 1, 3, 5];

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
pub fn is_valid_iban(iban: &str) -> Result<bool, String>  {

    let iban: String = iban.chars()
        .filter(|c| !c.is_whitespace())
        .collect();

    if iban.len() < 15 || iban.len() > 34 {
        return Err("Invalid IBAN length".into());
    }

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
            _ => return Err(format!("Invalid character in IBAN: {}", ch)),
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
pub fn is_valid_qr_reference(reference: &str) ->  Result<bool, String> {
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
    Ok(carry == 0)
}

