/*
 * Copyright (c) 2026 Thomas Prosser
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */

pub mod fonts;
pub mod draw_op_handler;
pub mod pdf_helpers;
pub mod render_bill;
pub mod coords;

use pdf_writer::{Name, Pdf, Ref};
pub use fonts::*;
pub use draw_op_handler::*;
pub use pdf_helpers::*;
use crate::{render, FontStyle, Mm, Pt, LIBERATION_SANS_BOLD_TTF, LIBERATION_SANS_REGULAR_TTF, MM_PER_PT};

const LIBERATION_SANS_REG_NAME: Name = Name(b"LiberationSansRegular");
const LIBERATION_SANS_BOLD_NAME: Name = Name(b"LiberationSansBold");

pub struct PdfFontLibrary {
    pub regular: EmbeddedFont,
    pub bold: EmbeddedFont,
}

impl PdfFontLibrary {
    pub fn new(pdf: &mut Pdf, next_id: &mut Ref) -> Self {
        Self {
            regular: embed_ttf_font(pdf, next_id, render::pdf::name(FontStyle::Regular), LIBERATION_SANS_REGULAR_TTF),
            bold: embed_ttf_font(pdf, next_id, render::pdf::name(FontStyle::Bold), LIBERATION_SANS_BOLD_TTF),
        }
    }

    pub fn get_ascender_mm(&self, style: FontStyle, size: Pt) -> Mm {
        let font = match style {
            FontStyle::Regular => &self.regular,
            FontStyle::Bold => &self.bold,
        };

        let ratio = font.face.ascender() as f32
            / font.face.units_per_em() as f32;

        let asc_pt = ratio * size.0;

        Mm(asc_pt * MM_PER_PT)
    }

    pub fn get_descender_mm(&self, style: FontStyle, size: Pt) -> Mm {
        let font = match style {
            FontStyle::Regular => &self.regular,
            FontStyle::Bold => &self.bold,
        };

        let ratio = font.face.descender() as f32
            / font.face.units_per_em() as f32;

        let desc_pt = ratio * size.0;

        Mm(desc_pt * MM_PER_PT)
    }

    pub fn get_line_height_mm(&self, style: FontStyle, size: Pt) -> Mm {
        let font = match style {
            FontStyle::Regular => &self.regular,
            FontStyle::Bold => &self.bold,
        };

        let face = &font.face;
        let units_per_em = face.units_per_em() as f32;

        // Sum the full typographic bounds
        let total_units = (face.ascender() as f32)
            - (face.descender() as f32)
            + (face.line_gap() as f32);

        let ratio = total_units / units_per_em;

        Mm(ratio * size.0 * MM_PER_PT)
    }

    pub fn get_text_width_mm(&self, style:FontStyle, text: &str, size: Pt) -> Mm {
        let font = match style {
            FontStyle::Regular => &self.regular,
            FontStyle::Bold => &self.bold,
        };
        let units_per_em = font.face.units_per_em() as f32;
        let mut total_advance = 0;

        for c in text.chars() {
            if let Some(glyph_id) = font.face.glyph_index(c) {
                // Retrieve advance width in font design units
                total_advance += font.face.glyph_hor_advance(glyph_id).unwrap_or(0) as u32;
            }
        }

        // Convert FUnits to points: (Advance * FontSize) / UnitsPerEm
        let width_pt = (total_advance as f32 * size.0) / units_per_em;

        // Convert points to mm: 1pt = 25.4 / 72 mm
        Mm(width_pt * MM_PER_PT)
    }

}

