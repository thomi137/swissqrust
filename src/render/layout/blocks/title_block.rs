/*
 * Copyright (c) 2026 Thomas Prosser 
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */
use crate::{label, Baseline, DrawOp, LabelKey, FONT_SIZE_TITLE};
use crate::layout::bill_layout::{BillLayout, Column, LayoutBlock, LayoutCursor};
use crate::layout::block::ColumnCursor;

pub struct TitleBlock {
    pub label: LabelKey,
}
impl LayoutBlock for TitleBlock {
    fn column(&self) -> Column {
        Column::Left
    }

    fn render(&self, layout: &mut BillLayout, ops: &mut Vec<DrawOp>, cursor: &mut ColumnCursor) {
        let x = layout.horizontal_offset;
        let y = cursor.y - (FONT_SIZE_TITLE.to_mm());

        // small detour because title is only available when running.
        let label_text = label(self.label, layout.language)
            .unwrap_or("");
        ops.push(DrawOp::Text {
            text: label_text.to_string(),
            at: Baseline { x, y },
            size: FONT_SIZE_TITLE,
            bold: true,
        });
    }
}
