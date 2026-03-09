/*
 * Copyright (c) 2026 Thomas Prosser
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */

use crate::render::layout::geometry::*;
use crate::render::types::DrawOp;
use crate::{BillData, Column, Language, LayoutStrategy, RenderContext, SlipPart};
use crate::constants::*;
use crate::block_elements::{ColumnCursor, LayoutBlock};
use crate::blocks::amount_block::AmountBlock;
use crate::blocks::information_block::InformationBlock;
use crate::blocks::qr_block::QrBlock;
use crate::blocks::title_block::TitleBlock;
use crate::render::FontMetrics;
use crate::spacer_block::SpacerBlock;

pub struct PaymentPartLayout<T: FontMetrics> {
    blocks: Vec<Box<dyn LayoutBlock<T>>>
}

impl<T: FontMetrics> PaymentPartLayout<T> {
    pub fn new() -> Self {
        Self {
            blocks: vec![
                Box::new(TitleBlock { label: crate::LabelKey::PaymentPart }),
                Box::new(InformationBlock { part: SlipPart::PaymentPart, offset: PP_INFO_SECTION_HORI_OFFSET, payable_box_width: DEBTOR_BOX_WIDTH_PP, payable_box_height: DEBTOR_BOX_HEIGHT }),
                Box::new(QrBlock),
                Box::new(SpacerBlock { min_height: Mm(260f32) }),
                Box::new(AmountBlock { part: SlipPart::PaymentPart, amount_box_width: AMOUNT_BOX_WIDTH_PP, amount_box_height: AMOUNT_BOX_HEIGHT_PP }),
            ]
        }
    }
}

impl <T: FontMetrics> LayoutStrategy<T> for PaymentPartLayout<T> {
    const LABEL_SIZE: Pt = PP_LABEL_PREF_FONT_SIZE;
    const TEXT_SIZE: Pt =  PP_TEXT_PREF_FONT_SIZE;
    const TITLE_SIZE: Pt = TITLE_FONT_SIZE;
    const MAX_HEIGHT: Mm = PAYMENT_PART_MAX_HEIGHT;

    fn render(&mut self,
              bill_data: &BillData,
              lang: Language,
              metrics: &T,
              ops: &mut Vec<DrawOp>) {

        // Calculate metrics specific to THIS part
        let ctx = RenderContext::for_strategy::<Self>(bill_data, lang, metrics);

        let base_x = RECEIPT_WIDTH + MARGIN;

        let mut left_cursor = ColumnCursor::new(
            base_x,
            A4_PAGE_HEIGHT - Mm(100f32),
        );

        let mut right_cursor = ColumnCursor::new(
            base_x + PP_INFO_SECTION_HORI_OFFSET,
            A4_PAGE_HEIGHT - Mm(100f32) + ctx.label_ascender,
        );

        for block in &self.blocks {
            match block.column() {
                Column::Left => block.render(&ctx, ops, &mut left_cursor),
                Column::Right => block.render(&ctx, ops, &mut right_cursor),
                Column::Absolute => block.render(&ctx, ops, &mut left_cursor),
            }
        }

    }
}
