/*
 * Copyright (c) 2026 Thomas Prosser
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */
use crate::{Baseline, DrawOp, LayoutBlock, Mm, A4_PAGE_HEIGHT, MARGIN, QR_CODE_HEIGHT, RECEIPT_WIDTH, SLIP_HEIGHT};
use crate::bill_layout::{BillLayout};
use crate::block::{Column, ColumnCursor};

pub struct QrBlock;

impl LayoutBlock for QrBlock {
    fn column(&self) -> Column {
        Column::Absolute
    }

    fn render(&self, _: &mut BillLayout, ops: &mut Vec<DrawOp>,  _: &mut ColumnCursor) {
        let x_start = RECEIPT_WIDTH + MARGIN;
        let y = A4_PAGE_HEIGHT - Mm(209f32);
        ops.push(DrawOp::QrCodeSpace {
            at: Baseline { x: x_start, y },
            size: Mm(46.0),
        });
    }
}
