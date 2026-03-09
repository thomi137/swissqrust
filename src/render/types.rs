use crate::{Baseline, Mm, Pt, QRBillLayoutRect};
use crate::pdf::coords::LayoutY;

#[derive(Debug)]
pub enum DrawOp {
    Text { text: String, at: Baseline, size: Pt, bold: bool },
    Box { rect: QRBillLayoutRect },
    Line { from: (Mm, LayoutY), to: (Mm, LayoutY), width: Mm },
    QrCodeSpace { at: Baseline, size: Mm },
}
