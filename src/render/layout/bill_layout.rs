/*
 * Copyright (c) 2026 Thomas Prosser
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */

use crate::{label, BillData, Language, ReferenceType};
use crate::language::*;
use crate::render::layout::draw::*;
use crate::render::layout::geometry::*;
use crate::render::layout::spacing::*;
use crate::render::types::DrawOp;
use crate::constants::*;
use crate::generated::{CORNER_MARKS_AMOUNT_POLYLINES, CORNER_MARKS_AMOUNT_VIEWBOX};
use crate::support::traits::{SwissQRFormatter, SliceExt};

/// Configuration for which sections to render.
pub struct BillLayoutConfig {
    pub has_qr_code: bool,
    pub has_acceptance_point: bool,
    pub max_height: Mm,
    pub debtor_box_height: Mm,
    pub amount_section_top: Mm,
}

/// Base layout engine shared by Payment Part and Receipt
pub struct BillLayout<'a> {
    pub bill_data: &'a BillData,
    pub config: BillLayoutConfig,

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

impl<'a> BillLayout<'a> {
    pub fn layout_title_section(&mut self, ops: &mut Vec<DrawOp>, label_key: LabelKey, x_offset: Mm) {
        let y = SLIP_HEIGHT - MARGIN - (self.label_ascender * FONT_SIZE_TITLE.to_mm() * Mm(MM_PER_PT));

        // small detour because title is only available when running.
        let label_text = label(label_key, self.language)
            .unwrap_or("");
        ops.push(DrawOp::Text {
            text: label_text.to_string(),
            at: Baseline { x: x_offset, y },
            size: Pt(11.0),
            bold: true,
        });
    }

    pub fn layout_amount_section(&mut self, ops: &mut Vec<DrawOp>, section_top: Mm, x_offset: Mm, amount_box_width: Mm, amount_box_height: Mm) {
        let y = Mm(section_top.0 - self.label_ascender.0);

        // Currency label
        ops.push(DrawOp::Text {
            text: label!(Currency, self.language).into(),
            at: Baseline { x: x_offset, y },
            size: self.label_font_size,
            bold: true,
        });

        let value_y = y - self.text_ascender - self.label_font_size.to_mm();

        // Currency value
        ops.push(DrawOp::Text {
            text: self.bill_data.currency.to_string(),
            at: Baseline { x: x_offset, y: value_y },
            size: self.text_font_size,
            bold: false,
        });

        // Amount Label
        let amount_x = x_offset + MARGIN + CUCCENCY_WIDTH_PP;
        ops.push(DrawOp::Text {
            text: label!(Amount, self.language).into(),
            at: Baseline { x: amount_x, y },
            size: self.label_font_size,
            bold: true,
        });

        match &self.bill_data.amount {
            Some(amount) => {
                ops.push(DrawOp::Text {
                    text: amount.format_amount(),
                    at: Baseline { x: amount_x, y: value_y },
                    size: self.text_font_size,
                    bold: false,
                });
            }
            None => {
                let rect = QRBillLayoutRect {
                    x: amount_x,
                    y: Mm(value_y.0 - amount_box_height.0 + self.text_ascender.0),
                    width: amount_box_width,
                    height: amount_box_height,
                };

                draw_corner_marks(
                    ops,
                    rect,
                    CORNER_MARKS_AMOUNT_VIEWBOX,
                    CORNER_MARKS_AMOUNT_POLYLINES
                )
            }
        }
    }

    pub fn layout_information_section(&mut self, ops: &mut Vec<DrawOp>, x_offset: Mm) {
        let mut y = Mm(self.top_start.0 - self.label_ascender.0);
        let x = x_offset;

        // Account / Payable to
        draw_label(ops, label!(AccountPayableTo, self.language), x, &mut y, self.label_font_size, self.line_spacing);
        draw_single_line(ops, &self.bill_data.iban.format_iban(), x, &mut y, self.text_font_size, self.line_spacing, Mm(0.0));
        draw_text_lines(ops, &self.bill_data.creditor_address.to_lines().all_but_last(), x, &mut y, self.text_font_size, self.line_spacing, self.extra_spacing);

        // Reference
        match &self.bill_data.reference_type {
            ReferenceType::QrRef(reference) => {
                draw_label(ops, label!(Reference, self.language), x, &mut y, self.label_font_size, self.line_spacing);
                draw_single_line(ops, &reference.format_qr_reference(), x, &mut y, self.text_font_size, self.line_spacing, self.extra_spacing);
            }
            ReferenceType::Creditor(reference) => {
                draw_label(ops, label!(Reference, self.language), x, &mut y, self.label_font_size, self.line_spacing);
                draw_single_line(ops, &reference.format_scor_reference(), x, &mut y, self.text_font_size, self.line_spacing, self.extra_spacing);
            }
            _ => {}
        }

        // Payable by
        if let Some(debtor) = &self.bill_data.debtor_address {
            draw_label(ops, label!(PayableBy, self.language), x, &mut y, self.label_font_size, self.line_spacing);
            draw_text_lines(ops, &debtor.to_lines().all_but_last(), x, &mut y, self.text_font_size, self.line_spacing, self.extra_spacing);
        } else {
            draw_label(ops, label!(PayableBy, self.language), x, &mut y, self.label_font_size, self.line_spacing);
            y = Mm(y.0 - self.config.debtor_box_height.0);
            ops.push(DrawOp::Box {
                rect: QRBillLayoutRect { x, y, width: DEBTOR_BOX_WIDTH_PP, height: self.config.debtor_box_height },
            });
            y = Mm(y.0 - self.extra_spacing.0);
        }

        if let Some(info_lines) = &self.bill_data.additional_information {
            if !info_lines.is_empty() {
                draw_label(ops, label!(AdditionalInformation, self.language), x, &mut y, self.label_font_size, self.line_spacing);
                draw_single_line(ops, info_lines, x, &mut y, self.text_font_size, self.line_spacing, self.extra_spacing);
            }
        }
    }

    pub fn compute_spacing(&mut self) -> bool {
        let mut text_lines = 0usize;
        let mut extra_blocks = 0usize;
        let mut fixed_height = Mm(0.0);

        text_lines += 1 + self.bill_data.creditor_address.to_lines().len();

        match &self.bill_data.reference_type {
            ReferenceType::NoRef => {}
            _ => {
                extra_blocks += 1;
                text_lines += 2;
            }
        }
        extra_blocks += 1;

        match &self.bill_data.debtor_address {
            Some(address) => text_lines += 1 + address.to_lines().len(),
            None => {
                text_lines += 1;
                fixed_height = Mm(fixed_height.0 + self.config.debtor_box_height.0);
            }
        }
        extra_blocks += 1;

        let spacing = compute_spacing(self.config.max_height, fixed_height, text_lines, extra_blocks, self.line_spacing);

        self.line_spacing = spacing.line_spacing;
        self.extra_spacing = spacing.extra_spacing;

        spacing.extra_spacing.0 / spacing.line_spacing.0 < 0.8
    }
}
