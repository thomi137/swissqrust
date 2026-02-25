/*
 * Copyright (c) 2026 Thomas Prosser 
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */
use crate::support::traits::SwissQRFormatter;
use crate::layout::bill_layout::{BillLayout };
use crate::layout::block::{Column, ColumnCursor, LayoutBlock};
use crate::{draw_corner_marks, draw_label, draw_single_line, draw_text_lines, label, DrawOp, Mm, QRBillLayoutRect, ReferenceType, CORNER_MARKS_PAYABLE_BY_POLYLINES, CORNER_MARKS_PAYABLE_BY_VIEWBOX};
use crate::support::traits::SliceExt;

pub struct InformationBlock{
    pub payable_box_width: Mm,
    pub payable_box_height: Mm,
    pub offset: Mm
}
impl LayoutBlock for InformationBlock {
    fn column(&self) -> Column {
        Column::Right
    }

    fn render(&self, layout: &mut BillLayout, ops: &mut Vec<DrawOp>, cursor: &mut ColumnCursor) {
        let x = cursor.x + self.offset;
        let mut y = cursor.y;

        // Account / Payable to
        draw_label(ops, label!(AccountPayableTo, layout.language), x, &mut y, layout.label_font_size, layout.line_spacing);
        draw_single_line(ops, &*layout.bill_data.iban.format_iban(), x, &mut y, layout.text_font_size, layout.line_spacing, Mm(0.0));
        draw_text_lines(ops, &layout.bill_data.creditor_address.to_lines().all_but_first(), x, &mut y, layout.text_font_size, layout.line_spacing, layout.extra_spacing);

        // Reference
        match &layout.bill_data.reference_type {
            ReferenceType::QrRef(reference) => {
                draw_label(ops, label!(Reference, layout.language), x, &mut y, layout.label_font_size, layout.line_spacing);
                draw_single_line(ops, &reference.format_qr_reference(), x, &mut y, layout.text_font_size, layout.line_spacing, layout.extra_spacing);
            }
            ReferenceType::Creditor(reference) => {
                draw_label(ops, label!(Reference, layout.language), x, &mut y, layout.label_font_size, layout.line_spacing);
                draw_single_line(ops, &reference.format_scor_reference(), x, &mut y, layout.text_font_size, layout.line_spacing, layout.extra_spacing);
            }
            _ => {}
        }

        // Payable by
        if let Some(debtor) = &layout.bill_data.debtor_address {
            draw_label(ops, label!(PayableBy, layout.language), x, &mut y, layout.label_font_size, layout.line_spacing);
            draw_text_lines(ops, &debtor.to_lines().all_but_last(), x, &mut y, layout.text_font_size, layout.line_spacing, layout.extra_spacing);
        } else {
            draw_label(ops, label!(PayableBy, layout.language), x, &mut y, layout.label_font_size, layout.line_spacing);
            y = Mm(y.0 - layout.config.debtor_box_height.0);
            draw_corner_marks(ops, QRBillLayoutRect {x, y, width: self.payable_box_width, height: self.payable_box_height  }, CORNER_MARKS_PAYABLE_BY_VIEWBOX, CORNER_MARKS_PAYABLE_BY_POLYLINES);
            y = Mm(y.0 - layout.extra_spacing.0);
        }

        if let Some(info_lines) = &layout.bill_data.additional_information {
            if !info_lines.is_empty() {
                draw_label(ops, label!(AdditionalInformation, layout.language), x, &mut y, layout.label_font_size, layout.line_spacing);
                draw_single_line(ops, info_lines, x, &mut y, layout.text_font_size, layout.line_spacing, layout.extra_spacing);
            }
        }

        cursor.y = y;
    }
}