/*
 * Copyright (c) 2026 Thomas Prosser
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */

use anyhow::*;

include!("generated/countries.rs");
include!("generated/cross.rs");
include!("generated/corner_marks_amount.rs");
include!("generated/corner_marks_payable_by.rs");

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

/// Reads file and decides whether it is .toml or .json
/// Maybe later used only for a CLI version, but keep it in here.
pub fn parse_bill_data(content: &str, extension: &str) -> Result<InputBill> {
    match extension {
        "toml" => Ok(toml::from_str(content)?),
        "json" => Ok(serde_json::from_str(content)?),
        other => Err(anyhow::anyhow!("Unsupported input format: {other}")),
    }
}



