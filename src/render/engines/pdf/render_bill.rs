/*
 * Copyright (c) 2026 Thomas Prosser
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */

use std::path::Path;
use anyhow::Result;
use crate::{execute_bill_ops, BillData, DrawOp, FontStyle, Language, Mm, PDFBuilder, PaymentPartLayout, QRBillLayoutRect, ReceiptLayout, A4_PAGE_HEIGHT, PP_LABEL_PREF_FONT_SIZE, PP_TEXT_PREF_FONT_SIZE, RC_LABEL_PREF_FONT_SIZE, RC_TEXT_PREF_FONT_SIZE};
use crate::pdf::coords::layout_y_to_pdf_y;

pub fn render_bill_to_pdf(bill: &BillData, path: &Path,) -> Result<()> {

    // --- 1. Create PDF builder ---
    let mut builder = PDFBuilder::new();
    builder.setup_pdf()?;

    // --- 2. Compute font metrics (single source of truth) ---
    let pp_label_ascender =
        builder.fonts.ascender_mm(FontStyle::Bold, PP_LABEL_PREF_FONT_SIZE);
    let pp_text_ascender =
        builder.fonts.ascender_mm(FontStyle::Regular, PP_TEXT_PREF_FONT_SIZE);

    let rc_label_ascender =
        builder.fonts.ascender_mm(FontStyle::Bold, RC_LABEL_PREF_FONT_SIZE);
    let rc_text_ascender =
        builder.fonts.ascender_mm(FontStyle::Regular, RC_TEXT_PREF_FONT_SIZE);

    let mut payment_part = PaymentPartLayout::new(
        bill,
        Language::De,
        PP_LABEL_PREF_FONT_SIZE,
        PP_TEXT_PREF_FONT_SIZE,
        Mm(3.5),
        Mm(2.0),
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
        Mm(3.5),
        Mm(2.0),
        rc_label_ascender,
        rc_text_ascender,
    );
    receipt.render(&mut builder.ops, &builder.fonts);

    // --- 5. Render DrawOps into PDF content ---

    for op in &mut builder.ops {
        match op {
            DrawOp::Text { at, .. } => {
                at.y = layout_y_to_pdf_y(at.y, A4_PAGE_HEIGHT);
            }
            DrawOp::Line { from, to, .. } => {
                from.1 = layout_y_to_pdf_y(from.1, A4_PAGE_HEIGHT);
                to.1   = layout_y_to_pdf_y(to.1, A4_PAGE_HEIGHT);
            }
            DrawOp::Box { rect } => {
                rect.y = layout_y_to_pdf_y(rect.y, A4_PAGE_HEIGHT);
            }
            DrawOp::QrCodeSpace { at, .. } => {
                at.y = layout_y_to_pdf_y(at.y, A4_PAGE_HEIGHT);
            }
        }
    }

    execute_bill_ops(
        &mut builder.content,
        &builder.fonts,
        std::mem::take(&mut builder.ops),
        bill.qr_code.as_ref(),
    );
    // --- 6. Attach content stream ---
    builder
        .pdf
        .stream(builder.content_id, &builder.content.finish());

    // --- 7. Write PDF ---
    std::fs::write(path, builder.pdf.finish())?;

    Ok(())
}