/*
 * Copyright (c) 2026 Thomas Prosser 
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */


use crate::bill_layout::{BillLayout};
use crate::{draw_corner_marks, label, Baseline, Column, DrawOp, LayoutBlock, Mm, QRBillLayoutRect, CORNER_MARKS_AMOUNT_POLYLINES, CORNER_MARKS_AMOUNT_VIEWBOX, CUCCENCY_WIDTH_PP, MARGIN};
use crate::block::ColumnCursor;
use crate::support::traits::SwissQRFormatter;

pub struct AmountBlock{
    pub amount_box_width: Mm,
    pub amount_box_height: Mm,
}
impl LayoutBlock for AmountBlock {
    fn column(&self) -> Column {
        Column::Left
    }

    fn render(&self, layout: &mut BillLayout, ops: &mut Vec<DrawOp>, cursor: &mut ColumnCursor) {
        let x = cursor.x;
        let mut y = cursor.y;

        // Currency label
        ops.push(DrawOp::Text {
            text: label!(Currency, layout.language).into(),
            at: Baseline { x, y },
            size: layout.label_font_size,
            bold: true,
        });

        let amount_x = x + MARGIN + CUCCENCY_WIDTH_PP;
        ops.push(DrawOp::Text {
            text: label!(Amount, layout.language).into(),
            at: Baseline { x: amount_x, y },
            size: layout.label_font_size,
            bold: true,
        });

        // update vertical cursor
        y = y - layout.text_ascender - layout.label_font_size.to_mm();

        // Currency text
        ops.push(DrawOp::Text {
            text: layout.bill_data.currency.to_string(),
            at: Baseline { x, y},
            size: layout.text_font_size,
            bold: false,
        });

        // Amount or box
        if let Some(amount) = &layout.bill_data.amount{
            ops.push(DrawOp::Text {
                text: amount.format_amount(),
                at: Baseline { x: amount_x, y },
                size: layout.text_font_size,
                bold: false,
            });
        } else {
            let rect = QRBillLayoutRect {
                x: amount_x,
                y:  y - self.amount_box_height + layout.text_ascender,
                width: self.amount_box_width,
                height: self.amount_box_height,
            };

            draw_corner_marks(
                ops,
                rect,
                CORNER_MARKS_AMOUNT_VIEWBOX,
                CORNER_MARKS_AMOUNT_POLYLINES
            )
        }

        cursor.y = y;
    }
}


