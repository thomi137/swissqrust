/*
 * Copyright (c) 2026 Thomas Prosser
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */
use crate::{Baseline, DrawOp, Mm, MARGIN, QR_CODE_HEIGHT, RECEIPT_WIDTH, SLIP_HEIGHT};
use crate::layout::bill_layout::{BillLayout, LayoutBlock,};
use crate::layout::block::{Column, ColumnCursor};

pub struct QrBlock;

impl LayoutBlock for QrBlock {
    fn column(&self) -> Column {
        Column::Absolute
    }

    fn render(&self, _: &mut BillLayout, ops: &mut Vec<DrawOp>,  _: &mut ColumnCursor) {
        let x_start = RECEIPT_WIDTH + MARGIN;
        let y = SLIP_HEIGHT - MARGIN - Mm(7f32) - MARGIN - Mm(QR_CODE_HEIGHT);
        ops.push(DrawOp::QrCodeSpace {
            at: Baseline { x: x_start, y },
            size: Mm(46.0),
        });
    }
}
