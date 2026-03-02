/*
 * Copyright (c) 2026 Thomas Prosser 
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */
use crate::{label, Baseline, Column, DrawOp, LabelKey, LayoutBlock, Mm, FONT_SIZE_TITLE, MARGIN, SLIP_HEIGHT};
use crate::bill_layout::{BillLayout};
use crate::block_elements::ColumnCursor;
use crate::coords::LayoutY;

pub struct TitleBlock {
    pub label: LabelKey,
}
impl LayoutBlock for TitleBlock {
    fn column(&self) -> Column {
        Column::Left
    }

    fn render(&self, layout: &mut BillLayout, ops: &mut Vec<DrawOp>, cursor: &mut ColumnCursor) {

        let x = cursor.x;
        let y = cursor.y;

        let title_height = Mm(7f32);

        // small detour because title is only available when running.
        let label_text = label(self.label, layout.language)
            .unwrap_or("");
        ops.push(DrawOp::Text {
            text: label_text.to_string(),
            at: Baseline { x, y: LayoutY(y + layout.title_ascender) },
            size: FONT_SIZE_TITLE,
            bold: true,
        });

        // TODO: Smell. Actually, this should be done in the layout.
        cursor.advance(title_height + layout.label_ascender);
    }
}
