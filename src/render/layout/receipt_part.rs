/*
 * Copyright (c) 2026 Thomas Prosser
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */

use crate::language::LabelKey;
use crate::{BillData, Language, LayoutStrategy, RenderContext, SlipPart, FontStyle};
use crate::render::layout::geometry::*;
use crate::render::types::DrawOp;
use crate::constants::*;
use crate::block_elements::{ColumnCursor, LayoutBlock};
use crate::blocks::amount_block::AmountBlock;
use crate::blocks::information_block::InformationBlock;
use crate::blocks::title_block::TitleBlock;
use crate::pdf::coords::LayoutY;
use crate::render::FontMetrics;
use crate::spacer_block::SpacerBlock;

pub struct ReceiptLayout<T: FontMetrics>{
    blocks: Vec<Box<dyn LayoutBlock<T>>>
}

impl<T: FontMetrics> ReceiptLayout<T> {
    pub fn new() -> Self {
        Self {
            blocks: vec![
                Box::new(TitleBlock { label: LabelKey::Receipt }),
                Box::new(InformationBlock { part: SlipPart::Receipt, offset: Mm(0f32), payable_box_width: DEBTOR_BOX_WIDTH_RC, payable_box_height: DEBTOR_BOX_HEIGHT_RC }),
                Box::new(SpacerBlock { min_height: Mm(260f32) }),
                Box::new(AmountBlock { part: SlipPart::Receipt, amount_box_width: AMOUNT_BOX_WIDTH_RC, amount_box_height: AMOUNT_BOX_HEIGHT_RC }),
            ]
        }
    }
}

impl <T: FontMetrics> LayoutStrategy<T> for ReceiptLayout<T> {

        const LABEL_SIZE: Pt = RC_LABEL_PREF_FONT_SIZE;
        const TEXT_SIZE: Pt = RC_TEXT_PREF_FONT_SIZE;
        const TITLE_SIZE: Pt = TITLE_FONT_SIZE;
        const MAX_HEIGHT: Mm = RECEIPT_PART_MAX_HEIGHT;

        fn render(&mut self, bill_data: &BillData, language: Language, metrics: &T, ops: &mut Vec<DrawOp>) {

            let ctx = RenderContext::for_strategy::<Self>(bill_data, language, metrics);

            let mut main_cursor = ColumnCursor::new(
                MARGIN,
                A4_PAGE_HEIGHT - Mm(100f32),
            );

            for block in &self.blocks {
                block.render(&ctx, ops, &mut main_cursor)
            }


            layout_acceptance_point::<T>(&ctx, ops, &metrics);

        }
    }

    fn layout_acceptance_point<T: FontMetrics>(ctx: &RenderContext<T>, ops: &mut Vec<DrawOp>, fonts: &T) {
        let y = A4_PAGE_HEIGHT - ACCEPTANCE_POINT_SECTION_TOP + fonts.ascender_mm(FontStyle::Bold, RC_LABEL_PREF_FONT_SIZE);
        let label_text = crate::language::label(LabelKey::AcceptancePoint, ctx.language)
            .unwrap_or("Acceptance point");
        let text_width_mm = ctx.metrics.text_width_mm(label_text, FontStyle::Bold, ctx.label_size);

        ops.push(DrawOp::Text {
            text: label_text.to_string(),
            at: Baseline {
                x: Mm(RECEIPT_WIDTH.0 - MARGIN.0 - text_width_mm.0),
                y: LayoutY(y),
            },
            size: ctx.label_size,
            bold: true,
        });
    }
