/*
 * Copyright (c) 2026 Thomas Prosser
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */
use crate::{label, BillData, Language, ReferenceType};
use crate::layout::draw::*;
use crate::layout::geometry::*;
use crate::layout::spacing::*;
use crate::constants::*;

pub struct PaymentPartLayout<'a> {
    pub bill_data: &'a BillData,

    // geometry
    pub horizontal_offset: Mm,
    pub top_start: Mm,

    // language
    pub language: Language,

    // typography
    pub label_font_size: Pt,
    pub text_font_size: Pt,
    pub label_ascender: Mm,
    pub text_ascender: Mm,

    // spacing (computed)
    pub line_spacing: Mm,
    pub extra_spacing: Mm,
}

impl<'a> PaymentPartLayout<'a> {

    // INFORMATION SECTION
    pub fn layout_payment_part_information_section(&mut self, ops: &mut Vec<DrawOp>) {
        let mut y = Mm(self.top_start.0 - self.label_ascender.0);
        let x = self.horizontal_offset;

        let creditor_lines = self.bill_data.creditor_address.to_lines();
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
            &creditor_lines,
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
                    rect: QRBillLayoutRect {
                        x,
                        y,
                        width: DEBTOR_BOX_WIDTH_PP,
                        height: DEBTOR_BOX_HEIGHT,
                    },
                });

                y = Mm(y.0 - self.extra_spacing.0);
            }

        if let Some(info_lines) = &self.bill_data.additional_information {
            if !info_lines.is_empty() {
                draw_label(
                    ops,
                    label!(AdditionalInformation, self.language),
                    x,
                    &mut y,
                    self.label_font_size,
                    self.line_spacing,
                );

                draw_single_line(
                    ops,
                    info_lines,
                    x,
                    &mut y,
                    self.text_font_size,
                    self.line_spacing,
                    self.extra_spacing,
                );
            }
        }
    }

    // AMOUNT SECTION
    pub fn layout_payment_part_amount_section(&mut self, ops: &mut Vec<DrawOp>, section_top: Mm) {
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
                - self.text_ascender.0
                - Pt(3.0).to_mm().0,
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
    }

    pub fn layout_payment_qr_section(&mut self, ops: &mut Vec<DrawOp>) {
        ops.push(DrawOp::Box {
            rect: QRBillLayoutRect {
                x: Mm(5f32),
                y: Mm(5f32),
                width: Mm(46f32),
                height: Mm(46f32),
            },
        });

    }
    
    // SPACING COMPUTATION
    pub fn compute_payment_part_spacing(&mut self) -> bool {
        let mut text_lines = 0usize;
        let mut extra_blocks = 0usize;
        let mut fixed_height = Mm(0.0);

        // Account / Payable to
        text_lines += 1;
        text_lines += self.bill_data.creditor_address.to_lines().len();

        // Reference
        match &self.bill_data.reference_type {
            ReferenceType::NoRef => {}
            _ => {
                extra_blocks += 1;
                text_lines += 2; // label + value
            }
        }

        extra_blocks += 1;

        match &self.bill_data.debtor_address {
            Some(address) => {
                let lines = address.to_lines();
                text_lines += 1 + lines.len();
            }
            None => {
                text_lines += 1;
                fixed_height =
                    Mm(fixed_height.0 + DEBTOR_BOX_HEIGHT.0);
            }
        }

        extra_blocks += 1;

        let spacing = compute_spacing(
            PAYMENT_PART_MAX_HEIGHT,            fixed_height,
            text_lines,
            extra_blocks,
            self.line_spacing,
        );

        self.line_spacing = spacing.line_spacing;
        self.extra_spacing = spacing.extra_spacing;

        spacing.extra_spacing.0 / spacing.line_spacing.0 < 0.8
    }

    pub fn render(&mut self, ops: &mut Vec<DrawOp>) {
        self.compute_payment_part_spacing();
        self.layout_payment_part_information_section(ops);
        self.layout_payment_part_amount_section(ops, AMOUNT_SECTION_TOP);
    }

}
