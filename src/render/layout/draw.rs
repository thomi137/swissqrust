/*
 * Copyright (c) 2026 Thomas Prosser 
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */

use crate::render::types::DrawOp;
use crate::{Baseline, FontStyle, Mm, Pt, QRBillLayoutRect};
use crate::pdf::coords::LayoutY;
use crate::render::FontMetrics;
use crate::shapes::Polygon;

/// Truncates `text` to fit `available_width`, appending "…" if it had to be
/// cut. Per Style Guide 3.5.4/3.5.5, overflowing text on the payment part
/// must be marked with a trailing ellipsis rather than silently clipped.
pub fn truncate_to_width<T: FontMetrics>(
    metrics: &T,
    text: &str,
    style: FontStyle,
    size: Pt,
    available_width: Mm,
) -> String {
    if metrics.text_width_mm(text, style, size).0 <= available_width.0 {
        return text.to_string();
    }
    let mut chars: Vec<char> = text.chars().collect();
    while !chars.is_empty() {
        chars.pop();
        let candidate: String = chars.iter().collect::<String>() + "…";
        if metrics.text_width_mm(&candidate, style, size).0 <= available_width.0 {
            return candidate;
        }
    }
    "…".to_string()
}

/// Draws a label and moves cursor down by line_spacing.
pub fn draw_label(
    ops: &mut Vec<DrawOp>,
    text: &str,
    x: Mm,
    y: &mut Mm,
    font_size: Pt,
) {
    ops.push(DrawOp::Text {
        text: text.into(),
        at: Baseline { x, y: LayoutY(*y) },
        size: font_size,
        bold: true,
    });

   // y.0 += line_spacing.0;
}

/// Draws one text line + block spacing.
pub fn draw_single_line(
    ops: &mut Vec<DrawOp>,
    text: &str,
    x: Mm,
    y: &mut Mm,
    font_size: Pt,
) {
    ops.push(DrawOp::Text {
        text: text.into(),
        at: Baseline { x, y: LayoutY(*y) },
        size: font_size,
        bold: false,
    });
}

/// Draws multiple lines.
pub fn draw_text_lines(
    ops: &mut Vec<DrawOp>,
    lines: &[String],
    x: Mm,
    y: &mut Mm,
    font_size: Pt,
    line_spacing: Mm,
) {
    for line in lines {
        ops.push(DrawOp::Text {
            text: line.clone(),
            at: Baseline { x, y: LayoutY(*y) },
            size: font_size,
            bold: false,
        });

        *y = Mm(y.0 + line_spacing.0);
    }

   // *y = Mm(y.0 + extra_spacing.0);
}

/// Draws a rectangular box.
/// These are top-down coordinates, meaning
/// y and height are positive in the downward direction
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
        at: Baseline { x, y: LayoutY(y) },
        size: font_size,
        bold,
    });
}

#[derive(Debug)]
pub enum CornerMarksViewBox {
    CMAmount,
    CMPayableBy,
}
/// Points-to-millimetre factor (1 pt = 1/72 inch). The corner-mark source
/// artwork is authored in points at its native (receipt-sized) box; the
/// Style Guide (p. 7) fixes the mark itself at 3 x 3 mm / 0.75 pt regardless
/// of the surrounding placeholder box's dimensions, so this factor - not a
/// per-box scale - is what converts the artwork to millimetres.
const POINTS_TO_MM: f32 = 25.4 / 72.0;

/// Draws the four Eckmarken (corner marks) for an empty placeholder box.
///
/// Each of the four polylines in `polylines` represents one L-shaped mark,
/// anchored to one corner of `viewbox`. Marks are re-anchored to the
/// matching corner of `rect` at their fixed native size (not stretched to
/// fill `rect`), since a payment-part box (e.g. 65x25mm) and its receipt
/// counterpart (52x20mm) must show identically-sized corner marks.
pub fn draw_corner_marks(
    ops: &mut Vec<DrawOp>,
    rect: QRBillLayoutRect,
    viewbox: (f64, f64),
    polylines: &[Polygon]
) {
    let vb_w = viewbox.0 as f32;
    let vb_h = viewbox.1 as f32;

    for poly in polylines {
        // Draw each polyline as a series of connected lines
        if poly.points.len() < 2 {
            continue;
        }

        // Every point of a given corner mark lies in the same quadrant of
        // the source artwork, so the first point tells us which corner of
        // `rect` this mark belongs to.
        let (fx, fy) = (poly.points[0].0 as f32, poly.points[0].1 as f32);
        let left = fx < vb_w / 2.0;
        let top = fy < vb_h / 2.0;

        let anchor = |px: f32, py: f32| -> (Mm, LayoutY) {
            let off_x = if left { px } else { vb_w - px };
            let off_y = if top { py } else { vb_h - py };
            let mm_x = off_x * POINTS_TO_MM;
            let mm_y = off_y * POINTS_TO_MM;

            let x = if left { rect.x.0 + mm_x } else { rect.x.0 + rect.width.0 - mm_x };
            let y = if top { rect.y.0 + rect.height.0 - mm_y } else { rect.y.0 + mm_y };
            (Mm(x), LayoutY(Mm(y)))
        };

        for window in poly.points.windows(2) {
            let (p1x, p1y) = (window[0].0 as f32, window[0].1 as f32);
            let (p2x, p2y) = (window[1].0 as f32, window[1].1 as f32);

            ops.push(DrawOp::Line {
                from: anchor(p1x, p1y),
                to: anchor(p2x, p2y),
                width: Mm(0.21), // 0.75pt ≈ 0.21mm
            });
        }
    }
}
