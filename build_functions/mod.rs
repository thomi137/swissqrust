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
    generate_svg_constants("assets/CH-Kreuz_7mm.svg", "CROSS", "src/generated/cross.rs");
}