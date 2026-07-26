/*
 * Copyright (c) 2026 Thomas Prosser
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */
use svg::node::element::Line as SvgLine;
use svg::Document;

/// Style Guide p.7: on the printed/PDF output, the QR-bill is separated from
/// the rest of the document by a dashed perforation line
/// Here we only keep a plain divider between receipt and
/// payment part, to visually separate the two without implying a real
/// perforation/cut mark exists on screen.
pub fn add_perforation_marks(mut doc: Document) -> Document {
    let vertical = SvgLine::new()
        .set("x1", 62)
        .set("y1", 0)
        .set("x2", 62)
        .set("y2", 105)
        .set("stroke", "black")
        .set("stroke-width", 0.26)
        .set("stroke-dasharray", "1.06,1.06");
    doc = doc.add(vertical);

    doc
}
