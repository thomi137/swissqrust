use crate::{Baseline, Mm, Pt, QRBillLayoutRect};

pub enum DrawOp {
    Text { text: String, at: Baseline, size: Pt, bold: bool },
    Box { rect: QRBillLayoutRect },
    Line { from: (Mm, Mm), to: (Mm, Mm), width: Mm },
    QrCodeSpace { at: Baseline, size: Mm },
}
