/*
 * Copyright (c) 2026 Thomas Prosser
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */
use pdf_writer::Content;
use qrcodegen::QrCode;
use svg::Document;
use svg::node::element::{Rectangle, Polygon as SvgPolygon};
use crate::{CROSS_POLYGONS, CROSS_RECTS, PT_PER_MM};

const PX_PER_MM: f64 = 3.543307;
const MODULE_MM: f64 = 0.4;
const QUIET_ZONE_MODULES: usize = 4;
const QR_MM: f64 = 46.0;
const CROSS_MM: f64 = 7.0;

// Spec is a little misleading it talks about a quiet
// zone of 4 modules (1.6 mm) but extends it to 5 mm.
const QUIET_MM: f64 = 0f64;
const CROSS_VIEWBOX: (f64, f64) = (19.8, 19.8);


pub fn render_qr_pdf(content: &mut Content, qr: &QrCode, x_off: f32, y_off: f32) {
        let modules = qr.size();
        let drawable_pt = (QR_MM - 2.0 * QUIET_MM) as f32 * PT_PER_MM;
        let module_pt = drawable_pt / modules as f32;
        let quiet_pt = QUIET_MM as f32 * PT_PER_MM;

        content.save_state();
        content.set_fill_rgb(0.0, 0.0, 0.0); // Black modules

        for row in 0..modules {
            for col in 0..modules {
                if qr.get_module(col, row) {
                    let x = x_off + quiet_pt + (col as f32 * module_pt);
                    // Remember PDF is Y-up!
                    let y = y_off + quiet_pt + ((modules - 1 - row) as f32 * module_pt);

                    content.rect(x, y, module_pt, module_pt);
                }
            }
        }
        content.fill_nonzero(); // Draw all modules at once for efficiency

        content.save_state();
        content.set_fill_rgb(1.0, 1.0, 1.0);
        // Center of 46mm is 23mm. 7x7mm box starts at 23 - 3.5 = 19.5mm
        let cross_box_offset = 19.5 * PT_PER_MM as f32;
        content.rect(x_off + cross_box_offset, y_off + cross_box_offset, 7.0 * PT_PER_MM as f32, 7.0 * PT_PER_MM as f32);
        content.fill_nonzero();
        content.restore_state();

        // Draw the Swiss Cross (Vector)
        draw_swiss_cross_vector(content, (x_off + (23.0 * PT_PER_MM)) as f64, (y_off + (23.0 * PT_PER_MM)) as f64);
}

pub fn draw_swiss_cross_vector(content: &mut Content, x_center: f64, y_center: f64) {
    let cross_size_pt = 7.0 * PT_PER_MM as f64;
    let scale = cross_size_pt / CROSS_VIEWBOX.0;

    // Bottom-left corner of the 7x7mm cross area
    let x_origin = x_center - (cross_size_pt / 2.0);
    let y_origin = y_center - (cross_size_pt / 2.0);

    content.save_state();

    // 1. Black Background (The Polygon with rounded-ish corners)
    content.set_fill_rgb(0.0, 0.0, 0.0);
    for poly in CROSS_POLYGONS {
        if let Some((first_x, first_y)) = poly.points.first() {
            content.move_to((x_origin + (first_x * scale)) as f32, (y_origin + (cross_size_pt - (first_y * scale))) as f32);
            for (px, py) in poly.points.iter().skip(1) {
                content.line_to((x_origin + (px * scale)) as f32, (y_origin + (cross_size_pt - (py * scale))) as f32);
            }
            content.close_path();
        }
    }
    content.fill_nonzero();

    // 2. White Cross Arms (The Rects)
    content.set_fill_rgb(1.0, 1.0, 1.0);
    for r in CROSS_RECTS {
        let rx = x_origin + (r.x * scale);
        // Flip Y: SVG Y is distance from top, PDF Y is distance from bottom
        let ry = y_origin + (cross_size_pt - ((r.y + r.height) * scale));
        content.rect(rx as f32, ry as f32, r.width as f32 * scale as f32, r.height as f32 * scale as f32);
    }
    content.fill_nonzero();

    content.restore_state();
}



/// Render a Swiss QR Code SVG (modules only).
/// The Swiss cross area is cleared here.
/// The official SVG cross is rendered later on top.
pub fn render_qr_svg(qr: QrCode) -> Document {
    let modules = qr.size();
    let drawable_mm = QR_MM - 2.0 * QUIET_MM;
    let module_mm = drawable_mm / modules as f64;

    let mut doc = Document::new()
        .set("width", format!("{}mm", QR_MM as usize))
        .set("height", format!("{}mm", QR_MM as usize))
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
