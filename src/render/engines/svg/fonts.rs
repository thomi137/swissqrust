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
        let face = match style {
            FontStyle::Regular => &self.regular_face,
            FontStyle::Bold => &self.bold_face,
        };
        let ratio = face.ascender() as f32 / face.units_per_em() as f32;
        Mm(ratio * size.0 * MM_PER_PT)
    }

    pub fn descender_mm(&self, style: FontStyle, size: Pt) -> Mm {
        let face = match style {
            FontStyle::Regular => &self.regular_face,
            FontStyle::Bold => &self.bold_face,
        };
        let ratio = face.descender() as f32 / face.units_per_em() as f32;
        Mm(ratio * size.0 * 25.4 / 72.0)
    }

}