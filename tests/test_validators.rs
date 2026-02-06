/*
 * Copyright (c) 2026 Thomas Prosser
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */

use swiss_qrust::validators::is_valid_iban;
use swiss_qrust::bill::*;

#[test]
fn test_valid_iban(){
    const IBAN: &str = "CH93 0076 2011 6238 5295 7";

    let result = is_valid_iban(IBAN);

    assert!(result.is_ok());
}

#[test]
fn test_valid_qr_iban(){
    const IBAN: &str = "CH55 30024 123456789012";

    let result = is_valid_iban(IBAN);

    assert!(result.is_ok());
}

#[test]
fn test_invalid_iban(){
    const IBAN: &str = "CH44 0871 0000 0033 1272 0007";

    let result = is_valid_iban(IBAN);
    assert!(result.is_err(), "Expected '{}' to be invalid, got OK", IBAN);    }

#[test]
fn test_amount_regex(){
    let amount_f64 = 2384.15;
    assert!(AMOUNT_REGEX.is_match(&amount_f64.to_string()));
}

#[test]
fn test_max_amount_regex(){
    let amount_f64 = 999999999999.99;
    assert!(!AMOUNT_REGEX.is_match(&amount_f64.to_string()));
}

#[test]
fn test_invalid_amount_regex(){
    let amount_f64 = 2384.1;
    assert!(!AMOUNT_REGEX.is_match(&amount_f64.to_string()));
}


