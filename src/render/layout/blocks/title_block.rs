/*
 * Copyright (c) 2026 Thomas Prosser 
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */
use crate::{label, Baseline, Column, DrawOp, LabelKey, LayoutBlock, Mm, FONT_SIZE_TITLE, MARGIN, SLIP_HEIGHT};
use crate::bill_layout::{BillLayout};
use crate::block::ColumnCursor;

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

        let baseline_y = cursor.y;

        let baseline_y = y ;

        let title_height = Mm(7f32);
        let pdf_y = SLIP_HEIGHT  - baseline_y;

        // small detour because title is only available when running.
        let label_text = label(self.label, layout.language)
            .unwrap_or("");
        ops.push(DrawOp::Text {
            text: label_text.to_string(),
            at: Baseline { x, y: pdf_y },
            size: FONT_SIZE_TITLE,
            bold: true,
        });

        cursor.advance(title_height);
    }
}
