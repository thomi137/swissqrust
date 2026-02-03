/*
 * Copyright (c) 2026 Thomas Prosser
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */
use std::fmt::{Display, Formatter};

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
        write!(f, "{:02}{:02}", self.major, self.minor)
    }
}

const QR_TYPE: &'static str = "SPC";


