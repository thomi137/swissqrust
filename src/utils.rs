/*
 * Copyright (c) 2026 Thomas Prosser
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */

/// Damm Table for testing QR Reference against mod-10
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
/// use swiss_qrust::utils::remove_whitespace;
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
/// use swiss_qrust::validators::is_valid_iban;
/// use swiss_qrust::utils::is_qr_iban;
/// let qr_iban = "CH4331999000001265789";
/// assert!(is_valid_iban(qr_iban).is_ok());
/// assert_eq!(is_qr_iban(qr_iban), true);
/// ```
///
/// ```
/// use swiss_qrust::validators::is_valid_iban;
/// use swiss_qrust::utils::is_qr_iban;
/// let not_qr_iban = "CH9300762011623852957";
/// assert!(is_valid_iban(not_qr_iban).is_ok());
/// assert_eq!(is_qr_iban(not_qr_iban), false);
/// ```
pub fn is_qr_iban(s: &str) -> bool {

    let mut iter = s
        .chars()
        .skip(4)
        .take(5);

    let mut value: u16 = 0;

    for c in &mut iter {
        let digit = match c.to_digit(10) {
            Some(d) => d as u16,
            None => return false,
        };

        value = value * 10 + digit;
    }

    // let slice: u16 = s[4..9].parse().unwrap();

    (30000u16..=31999u16).contains(&value)
}

/// Used for checking QR Reference for digits only. Fails at first non-digit
/// character
///
/// ```
/// use swiss_qrust::utils::qr_ref_is_numeric;
/// assert_eq!(qr_ref_is_numeric("210000000003139471430009017"), true)
/// ```
/// Note that the string needs to be clear of whitespace.
/// `is_numeric(s: &str)` has to fail in that case because whitespace
/// is not numeric:
///
/// ```
/// use swiss_qrust::utils::qr_ref_is_numeric;
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
/// use swiss_qrust::utils::is_in_extended_sps_charset;
/// assert_eq!(is_in_extended_sps_charset('ț' as u32), true)
/// ```
/// ```
/// use swiss_qrust::utils::is_in_extended_sps_charset;
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

    ((10 - carry) % 10) == 0
}

// Add to existing utils.rs

pub trait SliceExt<T> {
    fn all_but_last(&self) -> &[T];
    fn all_but_first(&self) -> &[T];
}

impl<T> SliceExt<T> for [T] {
    fn all_but_last(&self) -> &[T] {
        self.get(..self.len().saturating_sub(1)).unwrap_or(&[])
    }

    fn all_but_first(&self) -> &[T] {
        self.get(1..).unwrap_or(&[])
    }
}
