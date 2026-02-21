/*
 * Copyright (c) 2026 Thomas Prosser
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */

use std::fs;
use roxmltree::Document;

/// Generate Rust constants from an SVG file.
/// This function is intended to be called from build.rs.
///
/// The generated file is COMPLETE and SELF-CONTAINED.
pub fn generate_svg_constants(
    svg_file: &str,
    const_prefix: &str,
    out_file: &str,
) {
    let svg_source =
        fs::read_to_string(svg_file).expect("Failed to read SVG file");

    let doc =
        Document::parse(&svg_source).expect("Invalid SVG document");

    let svg = doc
        .descendants()
        .find(|n| n.has_tag_name("svg"))
        .expect("SVG missing <svg> root");

    // ---------------- Parse viewBox ----------------

    let viewbox_vals: Vec<f64> = svg
        .attribute("viewBox")
        .expect("SVG missing viewBox")
        .split_whitespace()
        .map(|v| v.parse::<f64>().expect("Invalid viewBox value"))
        .collect();

    let viewbox = (viewbox_vals[2], viewbox_vals[3]);

    // ---------------- Parse rects ----------------

    let rects: Vec<(f64, f64, f64, f64)> = svg
        .descendants()
        .filter(|n| n.has_tag_name("rect"))
        .map(|r| {
            (
                r.attribute("x").unwrap_or("0").parse().unwrap(),
                r.attribute("y").unwrap_or("0").parse().unwrap(),
                r.attribute("width").unwrap().parse().unwrap(),
                r.attribute("height").unwrap().parse().unwrap(),
            )
        })
        .collect();

    // ---------------- Parse polygons ----------------

    let polygons: Vec<Vec<(f64, f64)>> = svg
        .descendants()
        .filter(|n| n.has_tag_name("polygon"))
        .map(|p| {
            p.attribute("points")
                .expect("Polygon missing points")
                .split_whitespace()
                .map(|pair| {
                    let mut it = pair.split(',');
                    (
                        it.next().unwrap().parse::<f64>().unwrap(),
                        it.next().unwrap().parse::<f64>().unwrap(),
                    )
                })
                .collect::<Vec<(f64, f64)>>()
        })
        .collect();

    // ---------------- Parse Polylines ----------

    let polylines: Vec<Vec<(f64, f64)>> = svg
        .descendants()
        .filter(|n| n.has_tag_name("polyline"))
        .map(|p| {
            p.attribute("points")
                .expect("Polyline missing points")
                .split_whitespace()
                .map(|pair| {
                    let mut it = pair.split(',');
                    (
                        it.next().unwrap().parse::<f64>().unwrap(),
                        it.next().unwrap().parse::<f64>().unwrap(),
                    )
                })
                .collect::<Vec<(f64, f64)>>()
        })
        .collect();




    // ---------------- Emit Rust ----------------

    let name = const_prefix.to_uppercase();
    let mut out = String::new();

    // ---- File header / warning ----
    out.push_str(
        "/*\n\
         * ------------------------------------------------------------------\n\
         *  ⚠️  THIS FILE IS AUTO-GENERATED — DO NOT EDIT MANUALLY\n\
         *\n\
         *  Changes to this file will be LOST the next time build.rs runs.\n\
         *\n\
         *  Source SVG: generated at build time\n\
         * ------------------------------------------------------------------\n\
         */\n\n",
    );

    // ---- Runtime types (guaranteed to exist here) ----
    out.push_str(
        "use crate::generated::shapes::*;\n\n"
    );

    // ---- ViewBox ----
    out.push_str(&format!(
        "pub const {name}_VIEWBOX: (f64, f64) = ({}f64, {}f64);\n\n",
        viewbox.0, viewbox.1
    ));

    // ---- Rects ----
    out.push_str(&format!("pub const {name}_RECTS: &[Rect] = &[\n"));
    for (x, y, w, h) in rects {
        out.push_str(&format!(
            "    Rect {{ x: {}f64, y: {}f64, width: {}f64, height: {}f64 }},\n",
            x, y, w, h
        ));
    }
    out.push_str("];\n\n");

    // ---- Polygons ----
    out.push_str(&format!(
        "pub const {name}_POLYGONS: &[Polygon] = &[\n"
    ));
    for poly in polygons {
        out.push_str("    Polygon { points: &[\n");
        for (x, y) in poly {
            out.push_str(&format!("        ({}f64, {}f64),\n", x, y));
        }
        out.push_str("    ]},\n");
    }
    out.push_str("];\n");

    // Emit Polylines (they function like Polygons but are open-ended)
    out.push_str(&format!("pub const {name}_POLYLINES: &[Polygon] = &[\n"));
    for poly in polylines {
        out.push_str("    Polygon { points: &[\n");
        for (x, y) in poly {
            out.push_str(&format!("        ({}f64, {}f64),\n", x, y));
        }
        out.push_str("    ]},\n");
    }
    out.push_str("];\n");


    fs::write(out_file, out)
        .expect("Failed to write generated Rust file");
}
