/*
 * Copyright (c) 2026 Thomas Prosser
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */

use std::fs;
use std::path::Path;

use usvg::{Options, Tree};
use resvg::tiny_skia::Pixmap;

const QR_MM: f64 = 46.0;
const DPI: f64 = 300.0;
const MM_PER_INCH: f64 = 25.4;

/// Rasterize an `svg::Document` to PNG at exact physical size.
pub fn render_svg_to_png(
    doc: &svg::Document,
    out_path: impl AsRef<Path>,
) -> Result<(), Box<dyn std::error::Error>> {

    let svg_string = doc.to_string();

    let px_per_mm = DPI / MM_PER_INCH;
    let size_px = (QR_MM * px_per_mm).round() as u32;

    let opt = Options::default();
    let tree = Tree::from_str(&svg_string, &opt)?;

    let mut pixmap = Pixmap::new(size_px, size_px)
        .ok_or("Failed to allocate Pixmap")?;

    // `Transform::default()` keeps the SVG coordinates unchanged
    let pixmap_mut = &mut pixmap.as_mut();
    resvg::render(&tree, usvg::Transform::default(), pixmap_mut);

    fs::write(out_path, pixmap.encode_png()?)?;
    Ok(())
}
