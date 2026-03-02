/*
 * Copyright (c) 2026 Thomas Prosser
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */
use crate::{DrawOp, Mm, A4_PAGE_HEIGHT};

///
/// Flips y coordinate. In the TV age, x,y started top left.
/// Then, Apple and hence PostScript and PDF started using
/// the origin at the conventional position bottom left.
///
/// This flips y so the layout layer can stay regardless of what the renderer does.
/// So this converts from top down to a bottom up position
///
/// Distance from top of the layout area.

#[derive(Copy, Clone, Debug)]
pub struct LayoutY(pub Mm);

impl LayoutY {

    #[inline]
    pub fn to_pdf(self) -> PdfY {
        PdfY(A4_PAGE_HEIGHT - self.0)
    }

    pub fn to_pdf_with_height(self, page_height: Mm, obj_height: Mm) -> Mm {
        page_height - self.0 - obj_height
    }
}

#[derive(Copy, Clone, Debug)]
pub struct PdfY(pub Mm);