/*
 * Copyright (c) 2026 Thomas Prosser
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */
use crate::Mm;

pub struct Spacing {
    pub line_spacing: Mm,
    pub extra_spacing: Mm,
}

/// Computes dynamic vertical spacing between text blocks.
///
/// max_height: total available vertical space
/// fixed_height: height occupied by non-text elements (e.g. boxes)
/// text_lines: total number of text lines
/// extra_blocks: number of gaps between text blocks
/// text_line_height: height of one text line
pub fn compute_spacing(
    max_height: Mm,
    fixed_height: Mm,
    text_lines: usize,
    extra_blocks: usize,
    text_line_height: Mm,
) -> Spacing {
    let total_text_height = Mm(text_line_height.0 * text_lines as f32);
    let remaining = Mm(
        max_height.0 - fixed_height.0 - total_text_height.0
    );

    let mut extra = if extra_blocks > 0 {
        Mm(remaining.0 / extra_blocks as f32)
    } else {
        Mm(0.0)
    };
    if extra.0 < 0.0 {
        extra = Mm(0.0);
    }
    if extra.0 > text_line_height.0 {
        extra = text_line_height;
    }

    Spacing {
        line_spacing: text_line_height,
        extra_spacing: extra,
    }

}
