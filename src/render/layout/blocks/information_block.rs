/*
 * Copyright (c) 2026 Thomas Prosser 
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */
use crate::support::traits::SwissQRFormatter;
use crate::bill_layout::{BillLayout };
use crate::block_elements::{Column, ColumnCursor, LayoutBlock};
use crate::{draw_corner_marks, draw_label, draw_single_line, draw_text_lines, label, DrawOp, Mm, QRBillLayoutRect, ReferenceType, SlipPart, CORNER_MARKS_PAYABLE_BY_POLYLINES, CORNER_MARKS_PAYABLE_BY_VIEWBOX};
use crate::support::traits::SliceExt;

pub struct InformationBlock{
    pub part: SlipPart,
    pub payable_box_width: Mm,
    pub payable_box_height: Mm,
    pub offset: Mm
}
impl LayoutBlock for InformationBlock {
    fn column(&self) -> Column {
        Column::Right
    }

    fn render(&self, layout: &mut BillLayout, ops: &mut Vec<DrawOp>, cursor: &mut ColumnCursor) {

        // Account / Payable to
        draw_label(ops, label!(AccountPayableTo, layout.language), cursor.x, &mut cursor.y, layout.label_font_size);
        cursor.advance(layout.text_ascender);
        draw_single_line(ops, &layout.bill_data.iban.format_iban(), cursor.x, &mut cursor.y, layout.text_font_size);
        cursor.advance(layout.text_ascender);
        draw_text_lines(ops, &layout.bill_data.creditor_address.to_lines().all_but_last(), cursor.x, &mut cursor.y, layout.text_font_size, layout.line_spacing);
        cursor.advance(layout.line_spacing);

        // Reference
        match &layout.bill_data.reference_type {
            ReferenceType::QrRef(reference) => {
                draw_label(ops, label!(Reference, layout.language), cursor.x, &mut cursor.y, layout.label_font_size);
                cursor.advance(layout.text_ascender);
                draw_single_line(ops, &reference.format_qr_reference(), cursor.x, &mut cursor.y, layout.text_font_size);
                cursor.advance(layout.line_spacing + layout.extra_spacing);
            },
            ReferenceType::Creditor(reference) => {
                draw_label(ops, label!(Reference, layout.language), cursor.x, &mut cursor.y, layout.label_font_size);
                cursor.advance(layout.text_ascender);
                draw_single_line(ops, &reference.format_scor_reference(), cursor.x, &mut cursor.y, layout.text_font_size);
                cursor.advance(layout.line_spacing + layout.extra_spacing);
            },
            _ => {},
        }

        // Unstructured message
        if self.part == SlipPart::PaymentPart
            && let Some(unstructured_message) = &layout.bill_data.unstructured_message {
                draw_label(ops, label!(AdditionalInformation, layout.language), cursor.x, &mut cursor.y, layout.label_font_size);
                cursor.advance(layout.text_ascender);
                draw_single_line(ops, unstructured_message, cursor.x, &mut cursor.y, layout.text_font_size);
                cursor.advance(layout.line_spacing + layout.extra_spacing );
            }


        // Payable by
        if let Some(debtor) = &layout.bill_data.debtor_address {
            draw_label(ops, label!(PayableBy, layout.language), cursor.x, &mut cursor.y, layout.label_font_size);
            cursor.advance(layout.text_ascender);
            draw_text_lines(ops, &debtor.to_lines().all_but_last(), cursor.x, &mut cursor.y, layout.text_font_size, layout.line_spacing);
            cursor.advance(layout.extra_spacing);
        } else {
            draw_label(ops, label!(PayableByNameAddress, layout.language), cursor.x, &mut cursor.y, layout.label_font_size);
            cursor.advance(layout.text_ascender);
            draw_corner_marks(ops, QRBillLayoutRect {x: cursor.x, y: cursor.y, width: self.payable_box_width, height: self.payable_box_height  }, CORNER_MARKS_PAYABLE_BY_VIEWBOX, CORNER_MARKS_PAYABLE_BY_POLYLINES);
            cursor.advance(self.payable_box_height + layout.extra_spacing);
        }

        if let Some(info_lines) = &layout.bill_data.additional_information
            && !info_lines.is_empty() {
                draw_label(ops, label!(AdditionalInformation, layout.language), cursor.x, &mut cursor.y, layout.label_font_size);
                cursor.advance(layout.text_ascender);
                draw_single_line(ops, info_lines, cursor.x, &mut cursor.y, layout.text_font_size);
                cursor.advance(layout.extra_spacing);
            }

    }
}