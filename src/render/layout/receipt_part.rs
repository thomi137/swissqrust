
/*
 * Copyright (c) 2026 Thomas Prosser
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */

use crate::language::LabelKey;
use crate::{BillData, FontLibrary, Language};
use crate::render::layout::bill_layout::{BillLayout, BillLayoutConfig};
use crate::render::layout::geometry::*;
use crate::render::types::DrawOp;
use crate::constants::*;
use crate::layout::block::{ColumnCursor, LayoutBlock};
use crate::layout::blocks::amount_block::AmountBlock;
use crate::layout::blocks::information_block::InformationBlock;
use crate::layout::blocks::title_block::TitleBlock;

pub struct ReceiptLayout<'a>{
    layout: BillLayout<'a>,
    blocks: Vec<Box<dyn LayoutBlock>>
}

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
            let layout = BillLayout {
            bill_data,
            config: BillLayoutConfig {
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
        };
        Self{
            layout,
            blocks: vec![
                    Box::new(TitleBlock { label: LabelKey::Receipt }),
                    Box::new(InformationBlock {offset: Mm(0.0), payable_box_width: DEBTOR_BOX_WIDTH_RC, payable_box_height: DEBTOR_BOX_HEIGHT_RC}),
                    Box::new(AmountBlock{amount_box_width: AMOUNT_BOX_WIDTH_RC, amount_box_height: AMOUNT_BOX_HEIGHT_RC}),
            ]
        }

    }

    pub fn layout_acceptance_point(&mut self, ops: &mut Vec<DrawOp>, fonts: &FontLibrary) {
        let y = ACCEPTANCE_POINT_SECTION_TOP - self.layout.label_ascender;
        let label_text = crate::language::label(LabelKey::AcceptancePoint, self.layout.language)
            .unwrap_or("Acceptance point");
        let text_width_mm = fonts.bold.measure(label_text, 6.0);

        ops.push(DrawOp::Text {
            text: label_text.to_string(),
            at: Baseline {
                x: Mm(RECEIPT_WIDTH.0 - MARGIN.0 - text_width_mm),
                y,
            },
            size: self.layout.label_font_size,
            bold: true,
        });
    }

    pub fn render(&mut self, ops: &mut Vec<DrawOp>, fonts: &FontLibrary) {

        self.layout.compute_spacing();


        let mut main_cursor = ColumnCursor::new(
            self.layout.horizontal_offset,
            self.layout.top_start,
        );

        for block in &self.blocks {
            block.render(&mut self.layout, ops, &mut main_cursor)
        }
        self.layout_acceptance_point(ops, fonts);
    }
}