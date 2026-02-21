
/*
 * Copyright (c) 2026 Thomas Prosser
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */

use crate::language::LabelKey;
use crate::{BillData, FontLibrary, Language, ReferenceType, CORNER_MARKS_AMOUNT_POLYLINES, CORNER_MARKS_AMOUNT_VIEWBOX, CORNER_MARKS_PAYABLE_BY_POLYLINES, CORNER_MARKS_PAYABLE_BY_VIEWBOX};
use crate::render::layout::bill_layout::{BillLayout, BillLayoutConfig};
use crate::render::layout::draw::draw_corner_marks;
use crate::render::layout::geometry::*;
use crate::render::layout::spacing::*;
use crate::render::types::DrawOp;
use crate::constants::*;
use crate::support::traits::{SwissQRFormatter, SliceExt};

pub struct ReceiptLayout<'a>(pub BillLayout<'a>);

impl<'a> ReceiptLayout<'a> {
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
                has_qr_code: false,
                has_acceptance_point: true,
                max_height: RECEIPT_MAX_HEIGHT,
                debtor_box_height: DEBTOR_BOX_HEIGHT_RC,
                amount_section_top: AMOUNT_SECTION_TOP,
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

    pub fn layout_acceptance_point(&mut self, ops: &mut Vec<DrawOp>, fonts: &FontLibrary) {
        let y = Mm(ACCEPTANCE_POINT_SECTION_TOP.0 - self.0.label_ascender.0);
        let label_text = crate::language::label(LabelKey::AcceptancePoint, self.0.language)
            .unwrap_or("Acceptance point");
        let text_width_mm = fonts.bold.measure(label_text, 6.0);

        ops.push(DrawOp::Text {
            text: label_text.to_string(),
            at: Baseline {
                x: Mm(RECEIPT_WIDTH.0 - MARGIN.0 - text_width_mm),
                y,
            },
            size: self.0.label_font_size,
            bold: true,
        });
    }

    pub fn render(&mut self, ops: &mut Vec<DrawOp>, fonts: &FontLibrary) {
        self.0.compute_spacing();
        self.0.layout_title_section(ops, LabelKey::Receipt, MARGIN);
        self.0.top_start = Mm(self.0.top_start.0 - 7f32);
        self.0.layout_information_section(ops, self.0.horizontal_offset);
        self.0.layout_amount_section(ops, AMOUNT_SECTION_TOP, self.0.horizontal_offset, AMOUNT_BOX_WIDTH_RC, AMOUNT_BOX_HEIGHT_RC);
        self.layout_acceptance_point(ops, fonts);
    }
}