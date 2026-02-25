/*
 * Copyright (c) 2026 Thomas Prosser
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */

use crate::render::layout::bill_layout::{BillLayout, BillLayoutConfig};
use crate::render::layout::geometry::*;
use crate::render::types::DrawOp;
use crate::{BillData, FontLibrary, Language};
use crate::constants::*;
use crate::layout::bill_layout::Column;
use crate::layout::block::{ColumnCursor, LayoutBlock};
use crate::layout::blocks::amount_block::AmountBlock;
use crate::layout::blocks::information_block::InformationBlock;
use crate::layout::blocks::qr_block::QrBlock;
use crate::layout::blocks::title_block::TitleBlock;

pub struct PaymentPartLayout<'a> {
    layout: BillLayout<'a>,
    blocks: Vec<Box<dyn LayoutBlock>>
}

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
        let layout = BillLayout {
            bill_data,
            config: BillLayoutConfig {
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
        };
        Self {
            layout,
            blocks: vec![
                Box::new(TitleBlock { label: crate::LabelKey::PaymentPart }),
                Box::new(InformationBlock {offset: PP_INFO_SECTION_HORI_OFFSET, payable_box_width: DEBTOR_BOX_WIDTH_PP, payable_box_height: DEBTOR_BOX_HEIGHT}),
                Box::new(AmountBlock{amount_box_width: AMOUNT_BOX_WIDTH_PP, amount_box_height: AMOUNT_BOX_HEIGHT_PP}),
                Box::new( QrBlock ),
            ] }

    }

    pub fn render(&mut self, ops: &mut Vec<DrawOp>, _fonts: &FontLibrary) {

        self.layout.compute_spacing();

       let mut left_cursor = ColumnCursor {
           x: RECEIPT_WIDTH + MARGIN,
           y: self.layout.top_start,
       };

        let mut right_cursor = ColumnCursor::new(
            RECEIPT_WIDTH + MARGIN + PP_INFO_SECTION_HORI_OFFSET,
            self.layout.top_start,
        );

        for block in &self.blocks {
            for block in &self.blocks {
                match block.column() {
                    Column::Left => block.render(&mut self.layout, ops, &mut left_cursor),
                    Column::Right => block.render(&mut self.layout, ops, &mut right_cursor),
                    Column::Absolute => block.render(&mut self.layout, ops, &mut left_cursor),
                }
            }
        }

    }
}
