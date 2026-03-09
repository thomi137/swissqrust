/*
 * Copyright (c) 2026 Thomas Prosser
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */

use svg::node::element::{Rectangle, Text as SvgText, Line as SvgLine};
use svg::Document;

use qrcodegen::QrCode;
use crate::{DrawOp, MM_PER_PT};
use crate::render::qr_renderers::{render_qr_svg, add_swiss_cross};
use crate::svg::fonts::SvgFontLibrary;

/// This is the equivaaent of the draw_op_handler for the PDF engine.
/// It renders to a PDF
///
/// TODO: Integrate Liberation Sans font.
/// TODO: Should also be extracted. But has no priority
pub fn execute_bill_ops_svg (
    ops: Vec<DrawOp>,
    qr_data: Option<&QrCode>,
    fonts: SvgFontLibrary,
) -> String {

    let mut doc = Document::new()
        .set("viewBox", (0, 0, 210, 297))
        .set("class", "swiss-qr-preview");

    for op in ops.iter() {
        match op {
            DrawOp::Text { text, at, size, bold } => {
                let txt = SvgText::new(text)
                    .set("x", at.x.0)
                    .set("y", at.y.0.0) // SVG is naturally top-left!
                    .set("font-size", &size.0 * MM_PER_PT) // viewport units (mm in this case)
                    .set("font-family", "Liberation Sans, Arial, sans-serif")
                    .set("font-weight", if *bold { "bold" } else { "normal" });
                doc = doc.add(txt);
            },
            DrawOp::Line { from, to, width } => {
                let line = SvgLine::new()
                    .set("x1", from.0 .0)
                    .set("y1", from.1.0 .0)
                    .set("x2", to.0 .0)
                    .set("y2", to.1.0 .0)
                    .set("stroke", "black")
                    .set("stroke-width", width.0);
                doc = doc.add(line);
            },
            DrawOp::Box { rect } => {
                let box_rect = Rectangle::new()
                    .set("x", rect.x.0)
                    .set("y", rect.y.0)
                    .set("width", rect.width.0)
                    .set("height", rect.height.0)
                    .set("fill", "none")
                    .set("stroke", "black")
                    .set("stroke-width", 0.2); // 0.75pt approx
                doc = doc.add(box_rect);
            },
            DrawOp::QrCodeSpace { .. } => {
            if let Some(qr) = qr_data {

                // Reuse your existing render_qr_svg + add_swiss_cross
                let qr_group = add_swiss_cross(
                    render_qr_svg(qr.clone())
                );
                doc = doc.add(qr_group);
            }
        },
        }
    }
    doc.to_string()
}

