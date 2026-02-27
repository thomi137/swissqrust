/*
 * Copyright (c) 2026 Thomas Prosser
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */

pub mod fonts;
pub mod draw_op_handler;
pub mod pdf_helpers;
pub mod pdf_state;
pub mod render_bill;

use pdf_writer::{Name, Pdf, Ref};
pub use fonts::*;
pub use draw_op_handler::*;
pub use pdf_helpers::*;
pub use pdf_state::*;
use crate::{Mm, Pt};

const LIBERATION_SANS_REG_NAME: Name = Name(b"LiberationSansRegular");
const LIBERATION_SANS_BOLD_NAME: Name = Name(b"LiberationSansBold");

pub enum FontStyle {
    Regular,
    Bold,
}

pub struct FontLibrary {
    pub regular: EmbeddedFont,
    pub bold: EmbeddedFont,
}

impl FontLibrary {
    pub fn new(pdf: &mut Pdf, next_id: &mut Ref) -> Self {
        Self {
            regular: embed_ttf_font(pdf, next_id, name(FontStyle::Regular), LIBERATION_SANS_REGULAR_TTF),
            bold: embed_ttf_font(pdf, next_id, name(FontStyle::Bold), LIBERATION_SANS_BOLD_TTF),
        }
    }

    pub fn ascender_mm(&self, style: FontStyle, size: Pt) -> Mm {
        let font = match style {
            FontStyle::Regular => &self.regular,
            FontStyle::Bold => &self.bold,
        };

        let ratio = font.face.ascender() as f32
            / font.face.units_per_em() as f32;

        let asc_pt = ratio * size.0;

        Mm(asc_pt * 25.4 / 72.0)
    }
}

pub fn name(style: FontStyle) -> Name<'static> {
    match style {
        FontStyle::Regular => LIBERATION_SANS_REG_NAME,
        FontStyle::Bold => LIBERATION_SANS_BOLD_NAME,
    }
}
