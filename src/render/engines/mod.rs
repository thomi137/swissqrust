/*
 * Copyright (c) 2026 Thomas Prosser
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */

use crate::render::FontMetrics;

pub mod pdf;
pub mod svg;

pub mod png_renderers;
pub mod qr_renderers;

pub enum FontStyle {
    Regular,
    Bold,
}

pub const LIBERATION_SANS_REGULAR_TTF: &[u8] =
    include_bytes!("../../../assets/fonts/LiberationSansRegular.ttf");

pub const LIBERATION_SANS_BOLD_TTF: &[u8] =
    include_bytes!("../../../assets/fonts/LiberationSansBold.ttf");
