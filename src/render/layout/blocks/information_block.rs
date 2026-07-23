/*
 * Copyright (c) 2026 Thomas Prosser
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */
use crate::support::traits::SwissQRFormatter;
use crate::block_elements::{Column, ColumnCursor, LayoutBlock};
use crate::constants::{A4_PAGE_WIDTH, MARGIN};
use crate::{draw_corner_marks, draw_label, draw_single_line, draw_text_lines, label, truncate_to_width, DrawOp, FontStyle, Mm, QRBillLayoutRect, ReferenceType, RenderContext, SlipPart, CORNER_MARKS_PAYABLE_BY_POLYLINES, CORNER_MARKS_PAYABLE_BY_VIEWBOX};
use crate::render::FontMetrics;
use crate::support::traits::SliceExt;

pub struct InformationBlock{
    pub part: SlipPart,
    pub payable_box_width: Mm,
    pub payable_box_height: Mm,
    pub offset: Mm
}
impl <T: FontMetrics> LayoutBlock<T> for InformationBlock {
    fn column(&self) -> Column {
        Column::Right
    }

    /// Style Guide p.15: "Da es keine fixen Platzierungen der einzelnen
    /// Textblöcke gibt, rückt bei fehlenden Textblöcken jeweils alles nach
    /// oben" - there are no fixed positions for these fields; a block with
    /// no content (no reference, no message, ...) is omitted entirely (no
    /// heading, no reserved space) and everything below it moves up. Blocks
    /// that are present are separated by exactly one blank line (Zeilenabstand
    /// 9pt on the receipt / 11pt on the payment part, via `ctx.line_spacing`).
    fn render(&self, ctx: &RenderContext<'_, T>, ops: &mut Vec<DrawOp>, cursor: &mut ColumnCursor) {
        // Account / Payable to
        draw_label(ops, label!(AccountPayableTo, ctx.language), cursor.x, &mut cursor.y, ctx.label_size);
        cursor.advance(ctx.line_spacing);
        draw_single_line(ops, &ctx.bill_data.iban.format_iban(), cursor.x, &mut cursor.y, ctx.text_size);
        cursor.advance(ctx.line_spacing);
        draw_text_lines(ops, &ctx.bill_data.creditor_address.to_lines().all_but_last(), cursor.x, &mut cursor.y, ctx.text_size, ctx.line_spacing);
        cursor.advance(ctx.line_spacing); // blank line before the next block

        // Reference - omitted entirely (no heading) when the bill has none.
        let reference_value = match &ctx.bill_data.reference_type {
            ReferenceType::QrRef(reference) => Some(reference.format_qr_reference()),
            ReferenceType::Creditor(reference) => Some(reference.format_scor_reference()),
            ReferenceType::NoRef => None,
        };
        if let Some(reference_value) = reference_value {
            draw_label(ops, label!(Reference, ctx.language), cursor.x, &mut cursor.y, ctx.label_size);
            cursor.advance(ctx.line_spacing);
            draw_single_line(ops, &reference_value, cursor.x, &mut cursor.y, ctx.text_size);
            cursor.advance(ctx.line_spacing); // move past the value line
            cursor.advance(ctx.line_spacing); // blank line before the next block
        }

        // Additional information - payment part only; omitted on the
        // receipt (which has no such field) and when there is nothing to
        // show. Style Guide 3.5.4: Ustrd and StrdBkgInf (which may hold
        // Swico billing information) are shown together here, one per line
        // if both are present, each truncated with "…" if it overflows the
        // column width.
        if self.part == SlipPart::PaymentPart {
            let lines: Vec<&str> = [
                ctx.bill_data.unstructured_message.as_deref(),
                ctx.bill_data.bill_information.as_deref(),
            ]
            .into_iter()
            .flatten()
            .collect();

            if !lines.is_empty() {
                let available_width = A4_PAGE_WIDTH - cursor.x - MARGIN;
                draw_label(ops, label!(AdditionalInformation, ctx.language), cursor.x, &mut cursor.y, ctx.label_size);
                cursor.advance(ctx.line_spacing);
                for line in lines {
                    let line = truncate_to_width(ctx.metrics, line, FontStyle::Regular, ctx.text_size, available_width);
                    draw_single_line(ops, &line, cursor.x, &mut cursor.y, ctx.text_size);
                    cursor.advance(ctx.line_spacing); // move past the value line
                }
                cursor.advance(ctx.line_spacing); // blank line before the next block
            }
        }

        // Payable by
        if let Some(debtor) = &ctx.bill_data.debtor_address {
            draw_label(ops, label!(PayableBy, ctx.language), cursor.x, &mut cursor.y, ctx.label_size);
            cursor.advance(ctx.line_spacing);
            draw_text_lines(ops, &debtor.to_lines().all_but_last(), cursor.x, &mut cursor.y, ctx.text_size, ctx.line_spacing);
        } else {
            draw_label(ops, label!(PayableByNameAddress, ctx.language), cursor.x, &mut cursor.y, ctx.label_size);
            cursor.advance(ctx.line_spacing);
            draw_corner_marks(ops, QRBillLayoutRect {x: cursor.x, y: cursor.y, width: self.payable_box_width, height: self.payable_box_height  }, CORNER_MARKS_PAYABLE_BY_VIEWBOX, CORNER_MARKS_PAYABLE_BY_POLYLINES);
            cursor.advance(self.payable_box_height);
        }
    }
}
