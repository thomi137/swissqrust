/*
 * Copyright (c) 2026 Thomas Prosser
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */

use crate::{BillData, DrawOp, Language, Mm, PaymentPartLayout};
use crate::constants::{TITLE_FONT_SIZE, PP_LABEL_PREF_FONT_SIZE, PP_TEXT_PREF_FONT_SIZE};
use crate::render::FontMetrics;

pub mod pdf;

pub mod png_renderers;
pub mod svg;
pub mod qr_renderers;

pub enum FontStyle {
    Regular,
    Bold,
}

pub const LIBERATION_SANS_REGULAR_TTF: &[u8] =
    include_bytes!("../../../assets/fonts/LiberationSansRegular.ttf");

pub const LIBERATION_SANS_BOLD_TTF: &[u8] =
    include_bytes!("../../../assets/fonts/LiberationSansBold.ttf");

/*pub fn generate_bill_ops(
    bill: &BillData,
    language: Language,
    fonts: &impl FontMetrics
) -> Vec<DrawOp> {

    let pp_title_ascender = fonts.ascender_mm(FontStyle::Bold, TITLE_FONT_SIZE);
    let pp_label_ascender = fonts.ascender_mm(FontStyle::Regular, PP_LABEL_PREF_FONT_SIZE);
    let pp_text_ascender = fonts.ascender_mm(FontStyle::Regular, PP_TEXT_PREF_FONT_SIZE);
    let pp_line_spacing: Mm = (PP_TEXT_PREF_FONT_SIZE) .to_mm();

    let mut payment_part = PaymentPartLayout::new();
    payment_part.render(&mut builder.ops);

}*/