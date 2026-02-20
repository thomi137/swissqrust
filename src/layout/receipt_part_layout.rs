/*
 * Copyright (c) 2026 Thomas Prosser
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */
use std::ops::Mul;
use crate::{Baseline, DrawOp, Mm, compute_spacing, Pt, QRBillLayoutRect, label, Language, BillData, ReferenceType, CORNER_MARKS_AMOUNT_VIEWBOX, CORNER_MARKS_AMOUNT_POLYLINES, CORNER_MARKS_PAYABLE_BY_VIEWBOX, CORNER_MARKS_PAYABLE_BY_POLYGONS, CORNER_MARKS_PAYABLE_BY_POLYLINES, FontLibrary, draw_text_at, MM_PER_PT};
use crate::layout::draw::{draw_corner_marks, draw_label, draw_single_line, draw_text_lines};
use crate::constants::*;
use crate::formatters::{SliceExt, SwissQRFormatter};

pub struct ReceiptLayout<'a> {

    pub bill_data: &'a BillData,

    // geometry
    pub horizontal_offset: Mm,
    pub top_start: Mm,

    // typography
    pub label_font_size: Pt,
    pub text_font_size: Pt,
    pub label_ascender: Mm,
    pub text_ascender: Mm,

    // language
    pub language: Language,

    // spacing
    pub line_spacing: Mm,
    pub extra_spacing: Mm,

}


impl<'a> ReceiptLayout<'a> {

    pub fn layout_receipt_title_section(&mut self, ops: &mut Vec<DrawOp>) {
        let y = SLIP_HEIGHT - MARGIN - self.label_ascender * FONT_SIZE_TITLE.to_mm() * Mm(MM_PER_PT); // Baseline approx 4mm from top margin
        ops.push(DrawOp::Text {
            text: label!(Receipt, self.language).into(),
            at: Baseline { x: MARGIN, y },
            size: Pt(11.0),
            bold: true,
        });
    }

    pub fn layout_receipt_information_section(&mut self, ops: &mut Vec<DrawOp>) {
        let mut y = Mm(self.top_start.0 - self.label_ascender.0);
        let x = self.horizontal_offset;

        // Account / Payable to
        draw_label(
            ops,
            label!(AccountPayableTo, self.language),
            x,
            &mut y,
            self.label_font_size,
            self.line_spacing,
        );

        draw_single_line(
            ops,
            &self.bill_data.iban.format_iban(),
            x,
            &mut y,
            self.text_font_size,
            self.line_spacing,
            Mm(0.0),
        );

        draw_text_lines(
            ops,
            &self.bill_data.creditor_address.to_lines().all_but_last(),
            x,
            &mut y,
            self.text_font_size,
            self.line_spacing,
            self.extra_spacing,
        );

        // Reference / Creditor
        match &self.bill_data.reference_type {

            ReferenceType::QrRef(reference) => {

                draw_label(
                    ops,
                    label!(Reference, self.language),
                    x,
                    &mut y,
                    self.label_font_size,
                    self.line_spacing,
                );

                draw_single_line(
                    ops,
                    &reference.format_qr_reference(),
                    x,
                    &mut y,
                    self.text_font_size,
                    self.line_spacing,
                    self.extra_spacing,
                );

            },

            ReferenceType::Creditor(reference) => {

                draw_label(
                    ops,
                    label!(Reference, self.language),
                    x,
                    &mut y,
                    self.label_font_size,
                    self.line_spacing,
                );

                draw_single_line(
                    ops,
                    &reference.format_scor_reference(),
                    x,
                    &mut y,
                    self.text_font_size,
                    self.line_spacing,
                    self.extra_spacing,
                );

            },

            _ => {}
        }

        // Payable by
        draw_label(
            ops,
            label!(PayableBy, self.language),
            x,
            &mut y,
            self.label_font_size,
            self.line_spacing,
        );

        if let Some(debtor) = &self.bill_data.debtor_address {

            draw_text_lines(
                ops,
                &debtor.to_lines().all_but_last(),
                x,
                &mut y,
                self.text_font_size,
                self.line_spacing,
                self.extra_spacing,
            );
        } else {

            let rect = QRBillLayoutRect {
                    x,
                    y,
                    width: DEBTOR_BOX_WIDTH_RC,
                    height: DEBTOR_BOX_HEIGHT,
                };

            draw_corner_marks(
                ops,
                rect,
                CORNER_MARKS_PAYABLE_BY_VIEWBOX,
                CORNER_MARKS_PAYABLE_BY_POLYLINES
            );

        }
        y = Mm(y.0 - DEBTOR_BOX_HEIGHT.0);
    }

