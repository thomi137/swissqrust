/*
 * Copyright (c) 2026 Thomas Prosser 
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */


use crate::bill_layout::{BillLayout};
use crate::{draw_corner_marks, label, Baseline, Column, DrawOp, LayoutBlock, Mm, QRBillLayoutRect, SlipPart, CORNER_MARKS_AMOUNT_POLYLINES, CORNER_MARKS_AMOUNT_VIEWBOX, CURRENCY_WIDTH_PP, CURRENCY_WIDTH_RC};
use crate::block_elements::ColumnCursor;
use crate::coords::LayoutY;
use crate::support::traits::SwissQRFormatter;

pub struct AmountBlock{
    pub part: crate::SlipPart,
    pub amount_box_width: Mm,
    pub amount_box_height: Mm,
}
impl LayoutBlock for AmountBlock {
    fn column(&self) -> Column {
        Column::Left
    }

    fn render(&self, layout: &mut BillLayout, ops: &mut Vec<DrawOp>, cursor: &mut ColumnCursor) {
        let x = cursor.x;
        let y = cursor.y;

        // Currency label
        ops.push(DrawOp::Text {
            text: label!(Currency, layout.language).into(),
            at: Baseline { x, y: LayoutY(y) },
            size: layout.label_font_size,
            bold: true,
        });

        // Amount label
        let mut amount_x = if self.part == SlipPart::PaymentPart { x + CURRENCY_WIDTH_PP } else { x + CURRENCY_WIDTH_RC };
        ops.push(DrawOp::Text {
            text: label!(Amount, layout.language).into(),
            at: Baseline { x: amount_x, y: LayoutY(y) },
            size: layout.label_font_size,
            bold: true,
        });

        // update vertical cursor
        // TODO: Verify this is correct
        cursor.advance(layout.line_spacing);

        let mut y = cursor.y;
        // Currency text
        ops.push(DrawOp::Text {
            text: layout.bill_data.currency.to_string(),
            at: Baseline { x, y: LayoutY(y) },
            size: layout.text_font_size,
            bold: false,
        });

        // Amount or box
        if let Some(amount) = &layout.bill_data.amount{
            ops.push(DrawOp::Text {
                text: amount.format_amount(),
                at: Baseline { x: amount_x, y: LayoutY(y) },
                size: layout.text_font_size,
                bold: false,
            });
        } else if self.part == SlipPart::Receipt {
            amount_x = amount_x + Mm(10f32);
            y = Mm(260f32);
        } else if self.part == SlipPart::PaymentPart {
            amount_x = amount_x - CURRENCY_WIDTH_PP + Mm(11f32);
            y = Mm(260f32) + layout.line_spacing;
        } else { panic!("Invalid slip part. Only Receipt and PaymentPart are allowed.")}

        let rect = QRBillLayoutRect {
            x: amount_x,
            y,
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
    }

