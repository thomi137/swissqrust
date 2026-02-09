include!("generated/countries.rs");
include!("generated/cross.rs");

pub mod cli;
pub mod validators;
pub mod utils;
pub mod bill;
pub mod language;
pub mod svg;

pub use bill::*;

