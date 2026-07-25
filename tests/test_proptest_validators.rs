/*
 * Copyright (c) 2026 Thomas Prosser
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */

//! Property-based tests for the checksum validators/generators in
//! `support::validators` and `support::utils`. These functions sit right on
//! the crate's untrusted-input boundary (a caller can hand in any raw
//! string as an IBAN, reference, or account number), so the properties
//! that matter most are: "never panics on arbitrary input" and
//! "generate -> validate round-trips".

use proptest::prelude::*;
use swiss_qrust::{
    generate_iban_with_checksum, generate_iso11649_with_checksum, generate_qrr_with_checksum,
    is_valid_iban, is_valid_iso11649_reference, is_valid_qr_reference,
};

proptest! {
    #[test]
    fn is_valid_iban_never_panics(s in ".*") {
        let _ = is_valid_iban(&s);
    }

    // `.*` alone almost never generates a string starting with "CH"/"LI",
    // so it never reaches is_valid_iban's `iban[4..]` byte-slicing - the
    // one place that assumes ASCII. Force the interesting prefix and pad
    // with arbitrary (likely multi-byte) characters to actually exercise
    // that path.
    #[test]
    fn is_valid_iban_never_panics_with_multibyte_suffix(rest in "\\PC{10,30}") {
        for prefix in ["CH", "LI", "ch", "li"] {
            let candidate = format!("{prefix}{rest}");
            let _ = is_valid_iban(&candidate);
        }
    }

    #[test]
    fn is_valid_qr_reference_never_panics(s in ".*") {
        let _ = is_valid_qr_reference(&s);
    }

    #[test]
    fn is_valid_iso11649_reference_never_panics(s in ".*") {
        let _ = is_valid_iso11649_reference(&s);
    }

    #[test]
    fn generate_iban_with_checksum_never_panics(cty in ".*", raw in ".*") {
        let _ = generate_iban_with_checksum(&cty, &raw);
    }

    #[test]
    fn generate_qrr_with_checksum_never_panics(raw in ".*") {
        let _ = generate_qrr_with_checksum(&raw);
    }

    #[test]
    fn generate_iso11649_with_checksum_never_panics(raw in ".*") {
        let _ = generate_iso11649_with_checksum(&raw);
    }

    // Round-trip: a CH IBAN built from a 17-digit BBAN (matching real Swiss
    // IBAN structure) via the generator must itself validate - this is the
    // actual invariant the rest of the crate leans on, since BillData::new
    // validates whatever IBAN it's handed.
    #[test]
    fn generated_ch_iban_round_trips(digits in "[0-9]{17}") {
        let iban = generate_iban_with_checksum("CH", &digits).unwrap();
        prop_assert!(is_valid_iban(&iban).is_ok(), "generated IBAN {iban} did not validate");
    }

    // All-zero is a spec-reserved invalid QR reference (is_valid_qr_reference
    // rejects it on purpose), so the digits must contain at least one
    // non-zero digit - guaranteed here by requiring the string to end in one.
    #[test]
    fn generated_qr_reference_round_trips(digits in "[0-9]{0,25}[1-9]") {
        let reference = generate_qrr_with_checksum(&digits).unwrap();
        prop_assert!(is_valid_qr_reference(&reference).is_ok(), "generated QR reference {reference} did not validate");
    }

    #[test]
    fn generated_iso11649_reference_round_trips(alnum in "[0-9A-Za-z]{1,21}") {
        let reference = generate_iso11649_with_checksum(&alnum).unwrap();
        prop_assert!(is_valid_iso11649_reference(&reference).is_ok(), "generated SCOR reference {reference} did not validate");
    }
}