    pub fn layout_receipt_amount_section(
            &mut self,
            ops: &mut Vec<DrawOp>,
            section_top: Mm,
        ) {
            let currency_label_y = Mm(section_top.0 - self.label_ascender.0);

            // Currency label
            ops.push(DrawOp::Text {
                text: label!(Currency, self.language).into(),
                at: Baseline {
                    x: self.horizontal_offset,
                    y: currency_label_y,
                },
                size: self.label_font_size,
                bold: true,
            });

            let value_y = Mm(
                currency_label_y.0
                    - self.line_spacing.0
                   // - self.text_ascender.0,
            );

        // Currency value
        ops.push(DrawOp::Text {
            text: self.bill_data.currency.to_string(),
            at: Baseline {
                x: self.horizontal_offset,
                y: value_y,
            },
            size: self.text_font_size,
            bold: false,
        });

        let amount_x = Mm(self.horizontal_offset.0 + CURRENCY_WIDTH.0);

        // Amount label
        ops.push(DrawOp::Text {
            text: label!(Amount, self.language).into(),
            at: Baseline {
                x: amount_x,
                y: currency_label_y,
            },
            size: self.label_font_size,
            bold: true,
        });

        match &self.bill_data.amount {
            Some(amount) => {
                ops.push(DrawOp::Text {
                    text: amount.format_amount(),
                    at: Baseline {
                        x: amount_x,
                        y: value_y,
                    },
                    size: self.text_font_size,
                    bold: false,
                });
            },

            // TODO: Somehow it should be a box, but with special content
            None => {

                let rect = QRBillLayoutRect {
                    x: amount_x,
                    y: Mm(value_y.0 - AMOUNT_BOX_HEIGHT_RC.0 + self.text_ascender.0),
                    width: AMOUNT_BOX_WIDTH_RC,
                    height: AMOUNT_BOX_HEIGHT_RC,
                };

                draw_corner_marks(
                    ops,
                    rect,
                    CORNER_MARKS_AMOUNT_VIEWBOX,
                    CORNER_MARKS_AMOUNT_POLYLINES
                )
            },
        }
    }

    pub fn layout_receipt_acceptance_point(&mut self,  ops: &mut Vec<DrawOp>, fonts: &FontLibrary) {
        let y = Mm(ACCEPTANCE_POINT_SECTION_TOP.0 - self.label_ascender.0);
        let text = label!(AcceptancePoint, self.language).into();
        let text_width_mm = fonts.bold.measure(text, 6.0);

        ops.push(DrawOp::Text {
            text: text.to_string(),
            at: Baseline {
                x: Mm(RECEIPT_WIDTH.0 - MARGIN.0 - text_width_mm),
                y,
            },
            size: self.label_font_size,
            bold: true,
        });
    }

    pub fn compute_receipt_spacing(&mut self) -> bool {
        let mut text_lines = 0usize;
        let mut extra_blocks = 0usize;
        let mut fixed_height = Mm(0.0);

        text_lines += 1;
        text_lines += self.bill_data.creditor_address.to_lines().len();

        match &self.bill_data.reference_type {
            ReferenceType::NoRef => {}
            _ => {
                extra_blocks += 1;
                text_lines += 2; // label + value
            }
        }

        extra_blocks += 1;

        match &self.bill_data.debtor_address {
            Some(lines) => {
                text_lines += 1 + lines.to_lines().len();
            }
            None => {
                text_lines += 1;
                fixed_height =
                    Mm(fixed_height.0 + DEBTOR_BOX_HEIGHT_RC.0);
            }
        }

        extra_blocks += 1;

        let spacing = compute_spacing(
            RECEIPT_MAX_HEIGHT,
            fixed_height,
            text_lines,
            extra_blocks,
            self.line_spacing,
        );

        self.line_spacing = spacing.line_spacing;
        self.extra_spacing = spacing.extra_spacing;

        spacing.extra_spacing.0 / spacing.line_spacing.0 < 0.8
    }

    pub fn render(&mut self, ops: &mut Vec<DrawOp>, fonts: &FontLibrary) {
        self.compute_receipt_spacing();
        self.layout_receipt_title_section(ops);
        self.top_start = self.top_start - Mm(7f32);
        self.layout_receipt_information_section(ops);
        self.layout_receipt_amount_section(ops, AMOUNT_SECTION_TOP);
        self.layout_receipt_acceptance_point(ops, fonts);
    }
}
