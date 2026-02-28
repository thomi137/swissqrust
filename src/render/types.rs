use crate::{Baseline, Mm, Polygon, Pt, QRBillLayoutRect};

#[derive(Debug)]
pub enum DrawOp {
    Text { text: String, at: Baseline, size: Pt, bold: bool },
    Box { rect: QRBillLayoutRect },
    Line { from: (Mm, Mm), to: (Mm, Mm), width: Mm },
    QrCodeSpace { at: Baseline, size: Mm },
}
