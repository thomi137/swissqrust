/*
 * Copyright (c) 2026 Thomas Prosser
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */

use crate::{Baseline, DrawOp, Mm, compute_spacing, Pt, QRLayoutRect, label, Language, BillData, ReferenceType};
use crate::layout::draw::{draw_label, draw_single_line, draw_text_lines};
use crate::constants::*;

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

        draw_text_lines(
            ops,
            &self.bill_data.creditor_address.to_lines(),
            x,
            &mut y,
            self.text_font_size,
            self.line_spacing,
            self.extra_spacing,
        );

        // Reference
        match &self.bill_data.reference_type {
            ReferenceType::QrRef(reference)
            | ReferenceType::Creditor(reference) => {
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
                    reference,
                    x,
                    &mut y,
                    self.text_font_size,
                    self.line_spacing,
                    self.extra_spacing,
                );
            }
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
            draw_label(
                ops,
                label!(PayableBy, self.language),
                x,
                &mut y,
                self.label_font_size,
                self.line_spacing,
            );

            draw_text_lines(
                ops,
                &debtor.to_lines(),
                x,
                &mut y,
                self.text_font_size,
                self.line_spacing,
                self.extra_spacing,
            );
        } else {
            draw_label(
                ops,
                label!(PayableBy, self.language),
                x,
                &mut y,
                self.label_font_size,
                self.line_spacing,
            );

            y = Mm(y.0 - DEBTOR_BOX_HEIGHT.0);

            ops.push(DrawOp::Box {
                rect: QRLayoutRect {
                    x,
                    y,
                    width: DEBTOR_BOX_WIDTH_PP,
                    height: DEBTOR_BOX_HEIGHT,
                },
            });

            y = Mm(y.0 - self.extra_spacing.0);
        }
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
                - self.text_ascender.0,
        );

        // Currency value
        ops.push(DrawOp::Text {
            text: label!(Currency, self.language).into(),
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
                    text: amount.clone(),
                    at: Baseline {
                        x: amount_x,
                        y: value_y,
                    },
                    size: self.text_font_size,
                    bold: false,
                });
            }
            None => {
                ops.push(DrawOp::Box {
                    rect: QRLayoutRect {
                        x: amount_x,
                        y: Mm(value_y.0 - AMOUNT_BOX_HEIGHT_RC.0 + self.text_ascender.0),
                        width: AMOUNT_BOX_WIDTH_RC,
                        height: AMOUNT_BOX_HEIGHT_RC,
                    },
                });
            }
        }
    }

    pub fn layout_receipt_acceptance_point(&mut self,  ops: &mut Vec<DrawOp>, text_width: Mm,) {
        let y = Mm(ACCEPTANCE_POINT_SECTION_TOP.0 - self.label_ascender.0);

        ops.push(DrawOp::Text {
            text: label!(AcceptancePoint, self.language).into(),
            at: Baseline {
                x: Mm(self.horizontal_offset.0 + text_width.0),
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

    pub fn render(&mut self, ops: &mut Vec<DrawOp>) {
        self.compute_receipt_spacing();
        self.layout_receipt_information_section(ops);
        self.layout_receipt_amount_section(ops, AMOUNT_SECTION_TOP);
        self.layout_receipt_acceptance_point(ops, RECEIPT_TEXT_WIDTH);
    }
}
