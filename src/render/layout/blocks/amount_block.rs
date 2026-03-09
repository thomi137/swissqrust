/*
 * Copyright (c) 2026 Thomas Prosser 
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */

use crate::{draw_corner_marks, label, Baseline, Column, DrawOp, LayoutBlock, Mm, QRBillLayoutRect, RenderContext, SlipPart, CORNER_MARKS_AMOUNT_POLYLINES, CORNER_MARKS_AMOUNT_VIEWBOX};
use crate::block_elements::ColumnCursor;
use crate::constants::{CURRENCY_WIDTH_PP, CURRENCY_WIDTH_RC};
use crate::pdf::coords::LayoutY;
use crate::render::FontMetrics;
use crate::support::traits::SwissQRFormatter;

pub struct AmountBlock{
    pub part: crate::SlipPart,
    pub amount_box_width: Mm,
    pub amount_box_height: Mm,
}
impl <T: FontMetrics> LayoutBlock<T> for AmountBlock {
    fn column(&self) -> Column {
        Column::Left
    }

    fn render(&self, ctx: &RenderContext<'_, T>, ops: &mut Vec<DrawOp>, cursor: &mut ColumnCursor) {
        let x = cursor.x;
        let y = cursor.y;

        // Currency label
        ops.push(DrawOp::Text {
            text: label!(Currency, ctx.language).into(),
            at: Baseline { x, y: LayoutY(y) },
            size: ctx.label_size,
            bold: true,
        });

        // Amount label
        let amount_x = match self.part {
            SlipPart::PaymentPart => x + CURRENCY_WIDTH_PP,
            SlipPart::Receipt => x + CURRENCY_WIDTH_RC,
        };

        ops.push(DrawOp::Text {
            text: label!(Amount, ctx.language).into(),
            at: Baseline { x: amount_x, y: LayoutY(y) },
            size: ctx.label_size,
            bold: true,
        });

        // update vertical cursor
        // TODO: Verify this is correct
        cursor.advance(ctx.line_spacing);
        let y = cursor.y;

        // Currency text
        ops.push(DrawOp::Text {
            text: ctx.bill_data.currency.to_string(),
            at: Baseline { x, y: LayoutY(y) },
            size: ctx.text_size,
            bold: false,
        });

        // Amount or box
        if let Some(amount) = &ctx.bill_data.amount{
            ops.push(DrawOp::Text {
                text: amount.format_amount(),
                at: Baseline { x: amount_x, y: LayoutY(y) },
                size: ctx.text_size,
                bold: false,
            });
            return;
        }

        let rect = amount_box_geometry(
            self.part,
            x,
            &ctx,
            self.amount_box_width,
            self.amount_box_height,
        );

        draw_corner_marks(
                ops,
                rect,
                CORNER_MARKS_AMOUNT_VIEWBOX,
                CORNER_MARKS_AMOUNT_POLYLINES
            )
        }
    }

fn amount_box_geometry<T: FontMetrics>(
    part: SlipPart,
    base_x: Mm,
    ctx: &RenderContext<T>,
    amount_box_width: Mm,
    amount_box_height: Mm,
) -> QRBillLayoutRect {
    match part {
        SlipPart::Receipt => QRBillLayoutRect {
            x: base_x + CURRENCY_WIDTH_RC + Mm(10.0),

            // TODO: Make const out of this.
            y: Mm(260f32),
            width: amount_box_width,
            height: amount_box_height,
        },

        SlipPart::PaymentPart => QRBillLayoutRect {
            x: base_x + Mm(11.0),

            //TODO: Make const out of this.
            y: Mm(260f32) + ctx.line_spacing,
            width: amount_box_width,
            height: amount_box_height,
        },
    }
}
