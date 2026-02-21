/*
 * Copyright (c) 2026 Thomas Prosser
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */

use crate::render::layout::bill_layout::{BillLayout, BillLayoutConfig};
use crate::render::layout::geometry::*;
use crate::render::types::DrawOp;
use crate::{BillData, FontLibrary, Language, CORNER_MARKS_AMOUNT_POLYLINES, CORNER_MARKS_AMOUNT_VIEWBOX};
use crate::constants::*;

pub struct PaymentPartLayout<'a>(pub BillLayout<'a>);

impl<'a> PaymentPartLayout<'a> {
    pub fn new(
        bill_data: &'a BillData,
        horizontal_offset: Mm,
        top_start: Mm,
        language: Language,
        label_font_size: Pt,
        text_font_size: Pt,
        label_ascender: Mm,
        text_ascender: Mm,
        line_spacing: Mm,
        extra_spacing: Mm,
    ) -> Self {
        Self(BillLayout {
            bill_data,
            config: BillLayoutConfig {
                has_qr_code: true,
                has_acceptance_point: false,
                max_height: PAYMENT_PART_MAX_HEIGHT,
                debtor_box_height: DEBTOR_BOX_HEIGHT,
                amount_section_top: PP_AMOUNT_SECTION_TOP,
            },
            horizontal_offset,
            top_start,
            language,
            label_font_size,
            text_font_size,
            label_ascender,
            text_ascender,
            line_spacing,
            extra_spacing,
        })
    }

    pub fn draw_swiss_qr_code(&self, ops: &mut Vec<DrawOp>) {
        let x_start = RECEIPT_WIDTH + MARGIN;
        let y = SLIP_HEIGHT - MARGIN - Mm(7f32) - MARGIN - Mm(QR_CODE_HEIGHT);
        ops.push(DrawOp::QrCodeSpace {
            at: Baseline { x: x_start, y },
            size: Mm(46.0),
        });
    }

    pub fn render(&mut self, ops: &mut Vec<DrawOp>, _fonts: &FontLibrary) {
        self.0.compute_spacing();
        self.0.layout_title_section(ops, crate::LabelKey::PaymentPart, RECEIPT_WIDTH + MARGIN);
        self.draw_swiss_qr_code(ops);
        self.0.layout_amount_section(ops, PP_AMOUNT_SECTION_TOP, self.0.horizontal_offset, AMOUNT_BOX_WIDTH_PP, AMOUNT_BOX_HEIGHT_PP);
        self.0.layout_information_section(ops, self.0.horizontal_offset + PP_INFO_SECTION_HORI_OFFSET);
    }
}
