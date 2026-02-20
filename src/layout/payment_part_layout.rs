/*
 * Copyright (c) 2026 Thomas Prosser
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */
use std::ops::Mul;
use crate::{label, BillData, FontLibrary, Language, ReferenceType, CORNER_MARKS_AMOUNT_POLYLINES, CORNER_MARKS_AMOUNT_VIEWBOX};
use crate::layout::draw::*;
use crate::layout::geometry::*;
use crate::layout::spacing::*;
use crate::constants::*;
use crate::formatters::{SliceExt, SwissQRFormatter};

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

impl Mul<Mm> for i32 {
    type Output = ();

    fn mul(self, rhs: Mm) -> Self::Output {
        todo!()
    }
}

impl<'a> PaymentPartLayout<'a> {

    pub fn layout_payment_part_title_section(&mut self, ops: &mut Vec<DrawOp>) {
            let x_start = RECEIPT_WIDTH + MARGIN;
            let y = SLIP_HEIGHT - MARGIN - (self.label_ascender * FONT_SIZE_TITLE.to_mm() * Mm(MM_PER_PT));
            ops.push(DrawOp::Text {
                text: label!(PaymentPart, self.language).into(),
                at: Baseline { x: x_start, y },
                size: Pt(11.0),
                bold: true,
            });
    }

    pub fn draw_swiss_qr_code(&mut self, ops: &mut Vec<DrawOp>) {
        let x_start = RECEIPT_WIDTH + MARGIN;
        let y = SLIP_HEIGHT - MARGIN - Mm(7f32) - MARGIN - Mm(QR_CODE_HEIGHT);
        ops.push(DrawOp::QrCodeSpace {
            at: Baseline { x: x_start, y },
            size: Mm(46.0),
        });
    }

    pub fn layout_payment_part_amount_section(&mut self, ops: &mut Vec<DrawOp>, section_top: Mm) {
        let y = Mm(section_top.0 - self.label_ascender.0);

        // Currency label
        ops.push(DrawOp::Text {
            text: label!(Currency, self.language).into(),
            at: Baseline {
                x: self.horizontal_offset + MARGIN,
                y,
            },
            size: self.label_font_size,
            bold: true,
        });

        let value_y = y
            - self.text_ascender
            - self.label_font_size.to_mm();

        // Currency value
        ops.push(DrawOp::Text {
            text: self.bill_data.currency.to_string(),
            at: Baseline {
                x: self.horizontal_offset + MARGIN,
                y: value_y,
            },
            size: self.text_font_size,
            bold: false,
        });

        // Amount Label
        let amount_x = self.horizontal_offset + MARGIN + CURRENCY_WIDTH_PP;
        ops.push(DrawOp::Text {
            text: label!(Amount, self.language).into(),
            at: Baseline {
                x: amount_x,
                y,
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
                    width: AMOUNT_BOX_WIDTH_PP,
                    height: AMOUNT_BOX_HEIGHT_PP,
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

    // INFORMATION SECTION
    pub fn layout_payment_part_information_section(&mut self, ops: &mut Vec<DrawOp>) {
        let mut y = Mm(self.top_start.0 - self.label_ascender.0);
        let x = self.horizontal_offset + PP_INFO_SECTION_HORI_OFFSET;

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

        // Reference
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
                    &debtor.to_lines().all_but_last(),
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
            PAYMENT_PART_MAX_HEIGHT,
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
        self.compute_payment_part_spacing();
        self.layout_payment_part_title_section(ops);
        self.draw_swiss_qr_code(ops);
        self.layout_payment_part_amount_section(ops, PP_AMOUNT_SECTION_TOP);
        self.layout_payment_part_information_section(ops);
    }

}
