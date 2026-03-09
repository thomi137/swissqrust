/*
 * Copyright (c) 2026 Thomas Prosser 
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */
use crate::support::traits::SwissQRFormatter;
use crate::block_elements::{Column, ColumnCursor, LayoutBlock};
use crate::{draw_corner_marks, draw_label, draw_single_line, draw_text_lines, label, DrawOp, Mm, QRBillLayoutRect, ReferenceType, RenderContext, SlipPart, CORNER_MARKS_PAYABLE_BY_POLYLINES, CORNER_MARKS_PAYABLE_BY_VIEWBOX};
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

    fn render(&self, ctx: &RenderContext<'_, T>, ops: &mut Vec<DrawOp>, cursor: &mut ColumnCursor) {
        // Account / Payable to
        draw_label(ops, label!(AccountPayableTo, ctx.language), cursor.x, &mut cursor.y, ctx.label_size);
        cursor.advance(ctx.text_ascender);
        draw_single_line(ops, &ctx.bill_data.iban.format_iban(), cursor.x, &mut cursor.y, ctx.text_size);
        cursor.advance(ctx.text_ascender);
        draw_text_lines(ops, &ctx.bill_data.creditor_address.to_lines().all_but_last(), cursor.x, &mut cursor.y, ctx.text_size, ctx.line_spacing);
        cursor.advance(ctx.line_spacing);

        // Reference
        match &ctx.bill_data.reference_type {
            ReferenceType::QrRef(reference) => {
                draw_label(ops, label!(Reference, ctx.language), cursor.x, &mut cursor.y, ctx.label_size);
                cursor.advance(ctx.text_ascender);
                draw_single_line(ops, &reference.format_qr_reference(), cursor.x, &mut cursor.y, ctx.text_size);
                cursor.advance(ctx.line_spacing + ctx.extra_spacing);
            },
            ReferenceType::Creditor(reference) => {
                draw_label(ops, label!(Reference, ctx.language), cursor.x, &mut cursor.y, ctx.label_size);
                cursor.advance(ctx.text_ascender);
                draw_single_line(ops, &reference.format_scor_reference(), cursor.x, &mut cursor.y, ctx.text_size);
                cursor.advance(ctx.line_spacing + ctx.extra_spacing);
            },
            _ => {},
        }

        // Unstructured message
        if self.part == SlipPart::PaymentPart && let Some(unstructured_message) = &ctx.bill_data.unstructured_message {
                draw_label(ops, label!(AdditionalInformation, ctx.language), cursor.x, &mut cursor.y, ctx.label_size);
                cursor.advance(ctx.text_ascender);
                draw_single_line(ops, unstructured_message, cursor.x, &mut cursor.y, ctx.text_size);
                cursor.advance(ctx.line_spacing + ctx.extra_spacing );
            } else { cursor.advance(ctx.line_spacing + ctx.text_ascender + ctx.extra_spacing); }

        // Payable by
        if let Some(debtor) = &ctx.bill_data.debtor_address {
            draw_label(ops, label!(PayableBy, ctx.language), cursor.x, &mut cursor.y, ctx.label_size);
            cursor.advance(ctx.text_ascender);
            draw_text_lines(ops, &debtor.to_lines().all_but_last(), cursor.x, &mut cursor.y, ctx.text_size, ctx.line_spacing);
            cursor.advance(ctx.extra_spacing)
        } else {
            draw_label(ops, label!(PayableByNameAddress, ctx.language), cursor.x, &mut cursor.y, ctx.label_size);
            cursor.advance(ctx.text_ascender);
            draw_corner_marks(ops, QRBillLayoutRect {x: cursor.x, y: cursor.y, width: self.payable_box_width, height: self.payable_box_height  }, CORNER_MARKS_PAYABLE_BY_VIEWBOX, CORNER_MARKS_PAYABLE_BY_POLYLINES);
            cursor.advance(self.payable_box_height + ctx.extra_spacing);
        }

    }
}
