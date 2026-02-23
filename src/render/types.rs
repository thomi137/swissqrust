use crate::{Baseline, Mm, Polygon, Pt, QRBillLayoutRect};

pub enum DrawOp<'a> {
    Text { text: String, at: Baseline, size: Pt, bold: bool },
    Box { rect: QRBillLayoutRect },
    Line { from: (Mm, Mm), to: (Mm, Mm), width: Mm },
    QrCodeSpace { at: Baseline, size: Mm },
    CornerMarks { at: Baseline, polygon: &'a [Polygon], viewbox: (f64, f64) },
}
