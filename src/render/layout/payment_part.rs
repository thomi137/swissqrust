/*
 * Copyright (c) 2026 Thomas Prosser
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */

use crate::render::layout::bill_layout::{BillLayout, BillLayoutConfig};
use crate::render::layout::geometry::*;
use crate::render::types::DrawOp;
use crate::{BillData, Column, Language, SlipPart};
use crate::constants::*;
use crate::block_elements::{ColumnCursor, LayoutBlock};
use crate::blocks::amount_block::AmountBlock;
use crate::blocks::information_block::InformationBlock;
use crate::blocks::qr_block::QrBlock;
use crate::blocks::title_block::TitleBlock;
use crate::spacer_block::SpacerBlock;

pub struct PaymentPartLayout<'a> {
    layout: BillLayout<'a>,
    blocks: Vec<Box<dyn LayoutBlock>>
}


impl<'a> PaymentPartLayout<'a> {
    pub fn new(
        bill_data: &'a BillData,
        language: Language,
        label_font_size: Pt,
        text_font_size: Pt,
        line_spacing: Mm,
        extra_spacing: Mm,
        title_ascender: Mm,
        label_ascender: Mm,
        text_ascender: Mm
    ) -> Self {
        let layout = BillLayout {
            bill_data,
            config: BillLayoutConfig {
                has_acceptance_point: false,
                max_height: PAYMENT_PART_MAX_HEIGHT,
                debtor_box_height: DEBTOR_BOX_HEIGHT,
                amount_section_top: PP_AMOUNT_SECTION_TOP,
            },
            language,
            label_font_size,
            text_font_size,
            line_spacing,
            extra_spacing,
            title_ascender,
            label_ascender,
            text_ascender,
        };
        Self {
            layout,
            blocks: vec![
                Box::new(TitleBlock { label: crate::LabelKey::PaymentPart }),
                Box::new(InformationBlock {part: SlipPart::PaymentPart, offset: PP_INFO_SECTION_HORI_OFFSET, payable_box_width: DEBTOR_BOX_WIDTH_PP, payable_box_height: DEBTOR_BOX_HEIGHT}),
                Box::new( QrBlock ),
                Box::new(SpacerBlock{min_height: Mm(260f32)}),
                Box::new(AmountBlock{part: SlipPart::PaymentPart, amount_box_width: AMOUNT_BOX_WIDTH_PP, amount_box_height: AMOUNT_BOX_HEIGHT_PP}),
            ] }

    }

    pub fn render(&mut self, ops: &mut Vec<DrawOp>) {

       // self.layout.compute_spacing();

        let base_x = RECEIPT_WIDTH + MARGIN;

        let mut left_cursor = ColumnCursor::new(
            base_x,
            A4_PAGE_HEIGHT - Mm(100f32),
        );

        let mut right_cursor = ColumnCursor::new(
            base_x + PP_INFO_SECTION_HORI_OFFSET,
            A4_PAGE_HEIGHT - Mm(100f32) + self.layout.label_ascender,
        );

        for block in &self.blocks {
            match block.column() {
                Column::Left => block.render(&mut self.layout, ops, &mut left_cursor),
                Column::Right => block.render(&mut self.layout, ops, &mut right_cursor),
                Column::Absolute => block.render(&mut self.layout, ops, &mut left_cursor),
            }
        }

    }
}
