/*
 * Copyright (c) 2026 Thomas Prosser
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */

use std::path::Path;
use anyhow::Result;
use crate::{execute_bill_ops, BillData, DrawOp, FontStyle, Language, Mm, PDFBuilder, PaymentPartLayout, Pt, QRBillLayoutRect, ReceiptLayout, A4_PAGE_HEIGHT, FONT_SIZE_TITLE, PP_LABEL_PREF_FONT_SIZE, PP_TEXT_PREF_FONT_SIZE, RC_LABEL_PREF_FONT_SIZE, RC_TEXT_PREF_FONT_SIZE};

#[cfg(feature = "pdf-debug")]
use crate::render::debug_overlay::draw_debug_overlay;

pub fn render_bill_to_pdf(bill: &BillData, path: &Path,) -> Result<()> {

    // --- 1. Create PDF builder ---
    let mut builder = PDFBuilder::new();
    builder.setup_pdf()?;

    // --- 2. Compute font metrics (single source of truth) ---
    let pp_title_ascender =
        builder.fonts.ascender_mm(FontStyle::Bold, FONT_SIZE_TITLE);
    let pp_label_ascender =
        builder.fonts.ascender_mm(FontStyle::Bold, PP_LABEL_PREF_FONT_SIZE);
    let pp_text_ascender =
        builder.fonts.ascender_mm(FontStyle::Regular, PP_TEXT_PREF_FONT_SIZE);
    let pp_line_spacing: Mm = (PP_TEXT_PREF_FONT_SIZE) .to_mm();

    let rc_title_ascender =
    builder.fonts.ascender_mm(FontStyle::Bold, FONT_SIZE_TITLE);
    let rc_label_ascender =
        builder.fonts.ascender_mm(FontStyle::Bold, RC_LABEL_PREF_FONT_SIZE);
    let rc_text_ascender =
        builder.fonts.ascender_mm(FontStyle::Regular, RC_TEXT_PREF_FONT_SIZE);
    let rc_line_spacing: Mm = (RC_TEXT_PREF_FONT_SIZE ).to_mm();

    let mut payment_part = PaymentPartLayout::new(
        bill,
        Language::De,
        PP_LABEL_PREF_FONT_SIZE,
        PP_TEXT_PREF_FONT_SIZE,
        pp_line_spacing,
        Mm(2.0),
        pp_title_ascender,
        pp_label_ascender,
        pp_text_ascender,
    );
    payment_part.render(&mut builder.ops);

    // --- 4. Layout: Receipt ---
    let mut receipt = ReceiptLayout::new(
        bill,
        Language::De,
        RC_LABEL_PREF_FONT_SIZE,
        RC_TEXT_PREF_FONT_SIZE,
        rc_line_spacing,
        Mm(2.0),
        rc_title_ascender,
        rc_label_ascender,
        rc_text_ascender,
    );
    receipt.render(&mut builder.ops, &builder.fonts);

    execute_bill_ops(
        &mut builder.content,
        &builder.fonts,
        std::mem::take(&mut builder.ops),
        bill.qr_code.as_ref(),
    );

    // --- 5.1 Draw debug overlay ---
    // Good for debugging
    // requires pdf-debug feature
    #[cfg(feature = "pdf-debug")]
    draw_debug_overlay(&mut builder);

    // --- 6. Attach content stream ---
    builder
        .pdf
        .stream(builder.content_id, &builder.content.finish());


    // --- 7. Write PDF ---
    std::fs::write(path, builder.pdf.finish())?;

    Ok(())
}