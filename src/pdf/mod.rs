/*
 * Copyright (c) 2026 Thomas Prosser 
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */

mod fonts;
pub mod pdf_builder;

use pdf_writer::{Name, Pdf, Ref};
use fonts::*;

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
}

pub fn name(style: FontStyle) -> Name<'static> {
    match style {
        FontStyle::Regular => LIBERATION_SANS_REG_NAME,
        FontStyle::Bold => LIBERATION_SANS_BOLD_NAME,
    }
}

