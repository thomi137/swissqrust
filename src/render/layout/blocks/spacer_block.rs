/*
 * Copyright (c) 2026 Thomas Prosser
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */

use crate::{BillLayout, Column, ColumnCursor, DrawOp, LayoutBlock, Mm};

pub struct SpacerBlock {
    pub min_height: Mm,
}

impl LayoutBlock for SpacerBlock {

    fn column(&self) -> Column {
        Column::Left
    }

    fn render(&self, layout: &mut BillLayout, _ops: &mut Vec<DrawOp>, cursor: &mut ColumnCursor) {

        // basically clamping the cursor to the minimum height
        let dy = if cursor.y <= self.min_height {
            self.min_height - cursor.y
        } else {
            cursor.y - self.min_height
        };
        cursor.advance(dy + layout.label_ascender);
    }
}