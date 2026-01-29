use serde::{Deserialize, Serialize};

use crate::validators::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ReferenceType {
    #[serde(rename = "NON")]
    NoRef,

    #[serde(rename = "QRR")]
    QrRef(String),

    #[serde(rename = "SCOR")]
    Creditor(String),
}

impl ReferenceType {
    pub fn validate(&self) -> Result<(), &'static str> {
        match self {
            ReferenceType::NoRef => Ok(()),

            ReferenceType::QrRef(v) => {
                is_valid_qr_reference(v).unwrap();
                Ok(())
            }

            ReferenceType::Creditor(v) => {
                is_valid_iso11649_reference(v).unwrap();
                Ok(())
            }
        }
    }
}