/*
 * Copyright (c) 2026 Thomas Prosser 
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */
use std::num::ParseIntError;

/// Cyclic Mod-10 Vector for validating and calculating ESR Checksum
const MOD_10: [u8; 10] = [0, 9, 4, 6, 8, 2, 7, 1, 3, 5];

/// Helpers for String manipultion or checking.
/// Removes whitespace in-place
/// taken from
/// [Stackoverflow](https://stackoverflow.com/questions/57063777/remove-all-whitespace-from-a-string)
///
/// Using this because it is a little faster than a new alloc.
/// For validation purposes, that should suffice. Since I use it with a ref,
/// the performance should not increase that much, though.
///
/// ```
/// use swiss_qrust::support::utils::remove_whitespace;
/// let mut s = String::from("This has whitespace");
/// remove_whitespace(&mut s);
/// assert_eq!(s, "Thishaswhitespace");
/// ```
pub fn remove_whitespace(s: &mut String) {
    s.retain(|c| !c.is_whitespace());
}

/// Verifies if an IBAN is a QR Iban specific to
/// Swiss interbank clearing. Assumes whitespace removed.
/// You should check if the IBAN is valid first and remove whitespace.
/// This only checks the specifics for QR IBANs in order
/// to make sure there is also a QR Reference on the bill
///
/// ```
/// use swiss_qrust::support::validators::is_valid_iban;
/// use swiss_qrust::support::utils::is_qr_iban;
/// let qr_iban = "CH4331999000001265789";
/// assert!(is_valid_iban(qr_iban).is_ok());
/// assert_eq!(is_qr_iban(qr_iban), true);
/// ```
///
/// ```
/// use swiss_qrust::support::validators::is_valid_iban;
/// use swiss_qrust::support::utils::is_qr_iban;
/// let not_qr_iban = "CH9300762011623852957";
/// assert!(is_valid_iban(not_qr_iban).is_ok());
/// assert_eq!(is_qr_iban(not_qr_iban), false);
/// ```
pub fn is_qr_iban(s: &str) -> bool {

    let mut iter = s
        .chars()
        .skip(4)
        .take(5);

    let mut value: u32 = 0;

    for c in &mut iter {
        let digit = match c.to_digit(10) {
            Some(d) => d,
            None => return false,
        };

        value = value * 10 + digit;
    }

    // let slice: u16 = s[4..9].parse().unwrap();

    (30000u32..=31999u32).contains(&value)
}

/// Used for checking QR Reference for digits only. Fails at first non-digit
/// character
///
/// ```
/// use swiss_qrust::support::utils::qr_ref_is_numeric;
/// assert_eq!(qr_ref_is_numeric("210000000003139471430009017"), true)
/// ```
/// Note that the string needs to be clear of whitespace.
/// `is_numeric(s: &str)` has to fail in that case because whitespace
/// is not numeric:
///
/// ```
/// use swiss_qrust::support::utils::qr_ref_is_numeric;
/// assert_eq!(qr_ref_is_numeric("21 00000 00003 13947 14300 0901 7"), false)
/// ```
pub fn qr_ref_is_numeric(s: &str) -> bool {

    let mut iter = s.chars();
    for c in &mut iter {
        if !c.is_ascii_digit() { return false; }
    }
    true
}

