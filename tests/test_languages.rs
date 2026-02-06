/*
 * Copyright (c) 2026 Thomas Prosser 
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */

use swiss_qrust::language::{label, Language};

#[test]
pub fn test_amount_german() {
    let text = label("Amount", Language::De);
    assert_eq!(text, "Betrag");
}

#[test]
pub fn test_amount_french() {
    let text = label("Amount", Language::Fr);
    assert_eq!(text, "Montant");
}

#[test]
pub fn test_amount_italian() {
    let text = label("Amount", Language::It);
    assert_eq!(text, "Importo");
}
