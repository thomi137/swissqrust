/*
 * Copyright (c) 2026 Thomas Prosser
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */
mod build_countries;
mod parse_svg;

use crate::build_functions::parse_svg::generate_svg_constants;

const SVG_ASSETS: &[(&str, &str, &str)] = &[
    ("assets/svg/CH-Kreuz_7mm.svg", "CROSS", "src/generated/cross.rs"),
    ("assets/svg/Corner_marks_Amount.svg", "CORNER_MARKS_AMOUNT", "src/generated/corner_marks_amount.rs"),
    ("assets/svg/Corner_marks_Payable_by.svg", "CORNER_MARKS_PAYABLE_BY", "src/generated/corner_marks_payable_by.rs"),
    ("assets/svg/Scissors_symbol.svg", "SCISSORS", "src/generated/scissors.rs"),
];

pub fn run() {
    println!("cargo:warning=build_functions.rs is running");
    build_countries::generate();
    
    for (svg_path, const_name, output_path) in SVG_ASSETS {
        generate_svg_constants(svg_path, const_name, output_path);
    }
}