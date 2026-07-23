/*
 * Copyright (c) 2026 Thomas Prosser
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */

use crate::block_elements::{Column, ColumnCursor, LayoutBlock};
use crate::constants::{A4_PAGE_HEIGHT, A4_PAGE_WIDTH, MARGIN, RECEIPT_WIDTH, PP_FURTHER_INFO_LINE_SPACING, PP_FURTHER_INFO_SECTION_TOP, PP_FURTHER_INFO_TEXT_SIZE};
use crate::pdf::coords::LayoutY;
use crate::render::FontMetrics;
use crate::{truncate_to_width, Baseline, DrawOp, FontStyle, Mm, RenderContext};

/// Style Guide 3.5.5 "Further information section" / "Weitere
/// Informationen": up to two "Alternative procedures" lines at the bottom
/// of the payment part, 7pt, with the (short) procedure name in bold. The
/// receipt has no such section.
pub struct FurtherInformationBlock;

impl<T: FontMetrics> LayoutBlock<T> for FurtherInformationBlock {
    fn column(&self) -> Column {
        Column::Absolute
    }

    fn render(&self, ctx: &RenderContext<'_, T>, ops: &mut Vec<DrawOp>, _cursor: &mut ColumnCursor) {
        let x = RECEIPT_WIDTH + MARGIN;
        let available_width = A4_PAGE_WIDTH - x - MARGIN;
        let row_top = A4_PAGE_HEIGHT - PP_FURTHER_INFO_SECTION_TOP
            + ctx.metrics.ascender_mm(FontStyle::Bold, PP_FURTHER_INFO_TEXT_SIZE);

        for (i, scheme) in ctx.bill_data.alternative_schemes.iter().flatten().enumerate() {
            let y = row_top + Mm(i as f32) * PP_FURTHER_INFO_LINE_SPACING.to_mm();
            draw_alt_procedure_line(ops, ctx, scheme, x, y, available_width);
        }
    }
}

/// Draws one Alternative-procedure line: the part up to and including the
/// first separator ("eBill/" in "eBill/B/simon.muster@example.com") in
/// bold, the rest in regular weight, truncated with "…" if it would
/// otherwise overflow `available_width`.
fn draw_alt_procedure_line<T: FontMetrics>(
    ops: &mut Vec<DrawOp>,
    ctx: &RenderContext<'_, T>,
    scheme: &str,
    x: Mm,
    y: Mm,
    available_width: Mm,
) {
    let (name, rest) = match scheme.find('/') {
        Some(idx) => scheme.split_at(idx + 1),
        None => (scheme, ""),
    };

    let name_width = ctx.metrics.text_width_mm(name, FontStyle::Bold, PP_FURTHER_INFO_TEXT_SIZE);
    let rest = truncate_to_width(
        ctx.metrics,
        rest,
        FontStyle::Regular,
        PP_FURTHER_INFO_TEXT_SIZE,
        available_width - name_width,
    );

    ops.push(DrawOp::Text {
        text: name.to_string(),
        at: Baseline { x, y: LayoutY(y) },
        size: PP_FURTHER_INFO_TEXT_SIZE,
        bold: true,
    });

    if !rest.is_empty() {
        ops.push(DrawOp::Text {
            text: rest,
            at: Baseline { x: x + name_width, y: LayoutY(y) },
            size: PP_FURTHER_INFO_TEXT_SIZE,
            bold: false,
        });
    }
}
