/*
 * Copyright (c) 2026 Thomas Prosser
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */

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

    (30000u16..=39999u16).contains(&value)
}