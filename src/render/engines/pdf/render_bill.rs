/*
 * Copyright (c) 2026 Thomas Prosser
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */

use thiserror::Error;
use crate::{BillData, Language, PaymentPartLayout, ReceiptLayout};
use crate::render::layout::bill_layout::LayoutStrategy;
use crate::pdf::{execute_bill_ops, PDFBuilder};
use crate::qr_bill::qr_code;

#[cfg(feature = "pdf-debug")]
use crate::render::debug_overlay::draw_debug_overlay;

#[derive(Debug, PartialEq, Error)]
pub enum RenderError {
    #[error("Failed to create PDF builder")]
    BuilderCreationError,
    #[error("Failed to setup PDF")]
    SetupPdfError,
    #[error("Failed to compute font metrics")]
    FontMetricsError,
    #[error("Failed to execute bill operations")]
    BillOpsExecutionError,
    #[error("Failed to generate QR code")]
    QrCodeGenerationError,
}

/// Renders a bill to a set of PDF operations.
///
/// # Arguments:
/// * `bill` - Data of the bill
/// * `language` - Language of the bill
///
/// # Returns
///
/// A vector of raw bytes representing the PDF content.
///
pub fn render_bill_to_pdf(bill: &BillData, language: Language) -> Result<Vec<u8>, RenderError>  {

     // --- 1. Create PDF builder ---
    let mut builder = PDFBuilder::new();
    builder.setup_pdf()?;

    // --- 2. Layout: Payment Part --
    PaymentPartLayout::new()
        .render(bill, language, &builder.fonts, &mut builder.ops);

    // --- 3. Layout: Receipt --
    ReceiptLayout::new()
        .render(bill, language, &builder.fonts, &mut builder.ops);

    execute_bill_ops(
        &mut builder.content,
        &builder.fonts,
        std::mem::take(&mut builder.ops),
        qr_code(&bill).ok().as_ref(),
    );

    /// Draw a visual overlay grid 5x5 mm and positions and sizes of blocks.
    #[cfg(feature = "pdf-debug")]
    
    draw_debug_overlay(&mut builder);
    // --- 6. Attach content stream ---
    builder
        .pdf
        .stream(builder.content_id, &builder.content.finish());

    // --- 7. Write PDF -
    Ok(builder.pdf.finish())
}
