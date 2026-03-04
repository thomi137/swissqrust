/*
 * Copyright (c) 2026 Thomas Prosser
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */
use std::fs;
use swiss_qrust::{create_pdf, label, BillData, Language};
use swiss_qrust::input::InputBill;
use swiss_qrust::render_bill::render_bill_to_bytes;

#[test]
fn test_pdf() {
    // Arrange
    let content = fs::read_to_string("./tests/data/valid_input/normal_slip_valid_no_street.toml").unwrap();
    let input: InputBill = toml::from_str(&content).unwrap();
    let bill = BillData::try_from(input).unwrap();

    // Act
    let test_bill_bytes = render_bill_to_bytes(&bill, swiss_qrust::Language::It).unwrap();
    let text = pdf_extract::extract_text_from_mem(&test_bill_bytes).unwrap();
    let text = normalize_pdf_text(&text);

    // Assert
    assert!(text.contains("Robert Schneider AG"));
    assert!(text.contains("CHF 199.95"));

}

#[test]
fn test_language_de() {
    // Arrange
    let content = fs::read_to_string("./tests/data/valid_input/normal_slip_valid_no_street.toml").unwrap();
    let input: InputBill = toml::from_str(&content).unwrap();
    let bill = BillData::try_from(input).unwrap();

    // Act
    let test_bill_bytes = render_bill_to_bytes(&bill, swiss_qrust::Language::De).unwrap();
    let text = pdf_extract::extract_text_from_mem(&test_bill_bytes).unwrap();
    let text = normalize_pdf_text(&text);

    println!("{}", text);

    // Assert
    assert!(text.contains(label!(AccountPayableTo, Language::De)));
    assert!(text.contains(label!(Reference, Language::De)));
    assert!(text.contains(label!(AdditionalInformation, Language::De)));
    assert!(text.contains(label!(Currency, Language::De)));
    assert!(text.contains(label!(Amount, Language::De)));
    assert!(text.contains(label!(AcceptancePoint, Language::De)));
}

#[test]
fn test_language_fr() {
    // Arrange
    let content = fs::read_to_string("./tests/data/valid_input/normal_slip_valid_no_street.toml").unwrap();
    let input: InputBill = toml::from_str(&content).unwrap();
    let bill = BillData::try_from(input).unwrap();

    // Act
    let test_bill_bytes = render_bill_to_bytes(&bill, swiss_qrust::Language::Fr).unwrap();
    let text = pdf_extract::extract_text_from_mem(&test_bill_bytes).unwrap();
    let text = normalize_pdf_text(&text);

    println!("{}", text);

    // Assert
    assert!(text.contains(label!(AccountPayableTo, Language::Fr)));
    assert!(text.contains(label!(Reference, Language::Fr)));
    assert!(text.contains(label!(AdditionalInformation, Language::Fr)));
    assert!(text.contains(label!(Currency, Language::Fr)));
    assert!(text.contains(label!(Amount, Language::Fr)));
    assert!(text.contains(label!(AcceptancePoint, Language::Fr)));
}

#[test]
fn test_language_it() {
    // Arrange
    let content = fs::read_to_string("./tests/data/valid_input/normal_slip_valid_no_street.toml").unwrap();
    let input: InputBill = toml::from_str(&content).unwrap();
    let bill = BillData::try_from(input).unwrap();

    // Act
    let test_bill_bytes = render_bill_to_bytes(&bill, swiss_qrust::Language::It).unwrap();
    let text = pdf_extract::extract_text_from_mem(&test_bill_bytes).unwrap();
    let text = normalize_pdf_text(&text);

    println!("{}", text);

    // Assert
    assert!(text.contains(label!(AccountPayableTo, Language::It)));
    assert!(text.contains(label!(Reference, Language::It)));
    assert!(text.contains(label!(AdditionalInformation, Language::It)));
    assert!(text.contains(label!(Currency, Language::It)));
    assert!(text.contains(label!(Amount, Language::It)));
    assert!(text.contains(label!(AcceptancePoint, Language::It)));
}


// TODO: Stoss au er possibel en romontsch dal 2026

#[test]
fn test_language_en() {
    // Arrange
    let content = fs::read_to_string("./tests/data/valid_input/normal_slip_valid_no_street.toml").unwrap();
    let input: InputBill = toml::from_str(&content).unwrap();
    let bill = BillData::try_from(input).unwrap();

    // Act
    let test_bill_bytes = render_bill_to_bytes(&bill, swiss_qrust::Language::En).unwrap();
    let text = pdf_extract::extract_text_from_mem(&test_bill_bytes).unwrap();
    let text = normalize_pdf_text(&text);

    println!("{}", text);

    // Assert
    assert!(text.contains(label!(AccountPayableTo, Language::En)));
    assert!(text.contains(label!(Reference, Language::En)));
    assert!(text.contains(label!(AdditionalInformation, Language::En)));
    assert!(text.contains(label!(Currency, Language::En)));
    assert!(text.contains(label!(Amount, Language::En)));
    assert!(text.contains(label!(AcceptancePoint, Language::En)));
}


fn normalize_pdf_text(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            '\u{00A0}' => ' ',   // NBSP
            '\u{00AD}' => '-',   // soft hyphen
            _ => c,
        })
        .collect::<String>()
        .replace('\n', " ")
}
