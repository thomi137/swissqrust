/*
 * Copyright (c) 2026 Thomas Prosser
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */

use svg::node::element::{Rectangle, Text as SvgText, Line as SvgLine};
use svg::Document;
use qrcodegen::QrCode;
use base64::{engine::general_purpose, Engine as _};

use crate::{DrawOp, LIBERATION_SANS_BOLD_TTF, LIBERATION_SANS_REGULAR_TTF, MM_PER_PT};
use crate::pdf::render_bill::RenderError;
use crate::render::qr_renderers::{render_qr_svg, add_swiss_cross};
use crate::svg::fonts::SvgFontLibrary;

/// Handles SVG rendering operations
pub fn execute_bill_ops_svg (
    ops: Vec<DrawOp>,
    qr_data: Option<&QrCode>,
) -> Result<String, RenderError> {

    // 1. Prepare the Data URIs
    let reg_uri = get_font_data_uri(LIBERATION_SANS_REGULAR_TTF);
    let bold_uri = get_font_data_uri(LIBERATION_SANS_BOLD_TTF);

    // 2. Create the CSS string
    let font_css = format!(r#"
        @font-face {{
            font-family: 'LiberationSans';
            src: url('{}') format('truetype');
            font-weight: normal;
            font-style: normal;
        }}
        @font-face {{
            font-family: 'LiberationSans';
            src: url('{}') format('truetype');
            font-weight: bold;
            font-style: normal;
        }}
    "#, reg_uri, bold_uri);

    let y_offset = 192.0;

    let mut doc = Document::new()
        .set("viewBox", (0, 0, 210, 105))
        .set("width", "100%")
        .set("height", "100%")
        .set("shape-rendering", "geometricPrecision")
        .set("class", "swiss-qr-preview");

    for op in ops.iter() {
        match op {
            DrawOp::Text { text, at, size, bold } => {
                // The slip starts at 192mm from the top of an A4 page (297 - 105)
                let svg_y = at.y.0.0 - y_offset;
                let txt = SvgText::new(text)
                    .set("x", at.x.0)
                    .set("y", svg_y) // SVG is naturally top-left!
                    .set("font-size", &size.0 * MM_PER_PT) // viewport units (mm in this case)
                    .set("font-family", "LiberationSans, Arial, sans-serif")
                    .set("font-weight", if *bold { "bold" } else { "normal" });
                doc = doc.add(txt)
            },
            DrawOp::Line { from, to, width } => {
                let line = SvgLine::new()
                    .set("x1", from.0 .0)
                    .set("y1", from.1.0.0 -  y_offset)
                    .set("x2", to.0 .0)
                    .set("y2", to.1.0.0 - y_offset)
                    .set("stroke", "black")
                    .set("stroke-width", width.0);
                doc = doc.add(line);
            },
            DrawOp::Box { rect } => {
                let box_rect = Rectangle::new()
                    .set("x", rect.x.0)
                    .set("y", rect.y.0 - y_offset)
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
    Ok(doc.to_string())
}

fn get_font_data_uri(bytes: &[u8]) -> String {
    let b64 = general_purpose::STANDARD.encode(bytes);
    format!("data:font/ttf;base64,{}", b64)
}

