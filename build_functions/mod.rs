/*
 * Copyright (c) 2026 Thomas Prosser
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */
use crate::build_functions::parse_svg::generate_svg_constants;

mod build_countries;
mod parse_svg;

pub fn run() {
    println!("cargo:warning=build_functions.rs is running");
    build_countries::generate();
    generate_svg_constants("assets/svg/CH-Kreuz_7mm.svg", "CROSS", "src/generated/cross.rs");
    generate_svg_constants("assets/svg/Corner_marks_Amount.svg", "CORNER_MARKS_AMOUNT", "src/generated/corner_marks_amount.rs");
    generate_svg_constants("assets/svg/Corner_marks_Payable_by.svg", "CORNER_MARKS_PAYABLE_BY", "src/generated/corner_marks_payable_by.rs");
    generate_svg_constants("assets/svg/Scissors_symbol.svg", "SCISSORS", "src/generated/scissors.rs");
}