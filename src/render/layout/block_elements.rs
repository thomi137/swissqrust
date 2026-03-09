/*
 * Copyright (c) 2026 Thomas Prosser
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */

use crate::{Mm, DrawOp, RenderContext};
use crate::render::FontMetrics;

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

pub trait LayoutBlock<T: FontMetrics> {
    fn column(&self) -> Column;

    fn render(
        &self,
        ctx: &RenderContext<'_, T>,
        ops: &mut Vec<DrawOp>,
        cursor: &mut ColumnCursor,
    );
}