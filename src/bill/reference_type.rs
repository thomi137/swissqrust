use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ReferenceType {
    #[serde(rename = "NON")]
    NoRef,

    #[serde(rename = "QRR")]
    QrRef,

    #[serde(rename = "SCOR")]
    Creditor,
}