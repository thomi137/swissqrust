/*
 * Copyright (c) 2026 Thomas Prosser
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */
use std::fmt::{Display, Formatter};
use bincode::config::FixintEncoding;
use svg::node::NodeClone;
use crate::BillData;

pub enum QRBillError {}

/// According to the [spec] (https://www.six-group.com/dam/download/banking-services/standardization/qr-bill/ig-qr-bill-v2.3-de.pdf)
/// section 4.1.4, there are two allowed line separators.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub enum LineSeparator{
    // Carriage return and line feed. Unicode U+000D and U+000A.
    #[default]
    CrLF,
    // Line feed. Unicode U+000A.
    Lf
}
impl Display for LineSeparator {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            LineSeparator::CrLF => f.write_str("\r\n"),
            LineSeparator::Lf => f.write_str("\n"),
        }
    }
}


/// At the moment, only version "0200" is accepted. Starting from
/// version 3, minors are accepted.
///
/// ```
/// use swiss_qrust::qr_bill::Version;
/// let version = Version{ major: 2, minor: 0};
/// assert_eq!(version.qr_code_version(), "0200");
/// ```
///
/// Alternatively, you can also use .to_string():
///
/// ```
/// use swiss_qrust::qr_bill::Version;
/// let version = Version{ major: 2, minor: 0};
/// assert_eq!(version.to_string(), "0200");
/// ```
///
/// Seems kind of redundant, but could be helpful internally to
/// implement Display.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Version {
    pub major: u8,
    pub minor: u8,
}

impl Version {
    pub fn qr_code_version(&self) -> String {
        format!("{:02}{:02}", self.major, self.minor)
    }
}
impl Display for Version {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.qr_code_version())
    }
}

const QR_TYPE: &'static str = "SPC";
const VERSION: Version = Version { major: 2, minor: 0 };
const CODING_TYPE: &'static str = "1";

pub struct QrBill {
    bill_data: BillData,
    qr_type: String,
    version: String,
    coding_type: String,
}

impl QrBill {
    pub fn new(
        qr_type: String,
        version: String,
        coding_type: String,
        bill_data: BillData,
    ) -> Result<Self, QRBillError> {
        let qr_type = QR_TYPE.to_string();
        let version = VERSION.to_string();
        let coding_type = CODING_TYPE.to_string();
      
        
        Ok(QrBill{
            bill_data,
            qr_type,
            version,
            coding_type,
        })
    }
}
    
