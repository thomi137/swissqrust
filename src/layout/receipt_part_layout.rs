/*
 * Copyright (c) 2026 Thomas Prosser
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */
use pdf_writer::writers::AdditionalActions;
use crate::{Baseline, DrawOp, Mm, compute_spacing, Pt, QRLayoutRect};
use crate::layout::draw::{draw_label, draw_single_line, draw_text_lines};

pub struct ReceiptLayout<'a> {

    // output
    pub ops: Vec<DrawOp>,

    // geometry
    pub horizontal_offset: Mm,
    pub top_start: Mm,

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
    pub additional_information: Option<&'a str>,
}

impl<'a> ReceiptLayout<'a> {
    pub(crate) const RECEIPT_MAX_HEIGHT: Mm = Mm(56f32);
    const DEBTOR_BOX_WIDTH_RC: Mm = Mm(52f32);
    const DEBTOR_BOX_HEIGHT_RC: Mm = Mm(20f32);
    const ACCEPTANCE_POINT_SECTION_TOP: Mm = Mm(23f32);

    pub fn layout_payment_information_section(&mut self) {
        let mut y = Mm(self.top_start.0 - self.label_ascender.0);
        let x = self.horizontal_offset;

        // Account / Payable to
        draw_label(
            &mut self.ops,
            "Account / Payable to",
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
                "Reference",
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

        // Additional information

        // Payable by
        draw_label(
            &mut self.ops,
            "Payable by",
            x,
            &mut y,
            self.label_font_size,
            self.line_spacing,
        );

        match &self.payable_by_lines {
            Some(lines) => {
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
                self.ops.push(DrawOp::Box {
                    rect: QRLayoutRect {
                        x,
                        y: Mm(y.0 - Self::DEBTOR_BOX_HEIGHT_RC.0),
                        width: Self::DEBTOR_BOX_WIDTH_RC,
                        height: Self::DEBTOR_BOX_HEIGHT_RC,
                    },
                });

                y = Mm(y.0 - Self::DEBTOR_BOX_HEIGHT_RC.0 - self.extra_spacing.0);
            }
        }
    }

    pub fn layout_receipt_acceptance_point(&mut self, text_width: Mm) {
        let y = Mm(Self::ACCEPTANCE_POINT_SECTION_TOP.0 - self.label_ascender.0);

        self.ops.push(DrawOp::Text {
            text: "Acceptance point".into(),
            at: Baseline {
                x: Mm(self.horizontal_offset.0 + text_width.0),
                y,
            },
            size: self.label_font_size,
            bold: true,
        });
    }



    }










