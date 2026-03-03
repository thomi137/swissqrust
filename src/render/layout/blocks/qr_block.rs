/*
 * Copyright (c) 2026 Thomas Prosser
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */
use crate::{Baseline, DrawOp, LayoutBlock, Mm};
use crate::bill_layout::{BillLayout};
use crate::block_elements::{Column, ColumnCursor};
use crate::constants::{A4_PAGE_HEIGHT, MARGIN, RECEIPT_WIDTH};
use crate::coords::LayoutY;

pub struct QrBlock;

impl LayoutBlock for QrBlock {
    fn column(&self) -> Column {
        Column::Absolute
    }

    fn render(&self, _: &mut BillLayout, ops: &mut Vec<DrawOp>,  _: &mut ColumnCursor) {
        let x_start = RECEIPT_WIDTH + MARGIN;
        let y = A4_PAGE_HEIGHT -  Mm(42f32);
        ops.push(DrawOp::QrCodeSpace {
            at: Baseline { x: x_start, y: LayoutY(y) },
            size: Mm(46.0),
        });
    }
}
