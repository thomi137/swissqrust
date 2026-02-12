/*
 * Copyright (c) 2026 Thomas Prosser
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */
use crate::{label, Language};
use crate::layout::draw::*;
use crate::layout::geometry::*;
use crate::layout::spacing::*;
use crate::constants::*;

pub struct PaymentPartLayout<'a> {

    // output
    pub ops: Vec<DrawOp>,

    // geometry
    pub horizontal_offset: Mm,
    pub top_start: Mm,

    // Label language.
    pub language: Language,

    // typography
    pub label_font_size: Pt,
    pub text_font_size: Pt,
    pub label_ascender: Mm,
    pub text_ascender: Mm,

    // spacing
    pub line_spacing: Mm,
    pub extra_spacing: Mm,

    // content
    pub payable_to_lines: &'a [String],
    pub reference: Option<&'a str>,
    pub payable_by_lines: Option<&'a [String]>,
    pub currency: &'a str,
    pub amount: Option<&'a str>,
    pub additional_information: Option<&'a [String]>,
}

impl<'a> PaymentPartLayout<'a> {
   
    // INFORMATION SECTION
    pub fn layout_payment_part_information_section(&mut self) {
        let mut y = Mm(self.top_start.0 - self.label_ascender.0);
        let x = self.horizontal_offset;

        // Account / Payable to
        draw_label(
            &mut self.ops,
            label!(AccountPayableTo, self.language),
            x,
            &mut y,
            self.label_font_size,
            self.line_spacing,
        );

        draw_text_lines(
            &mut self.ops,
            self.payable_to_lines,
            x,
            &mut y,
            self.text_font_size,
            self.line_spacing,
            self.extra_spacing,
        );

        // Reference
        if let Some(reference) = &self.reference {
            draw_label(
                &mut self.ops,
                label!(Reference, self.language),
                x,
                &mut y,
                self.label_font_size,
                self.line_spacing,
            );

            draw_single_line(
                &mut self.ops,
                reference,
                x,
                &mut y,
                self.text_font_size,
                self.line_spacing,
                self.extra_spacing,
            );
        }

        // Payable by
        match &self.payable_by_lines {
            Some(lines) => {
                draw_label(
                    &mut self.ops,
                    "Payable by",
                    x,
                    &mut y,
                    self.label_font_size,
                    self.line_spacing,
                );

                draw_text_lines(
                    &mut self.ops,
                    lines,
                    x,
                    &mut y,
                    self.text_font_size,
                    self.line_spacing,
                    self.extra_spacing,
                );
            }
            None => {
                draw_label(
                    &mut self.ops,
                    "Payable by (name/address)",
                    x,
                    &mut y,
                    self.label_font_size,
                    self.line_spacing,
                );

                y = Mm(y.0 - DEBTOR_BOX_HEIGHT.0);

                self.ops.push(DrawOp::Box {
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

        if let Some(info_lines) = &self.additional_information {
            draw_label(
                &mut self.ops,
                label!(AdditionalInformation, self.language),
                x,
                &mut y,
                self.label_font_size,
                self.line_spacing,
            );

            draw_text_lines(
                &mut self.ops,
                info_lines,
                x,
                &mut y,
                self.text_font_size,
                self.line_spacing,
                self.extra_spacing,
            );
        }

    }

    // AMOUNT SECTION
    pub fn layout_payment_part_amount_section(&mut self, section_top: Mm) {
        let currency_label_y = Mm(section_top.0 - self.label_ascender.0);

        // Currency label
        self.ops.push(DrawOp::Text {
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
        self.ops.push(DrawOp::Text {
            text: self.currency.to_string(),
            at: Baseline {
                x: self.horizontal_offset,
                y: value_y,
            },
            size: self.text_font_size,
            bold: false,
        });
    }

    pub fn layout_payment_qr_section(&mut self) {
        self.ops.push(DrawOp::Box {
            rect: QRLayoutRect {
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
        text_lines += self.payable_to_lines.len();

        // Reference
        if self.reference.is_some() {
            extra_blocks += 1;
            text_lines += 2;
        }

        extra_blocks += 1;

        match &self.payable_by_lines {
            Some(lines) => {
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
}