/// According to spec.
/// Text has to be a subset of UTF-8
///
///  * Basic-Latin (Unicodepoint U+0020 – U+007E)
///  * Latin1-Supplement (Unicodepoint U+00A0 – U+00FF)
///  * Latin Extended-A (Unicodepoint U+0100 – U+017F)
///  * Ș – (LATIN CAPITAL LETTER S WITH COMMA BELOW, Unicodepoint U+0218)
///  * ș – (LATIN SMALL LETTER S WITH COMMA BELOW, Unicodepoint U+0219)
///  * Ț – (LATIN CAPITAL LETTER T WITH COMMA BELOW, Unicodepoint U+021A)
///  * ț – (LATIN SMALL LETTER T WITH COMMA BELOW, Unicodepoint U+021B)
///  * € – (EURO SIGN, Unicodepoint U+20AC)
///
/// ```
/// use swiss_qrust::support::utils::is_in_extended_sps_charset;
/// assert_eq!(is_in_extended_sps_charset('ț' as u32), true)
/// ```
/// ```
/// use swiss_qrust::support::utils::is_in_extended_sps_charset;
/// assert_eq!(is_in_extended_sps_charset('🍷' as u32), false)
/// ```
pub fn is_in_extended_sps_charset(ch: u32) -> bool {

    // Basic Latin
    (0x0020..=0x007E).contains(&ch)
        // Latin1 Supplement
        ||(0x00A0..=0x00FF).contains(&ch)
        // Latin Extended-A
        || (0x0100..=0x017F).contains(&ch)
        // Additional characters (Ș ș Ț ț)
        || (0x0218..=0x021B).contains(&ch)
        // Eurp sign
        || ch == 0x20AC

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

pub fn mod10(reference: &str) -> bool {
    let mut carry: u8 = 0;

    for ch in reference.bytes() {

        let digit = ch - b'0';
        carry = MOD_10[((carry + digit) % 10) as usize];
    }

    // Fancy Clippy-Rusty way of saying:
    // ((10 - carry) % 10) == 0
    // This is better, because it will not panic.
    (10 - carry).is_multiple_of(10)
}

/// Generates QR IBAN with Checksum
///
/// # Arguments
///
/// * country_prefix - An ISO 3166-1 alpha-2 Country code.
/// * raw - IBAN string from pos 5..21 (account number with QR-IID)
///
/// # Returns
///
/// * New IBAN with Checksum. NOTE: The function is used to calculate a valid Swiss/LI IBAN. As such, its use
/// is not limited to QR IBANS Only.
///
/// # Examples
///
/// ```
/// # use swiss_qrust::generate_iban_with_checksum;
/// let iban = generate_iban_with_checksum("BE", "435411161155").unwrap();
/// assert_eq!(iban, "BE31435411161155");
/// ```
///
/// ```
/// # use swiss_qrust::generate_iban_with_checksum;;
/// let iban = generate_iban_with_checksum("GR", "0110 1050 0000 1054 7023 795").unwrap();
/// assert_eq!(iban, "GR1601101050000010547023795");
/// ```
pub fn generate_iban_with_checksum(cty: &str, raw: &str) -> Result<String, ParseIntError> {

    let raw = raw.chars().filter(|ch| !ch.is_whitespace()).collect::<String>();

    let mut suffix = String::new();
    for c in cty.chars(){
        if let Some(digit) = c.to_digit(36) {
            suffix.push_str(&digit.to_string());
        }
    }
    suffix.push_str("00");
    let calc_string = format!("{}{}", raw, suffix);

    let mut remainder = 0;
    for c in calc_string.chars() {
        // `raw` is caller-controlled and only whitespace-stripped above, so
        // it can contain non-digit characters - route that through the
        // `Result` this function already promises instead of panicking.
        let digit = c.to_string().parse::<u32>()?;
        remainder = (remainder * 10 + digit) % 97;
    }

    let check_digit = 98 - remainder;

    Ok(format!("{}{:02}{}", cty, check_digit, raw))

}

/// Generates QR Reference with Checksum
///
/// # Arguments
///
/// * raw - numeric string of up to 26 chars length
///
/// # Returns
///
/// * QRR (ESR) String of 27 chars, last being the checksum
///
/// # Example
/// ```
/// # use swiss_qrust::utils::generate_qrr_with_checksum;
/// let with_checksum = generate_qrr_with_checksum("21000000000313947143000901");
/// assert_eq!(with_checksum.as_ref().unwrap().len(), 27);
/// assert_eq!(with_checksum.unwrap(), "210000000003139471430009017");
/// ```
///
/// ```
/// # use swiss_qrust::generate_qrr_with_checksum;
/// let with_checksum = generate_qrr_with_checksum("18 7858");
/// assert_eq!(with_checksum.as_ref().unwrap().len(), 27);
/// assert_eq!(with_checksum.unwrap(), "000000000000000000001878583");
/// ```
pub fn generate_qrr_with_checksum(raw: &str) -> Result<String, ParseIntError> {

    let raw = raw.chars().filter(|ch| !ch.is_whitespace()).collect::<String>();

    let mut carry = 0u8;

    for char in raw.chars() {
        if let Some(digit) = char.to_digit(10) {
            carry = MOD_10[(carry as usize + digit as usize) % 10];
        }
    }

    let checksum = (10 - carry) % 10;

    Ok(format!("{:0>26}{}", raw, checksum))
}

/// Generate SCOR (ISO 11649) Reference with Checksum
///
/// # Arguments
///
/// * raw - alphanumeric string of up to 21 chars length
///
/// # Returns
///
/// * SCOR Reference number of up to 25 chars, first 4 being 'RF' + XX, where XX is a two-digit checksum.
///
/// # Examples
///
/// ```
/// # use swiss_qrust::generate_iso11649_with_checksum;
/// let input = "5390075470Y";
/// let with_checksum = generate_iso11649_with_checksum(input);
/// assert_eq!(with_checksum.unwrap(), "RF185390075470Y");
/// ```
///
/// ```
/// # use swiss_qrust::generate_iso11649_with_checksum;
/// let input = "1234 5";
/// let with_checksum = generate_iso11649_with_checksum(input);
/// assert_eq!(with_checksum.unwrap(), "RF7812345");
/// ```
pub fn generate_iso11649_with_checksum(raw: &str) -> Result<String, ParseIntError> {

    let raw = raw.chars().filter(|ch| !ch.is_whitespace()).collect::<String>();

    let mut numeric_str = String::new();
    for c in raw.chars().filter(|ch| ch.is_alphanumeric()) {
        if let Some(d) = c.to_digit(36) {
            numeric_str.push_str(&d.to_string());
        }
    }

    numeric_str.push_str("271500"); // "RF" converted + "00"

    // Calculate mod 97 using Horner's method for big numbers.
    let mut remainder = 0;
    for c in numeric_str.chars() {
        let digit = c.to_digit(10).unwrap();
        remainder = (remainder * 10 + digit) % 97;
    }

    let check_digit = 98 - remainder;

    Ok(format!("RF{:02}{}", check_digit, raw))
}
