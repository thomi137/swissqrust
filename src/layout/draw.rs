/*
 * Copyright (c) 2026 Thomas Prosser 
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */

use crate::layout::geometry::{Baseline, Mm, Pt, DrawOp};
use crate::QRLayoutRect;

/// Draws a label and moves cursor down by line_spacing.
pub fn draw_label(
    ops: &mut Vec<DrawOp>,
    text: &str,
    x: Mm,
    y: &mut Mm,
    font_size: Pt,
    line_spacing: Mm,
) {
    ops.push(DrawOp::Text {
        text: text.into(),
        at: Baseline { x, y: *y },
        size: font_size,
        bold: true,
    });

    *y = Mm(y.0 - line_spacing.0);
}

/// Draws one text line + block spacing.
pub fn draw_single_line(
    ops: &mut Vec<DrawOp>,
    text: &str,
    x: Mm,
    y: &mut Mm,
    font_size: Pt,
    line_spacing: Mm,
    extra_spacing: Mm,
) {
    ops.push(DrawOp::Text {
        text: text.into(),
        at: Baseline { x, y: *y },
        size: font_size,
        bold: false,
    });

    *y = Mm(y.0 - line_spacing.0 - extra_spacing.0);
}

/// Draws multiple lines.
pub fn draw_text_lines(
    ops: &mut Vec<DrawOp>,
    lines: &[String],
    x: Mm,
    y: &mut Mm,
    font_size: Pt,
    line_spacing: Mm,
    extra_spacing: Mm,
) {
    for line in lines {
        ops.push(DrawOp::Text {
            text: line.clone(),
            at: Baseline { x, y: *y },
            size: font_size,
            bold: false,
        });

        *y = Mm(y.0 - line_spacing.0);
    }

    *y = Mm(y.0 - extra_spacing.0);
}

/// Draws a rectangular box.
pub fn draw_box(
    ops: &mut Vec<DrawOp>,
    x: Mm,
    y: Mm,
    width: Mm,
    height: Mm,
) {
    ops.push(DrawOp::Box {
        rect: QRLayoutRect {
            x,
            y,
            width,
            height,
        },
    });
}

pub fn draw_text_at(
    ops: &mut Vec<DrawOp>,
    text: &str,
    x: Mm,
    y: Mm,
    font_size: Pt,
    bold: bool,
) {
    ops.push(DrawOp::Text {
        text: text.into(),
        at: Baseline { x, y },
        size: font_size,
        bold,
    });
}

