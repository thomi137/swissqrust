/*
 * Copyright (c) 2026 Thomas Prosser
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */

pub mod layout;
pub mod engines;
pub mod types;
mod debug_overlay;

pub use layout::*;
pub use engines::*;
pub use types::*;
use crate::pdf::PdfFontLibrary;
use crate::svg::SvgFontLibrary;

pub trait FontMetrics {

    fn ascender_mm(&self, style: FontStyle, size: Pt) -> Mm;
    fn descender_mm(&self, style: FontStyle, size: Pt) -> Mm;
    fn line_height_mm(&self, style: FontStyle, size: Pt) -> Mm;
    fn text_width_mm(&self, text: &str, style: FontStyle, size: Pt) -> Mm;
}


/// Metrics for PDF fonts. Liberation Sans is embedded by default.
impl FontMetrics for PdfFontLibrary {

    // Get rid of recursion.
    fn ascender_mm(&self, style: FontStyle, size: Pt) -> Mm {
            PdfFontLibrary::get_ascender_mm(self, style, size)
    }

    fn descender_mm(&self, style: FontStyle, size: Pt) -> Mm {
        PdfFontLibrary::get_descender_mm(self, style, size)
    }

    fn line_height_mm(&self, style: FontStyle, size: Pt) -> Mm {
        PdfFontLibrary::get_line_height_mm(self,style, size)
    }
    fn text_width_mm(&self, text: &str, style: FontStyle, size: Pt) -> Mm {
            PdfFontLibrary::get_text_width_mm(self, style, text, size)
    }
}


impl FontMetrics for SvgFontLibrary {
    fn ascender_mm(&self, style: FontStyle, size: Pt) -> Mm {
        // This calls the method defined in 'impl SvgFontLibrary'
        SvgFontLibrary::get_ascender_mm(self, style, size)
    }
    fn descender_mm(&self, style: FontStyle, size: Pt) -> Mm {
        SvgFontLibrary::get_descender_mm(self, style, size)
    }
    fn line_height_mm(&self, style: FontStyle, size: Pt) -> Mm {
        SvgFontLibrary::get_line_height_mm(self, style, size)
    }
    fn text_width_mm(&self, text: &str, style: FontStyle, size: Pt) -> Mm {
        SvgFontLibrary::get_text_width_mm(self, style, text, size)
    }
}
