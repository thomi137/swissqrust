/*
 * Copyright (c) 2026 Thomas Prosser
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */
mod build_countries;
mod parse_svg;

use std::env;
use std::path::PathBuf;

use crate::build_functions::parse_svg::generate_svg_constants;

const SVG_ASSETS: &[(&str, &str, &str)] = &[
    ("assets/svg/CH-Kreuz_7mm.svg", "CROSS", "cross.rs"),
    ("assets/svg/Corner_marks_Amount.svg", "CORNER_MARKS_AMOUNT", "corner_marks_amount.rs"),
    ("assets/svg/Corner_marks_Payable_by.svg", "CORNER_MARKS_PAYABLE_BY", "corner_marks_payable_by.rs"),
];

pub fn run() {
    println!("cargo:warning=build_functions.rs is running");
    build_countries::generate();

    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");

    for (svg_path, const_name, output_file) in SVG_ASSETS {
        let dest_path = PathBuf::from(&out_dir).join(output_file);
        generate_svg_constants(svg_path, const_name, dest_path.to_str().expect("OUT_DIR is not valid UTF-8"));
    }
}