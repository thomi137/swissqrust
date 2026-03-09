/*
 * Copyright (c) 2026 Thomas Prosser
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */
use crate::constants::A4_PAGE_HEIGHT;
use crate::Mm;

/// Flips y so the origin is at the bottom left. PDF needs ths
///
///  # Arguments:
///
///  * `y` - Distance from top of the layout area.
///
///  # Returns:
///
///  * `LayoutY` a Type that can be converted to PDF coordinates.
///
/// Distance from top of the layout area.
///
/// # Example
///
/// ```
/// # use crate::render::pdf::LayoutY;
/// # use crate::{Mm, A4_PAGE_HEIGHT};
/// let y = LayoutY(Mm(10f32));
/// assert_eq!(y.to_pdf().0, Mm(A4_PAGE_HEIGHT - 10f32));
/// ```
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
