use crate::{Baseline, Mm, Polygon, Pt, QRBillLayoutRect};
use crate::coords::LayoutY;

#[derive(Debug)]
pub enum DrawOp {
    Text { text: String, at: Baseline, size: Pt, bold: bool },
    Box { rect: QRBillLayoutRect },
    Line { from: (Mm, LayoutY), to: (Mm, LayoutY), width: Mm },
    QrCodeSpace { at: Baseline, size: Mm },
}
