/*
 * Copyright (c) 2026 Thomas Prosser
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */

use std::fmt::{Display, Formatter, Write};
use crate::{Address, BillData, ReferenceType};

#[derive(Debug, Clone, Eq, PartialEq)]
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
impl LineSeparator {
    pub fn as_str(&self) -> &'static str {
        match self {
            LineSeparator::CrLF => "\r\n",
            LineSeparator::Lf => "\n",
        }
    }
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
const TRAILER_EPD: &str = "EPD";

pub struct QrBill {
    bill_data: BillData,
    qr_type: String,
    version: String,
    coding_type: String,
}

impl QrBill {
    pub fn new(
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

    pub fn create_qr_text(&self) -> Result<String, QRBillError> {

        // TODO Make available for other line separator as well
        let mut qr_text = QRTextBuilder::new(LineSeparator::default());

        // Header Data. Not expected to change anytime soon.
        qr_text.append_header(&self.qr_type, &self.version, &self.coding_type);

        // IBAN - Mandatory
        qr_text.append_data_field(Some(self.bill_data.iban.as_str()));
        // Creditor - Cdtr
        qr_text.append_person(Some(&self.bill_data.creditor_address));
        // UltmtCdtr - Has to be there, has to be empty
        qr_text.append_person(None);
        qr_text.append_data_field(Some(&self.bill_data.amount));
        qr_text.append_data_field(Some(&self.bill_data.currency.to_string()));

        // UltmtDbtr - Debtor
        qr_text.append_person(self.bill_data.debtor_address.as_ref());

        let ref_type = &self.bill_data.reference_type;
        // RmtInf + Tp - Reference Type
        qr_text.append_data_field(Some(ref_type.code()));

        // Reference
        match ref_type {
            ReferenceType::NoRef => qr_text.append_data_field(None),
            ReferenceType::QrRef(reference) => qr_text.append_data_field(Some(reference)),
            ReferenceType::Creditor(reference) => qr_text.append_data_field(Some(reference)),
        }

        //Trailer - End of Payment Data

        // Additional information
        qr_text.append_data_field(self.bill_data.unstructured_message.as_deref());
        qr_text.append_data_field(Some(TRAILER_EPD));

        Ok(qr_text.build())
    }
}

pub struct QRTextBuilder {
    text: String,
    separator: LineSeparator,
}
impl QRTextBuilder {
    pub fn new(separator: LineSeparator) -> Self {
        Self {
            text: String::new(),
            separator
        }
    }

    pub fn append_data_field(&mut self, value: Option<&str>) {
        let value = value.unwrap_or("");
        self.text.push_str(self.separator.as_str());
        self.text.push_str(value);
    }

    pub fn append_person(&mut self, address: Option<&Address>) {
        if let Some(addr) = address {
            self.append_data_field(Some(&addr.address_type));
            self.append_data_field(Some(&addr.name));
            self.append_data_field(addr.street.as_deref());
            self.append_data_field(addr.house_num.as_deref());
            self.append_data_field(Some(&addr.plz));
            self.append_data_field(Some(&addr.city));
            self.append_data_field(Some(&addr.country.meta().alpha2));
        } else {
            for _ in 0..7 {
                self.append_data_field(None);
            }
        }
    }

    pub fn append_header(&mut self, qr_type: &str, version: &str, coding_type: &str) {
        self.text.push_str(qr_type);
        self.append_data_field(Some(version.as_ref()));
        self.append_data_field(Some(coding_type.as_ref()));
    }

    pub fn build(self) -> String {
        self.text
    }
}
