use once_cell::sync::Lazy;
use regex::Regex;

static AMOUNT_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\d{1,9}\.\d{2}").unwrap());


/// This is all in Millimeters and all in DIN A4

/// A4 Width in mm
const A4_PORTRAIT_WIDTH: u16 = 210;

/// A4 Height in mm
const A4_PORTRAIT_HEIGHT: u16 = 297;

/// Width of a QR Slip is incidentally the same as the whole paper slip 🤣
const QR_BILL_SLIP_WIDTH: u16 = 210;

/// Now the height of the slip itself
const QR_BILL_HEIGHT: u16 = 105;

/// Then the width with scissors symbol and all... Surprise
const QR_BILL_WITH_ALL_WIDTH: u16 = 210;

/// Takes 5 mm more with all the scissors svg.
const QR_BILL_WITH_HORI_LINE_HEIGHT: u16 = 110;

/// QR Code Width
const QR_CODE_WIDTH: u16 = 46;

/// QR Code Height
const QR_CODE_HEIGHT: u16 = 46;

/// QR Code Padding
const QR_CODE_PADDING: u16 = 56;

/// QR Width in mm
const QR_CODE_OVERALL_WIDTH: u16 = 148;

