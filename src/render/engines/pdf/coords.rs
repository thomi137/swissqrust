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
/// ```
/// use swiss_qrust::support::utils::top_down_y;
/// use swiss_qrust::Mm;
/// assert_eq!(top_down_y(Mm(10.0)), Mm(287.0));
/// ```
/// So this converts from top down to a bottom up position
///
#[inline]
pub fn layout_y_to_pdf_y(y_from_top: Mm, page_height: Mm) -> Mm {
    page_height - y_from_top
}