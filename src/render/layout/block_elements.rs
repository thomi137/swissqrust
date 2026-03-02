/*
 * Copyright (c) 2026 Thomas Prosser
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */

use crate::{Mm, DrawOp};
use crate::bill_layout::BillLayout;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum SlipPart {
    PaymentPart,
    Receipt,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Column {
    Left,
    Right,
    Absolute,
}

pub struct ColumnCursor {
    pub x: Mm,
    pub y: Mm,
}

impl ColumnCursor {
    pub fn new(x: Mm, y: Mm) -> Self {
        Self { x, y }
    }

    pub fn advance(&mut self, dy: Mm) {
        self.y = self.y + dy;
    }
}

pub trait LayoutBlock {
    fn column(&self) -> Column;

    fn render(
        &self,
        layout: &mut BillLayout,
        ops: &mut Vec<DrawOp>,
        cursor: &mut ColumnCursor,
    );
}