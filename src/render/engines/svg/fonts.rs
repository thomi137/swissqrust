/*
 * Copyright (c) 2026 Thomas Prosser
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */

use ttf_parser::Face;
use crate::{FontStyle, Mm, Pt, LIBERATION_SANS_BOLD_TTF, LIBERATION_SANS_REGULAR_TTF, MM_PER_PT};

pub struct SvgFontLibrary {
    regular_face: Face<'static>,
    bold_face: Face<'static>,
}

impl SvgFontLibrary {
    pub fn new() -> Self {
        Self {
            regular_face: Face::parse(LIBERATION_SANS_REGULAR_TTF, 0).expect("Invalid Regular Font"),
            bold_face: Face::parse(LIBERATION_SANS_BOLD_TTF, 0).expect("Invalid Bold Font"),
        }
    }

    pub fn get_ascender_mm(&self, style: FontStyle, size: Pt) -> Mm {
        let face = self.get_face(style);

        let ratio = face.ascender() as f32 / face.units_per_em() as f32;
        Mm(ratio * size.0 * MM_PER_PT)
    }

    pub fn get_descender_mm(&self, style: FontStyle, size: Pt) -> Mm {
        let face = self.get_face(style);
        let ratio = face.descender() as f32 / face.units_per_em() as f32;
        Mm(ratio * size.0 * MM_PER_PT)
    }

    pub fn get_line_height_mm(&self, style: FontStyle, size: Pt) -> Mm {
        let face = self.get_face(style);
        let units_per_em = face.units_per_em() as f32;

        // Sum the full typographic bounds
        let total_units = (face.ascender() as f32)
            - (face.descender() as f32)
            + (face.line_gap() as f32);

        let ratio = total_units / units_per_em;

        Mm(ratio * size.0 * MM_PER_PT)
    }

    pub fn get_text_width_mm(&self, style:FontStyle, text: &str, size: Pt) -> Mm {
        let face = self.get_face(style);

        let units_per_em = face.units_per_em() as f32;
        let mut total_advance = 0;

        for c in text.chars() {
            if let Some(glyph_id) = face.glyph_index(c) {
                // Retrieve advance width in font design units
                total_advance += face.glyph_hor_advance(glyph_id).unwrap_or(0) as u32;
            }
        }

        // Convert FUnits to points: (Advance * FontSize) / UnitsPerEm
        let width_pt = (total_advance as f32 * size.0) / units_per_em;

        // Convert points to mm: 1pt = 25.4 / 72 mm
        Mm(width_pt * MM_PER_PT)
    }

    fn get_face(&self, style: FontStyle) -> &Face<'static> {
        match style {
            FontStyle::Regular => &self.regular_face,
            FontStyle::Bold => &self.bold_face,
        }
    }

}

