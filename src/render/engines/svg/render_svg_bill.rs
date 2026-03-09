/*
 * Copyright (c) 2026 Thomas Prosser
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */
use crate::{BillData, Language, PaymentPartLayout, ReceiptLayout};
use crate::pdf::render_bill::RenderError;
use crate::qr_bill::qr_code;
use crate::svg::{execute_bill_ops_svg, SvgFontLibrary};
use crate::render::layout::bill_layout::LayoutStrategy;

pub fn render_bill_to_svg(bill: &BillData, language: Language) -> Result<String, RenderError> {

    // 1. Initialise SVG-specific metrics (Lightweight for WASM)
    let fonts = SvgFontLibrary::new();

    // 2. Generate the "Bytecode" (DrawOps) using your shared Layout engines
    let mut ops = Vec::new();

    // Render Receipt
    ReceiptLayout::new()
        .render(bill, language, &fonts, &mut ops);

    // Render Payment Part
    PaymentPartLayout::new()
        .render(bill, language, &fonts, &mut ops);

    let svg = execute_bill_ops_svg(
        ops,
        qr_code(bill).ok().as_ref(),
    )?;

    Ok(svg)

}

