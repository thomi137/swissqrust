use once_cell::sync::Lazy;
use regex::Regex;
use crate::Address;
use crate::bill::reference_type::ReferenceType;

pub static AMOUNT_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^\d{1,9}\.\d{2}$").unwrap());

/// This is all in Millimeters and all in DIN A4

/// A4 Width in mm
pub const A4_PORTRAIT_WIDTH: u16 = 210;

/// A4 Height in mm
pub const A4_PORTRAIT_HEIGHT: u16 = 297;

/// Width of a QR Slip is incidentally the same as the whole paper slip 🤣
pub const QR_BILL_SLIP_WIDTH: u16 = 210;

/// Now the height of the slip itself
pub const QR_BILL_HEIGHT: u16 = 105;

/// Then the width with scissors symbol and all... Surprise
pub const QR_BILL_WITH_ALL_WIDTH: u16 = 210;

/// Takes 5 mm more with all the scissors svg.
pub const QR_BILL_WITH_HORI_LINE_HEIGHT: u16 = 110;

/// QR Code Width
pub const QR_CODE_WIDTH: u16 = 46;

/// QR Code Height
pub const QR_CODE_HEIGHT: u16 = 46;

/// QR Code Padding
pub const QR_CODE_PADDING: u16 = 56;

/// QR Width in mm
pub const QR_CODE_OVERALL_WIDTH: u16 = 148;


pub enum Version {
    V2_0,
}

pub enum Country {
    CH,
    LI
}

struct Bill {
    version: Version,
    creditor_address: Address,
    debtor_address: Address,
    country: Country,
    currency: String,
    amount: f64,
    reference_type: ReferenceType,
    reference: String,
    unstructured_message: Option<String>,
    bill_information: String,
}