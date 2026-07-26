/*
 * Copyright (c) 2026 Thomas Prosser
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */

//! Generate Swiss QR-bill payment slips (ISO 20022 / SIX Implementation
//! Guidelines) as PDF or SVG.
//!
//! The typical flow is: parse an [`InputBill`] from TOML or JSON, convert it
//! to a validated [`BillData`] via [`TryFrom`], then render it.
//!
//! ```
//! use swiss_qrust::{BillData, InputBill, Language};
//!
//! let toml = r#"
//! iban = "CH93 0076 2011 6238 5295 7"
//! currency = "CHF"
//! amount = "199.95"
//!
//! [creditor_address]
//! name = "Robert Schneider AG"
//! street = "Rue du Lac"
//! house_num = "1268"
//! plz = "2501"
//! city = "Biel"
//! country = "CH"
//! "#;
//!
//! let input: InputBill = toml::from_str(toml)?;
//! let bill = BillData::try_from(input)?;
//!
//! let svg = swiss_qrust::svg::render_bill_to_svg(&bill, Language::De)?;
//! assert!(svg.starts_with("<svg"));
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! For a PDF instead of SVG, use [`pdf::render_bill_to_pdf`] (returns the
//! raw bytes) or [`pdf::create_pdf`] (writes straight to a file). Both
//! render engines consume the same [`BillData`] - there is nothing
//! PDF-specific or SVG-specific about validating or building the bill
//! itself.
//!
//! [`InputBill`]: input::InputBill
//! [`BillData`]: bill::BillData

pub mod bill;
pub mod render;
pub mod constants;
pub mod language;
pub mod generated;
pub mod support;
pub mod input;

pub use bill::*;
pub use language::*;
pub use generated::*;
pub use support::*;
pub use render::layout::*;
pub use render::engines::*;
pub use render::types::*;
pub use input::*;

/// Parses `content` as an [`InputBill`], choosing the format by `extension`.
///
/// `extension` is matched literally against `"toml"` or `"json"` (as you'd
/// get from `Path::extension()`); anything else is an error. This is a thin
/// convenience wrapper - if you already know the format, call
/// `toml::from_str`/`serde_json::from_str` directly instead.
///
/// ```
/// use swiss_qrust::parse_bill_data;
///
/// let toml = r#"
/// iban = "CH93 0076 2011 6238 5295 7"
/// currency = "CHF"
///
/// [creditor_address]
/// name = "Robert Schneider AG"
/// street = "Rue du Lac"
/// house_num = "1268"
/// plz = "2501"
/// city = "Biel"
/// country = "CH"
/// "#;
///
/// let bill = parse_bill_data(toml, "toml")?;
/// assert_eq!(bill.iban, "CH93 0076 2011 6238 5295 7");
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn parse_bill_data(content: &str, extension: &str) -> std::result::Result<InputBill, ParseBillDataError> {
    match extension {
        "toml" => Ok(toml::from_str(content)?),
        "json" => Ok(serde_json::from_str(content)?),
        other => Err(ParseBillDataError::UnsupportedFormat(other.to_string())),
    }
}

/// Errors from [`parse_bill_data`].
#[derive(Debug, thiserror::Error)]
pub enum ParseBillDataError {
    #[error("invalid TOML: {0}")]
    Toml(#[from] toml::de::Error),
    #[error("invalid JSON: {0}")]
    Json(#[from] serde_json::Error),
    #[error("unsupported input format {0:?}; expected \"toml\" or \"json\"")]
    UnsupportedFormat(String),
}



