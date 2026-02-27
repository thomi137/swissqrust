/*
 * Copyright (c) 2026 Thomas Prosser 
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */

use crate::render::types::DrawOp;
use crate::{Baseline, Mm, Pt, QRBillLayoutRect};
use crate::shapes::Polygon;

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

    y.0 -= line_spacing.0;
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
        rect: QRBillLayoutRect {
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

#[derive(Debug)]
pub enum CornerMarksViewBox {
    CMAmount,
    CMPayableBy,
}
pub fn draw_corner_marks(
    ops: &mut Vec<DrawOp>,
    rect: QRBillLayoutRect,
    viewbox: (f64, f64),
    polylines: &[Polygon]
) {
    let scale_x = rect.width.0 / viewbox.0 as f32;
    let scale_y = rect.height.0 / viewbox.1 as f32;

    for poly in polylines {
        // Draw each polyline as a series of connected lines
        if poly.points.len() < 2 {
            continue;
        }

        for window in poly.points.windows(2) {
            let p1 = window[0];
            let p2 = window[1];

            ops.push(DrawOp::Line {
                // PDF Y is bottom-up, SVG Y is top-down:
                // We map SVG(x, y) to PDF(rect.x + x, (rect.y + rect.height) - y)
                from: (
                    Mm(rect.x.0 + (p1.0 as f32 * scale_x)),
                    Mm(rect.y.0 + rect.height.0 - (p1.1 as f32 * scale_y)),
                ),
                to: (
                    Mm(rect.x.0 + (p2.0 as f32 * scale_x)),
                    Mm(rect.y.0 + rect.height.0 - (p2.1 as f32 * scale_y)),
                ),
                width: Mm(0.21), // 0.75pt ≈ 0.21mm
            });
        }
    }
}
