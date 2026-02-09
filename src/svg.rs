/*
 * Copyright (c) 2026 Thomas Prosser
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */

use qrcodegen::QrCode;
use svg::Document;
use svg::node::element::{Rectangle, Polygon as SvgPolygon};
use crate::{CROSS_POLYGONS, CROSS_RECTS};

const PX_PER_MM: f64 = 3.543307;
const MODULE_MM: f64 = 0.4;
const QUIET_ZONE_MODULES: usize = 4;
const QR_MM: f64 = 46.0;
const CROSS_MM: f64 = 7.0;
const QUIET_MM: f64 = 1.6f64;
const CROSS_VIEWBOX: (f64, f64) = (19.8, 19.8);


/// Render a Swiss QR Code SVG (modules only).
/// The Swiss cross area is cleared here.
/// The official SVG cross is rendered later on top.
pub fn render_qr_svg(qr: QrCode) -> Document {
    let modules = qr.size();
    let drawable_mm = QR_MM - 2.0 * QUIET_MM;
    let module_mm = drawable_mm / modules as f64;

    let mut doc = Document::new()
        .set("width", QR_MM)
        .set("height", QR_MM)
        .set("viewBox", format!("0 0 {} {}", QR_MM, QR_MM));

    for row in 0..modules {
        for col in 0..modules {
            if !qr.get_module(col, row) {
                continue;
            }

            let x = QUIET_MM + col as f64 * module_mm;
            let y = QUIET_MM + row as f64 * module_mm;

            // module center in mm
            let cx = x + module_mm / 2.0;
            let cy = y + module_mm / 2.0;

            doc = doc.add(
                Rectangle::new()
                    .set("x", x)
                    .set("y", y)
                    .set("width", module_mm)
                    .set("height", module_mm)
                    .set("fill", "black"),
            );
        }
    }

    doc
}


/// Clears QR modules and draws Swiss cross centered
pub fn add_swiss_cross(mut doc: svg::Document) -> svg::Document {
    let scale = CROSS_MM / CROSS_VIEWBOX.0; // 7 / 19.8
    let offset = (QR_MM - CROSS_MM) / 2.0;  // 19.5

    // Clear QR modules underneath (white background)
    doc = doc.add(
        Rectangle::new()
            .set("x", offset)
            .set("y", offset)
            .set("width", CROSS_MM)
            .set("height", CROSS_MM)
            .set("fill", "white"),
    );

    // Draw polygon parts of cross
    for poly in CROSS_POLYGONS {
        let points = poly
            .points
            .iter()
            .map(|(x, y)| {
                format!(
                    "{},{}",
                    x * scale + offset,
                    y * scale + offset
                )
            })
            .collect::<Vec<_>>()
            .join(" ");

        doc = doc.add(
            SvgPolygon::new()
                .set("points", points)
                .set("fill", "black"),
        );
    }

    for r in CROSS_RECTS {
        doc = doc.add(
            Rectangle::new()
                .set("x", r.x * scale + offset)
                .set("y", r.y * scale + offset)
                .set("width", r.width * scale)
                .set("height", r.height * scale)
                .set("fill", "white"),
        );
    }

    doc
}
