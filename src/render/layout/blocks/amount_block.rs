/*
 * Copyright (c) 2026 Thomas Prosser 
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */

use crate::{draw_corner_marks, label, Baseline, Column, DrawOp, LayoutBlock, Mm, QRBillLayoutRect, RenderContext, SlipPart, CORNER_MARKS_AMOUNT_POLYLINES, CORNER_MARKS_AMOUNT_VIEWBOX};
use crate::block_elements::ColumnCursor;
use crate::constants::{A4_PAGE_HEIGHT, AMOUNT_SECTION_TOP, PP_AMOUNT_SECTION_TOP, CURRENCY_WIDTH_PP, CURRENCY_WIDTH_RC, RC_AMOUNT_LINE_SPACING, PP_AMOUNT_LINE_SPACING};
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
        let label_row_y = cursor.y;

        // Currency label
        ops.push(DrawOp::Text {
            text: label!(Currency, ctx.language).into(),
            at: Baseline { x, y: LayoutY(label_row_y) },
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
            at: Baseline { x: amount_x, y: LayoutY(label_row_y) },
            size: ctx.label_size,
            bold: true,
        });

        // Betrag E/Z use a dedicated line spacing (Style Guide p.15), distinct
        // from the general body line spacing used elsewhere in the section.
        let amount_line_spacing = match self.part {
            SlipPart::Receipt => RC_AMOUNT_LINE_SPACING.to_mm(),
            SlipPart::PaymentPart => PP_AMOUNT_LINE_SPACING.to_mm(),
        };
        cursor.advance(amount_line_spacing);
        let value_row_y = cursor.y;

        // Currency text
        ops.push(DrawOp::Text {
            text: ctx.bill_data.currency.to_string(),
            at: Baseline { x, y: LayoutY(value_row_y) },
            size: ctx.text_size,
            bold: false,
        });

        // Amount or box
        if let Some(amount) = &ctx.bill_data.amount{
            ops.push(DrawOp::Text {
                text: amount.format_amount(),
                at: Baseline { x: amount_x, y: LayoutY(value_row_y) },
                size: ctx.text_size,
                bold: false,
            });
            return;
        }

        // Box top is anchored to the spec's absolute AMOUNT_SECTION_TOP mm
        // constant, not derived from the label row's baseline/ascender. The
        // Receipt's box sits flush with that section top; the PaymentPart's
        // sits one amount-row-spacing below it, level with the value row.
        let box_top_y = match self.part {
            SlipPart::Receipt => A4_PAGE_HEIGHT - AMOUNT_SECTION_TOP,
            SlipPart::PaymentPart => A4_PAGE_HEIGHT - PP_AMOUNT_SECTION_TOP + amount_line_spacing,
        };
        let rect = amount_box_geometry(
            self.part,
            x,
            box_top_y,
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

/// `y` is the top of the box, flush with the top of the Currency/Amount
/// labels above it (see call site).
fn amount_box_geometry(
    part: SlipPart,
    base_x: Mm,
    y: Mm,
    amount_box_width: Mm,
    amount_box_height: Mm,
) -> QRBillLayoutRect {
    match part {
        SlipPart::Receipt => QRBillLayoutRect {
            x: base_x + CURRENCY_WIDTH_RC + Mm(10.0),
            y,
            width: amount_box_width,
            height: amount_box_height,
        },

        SlipPart::PaymentPart => QRBillLayoutRect {
            x: base_x + Mm(11.0),
            y,
            width: amount_box_width,
            height: amount_box_height,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::PP_AMOUNT_LINE_SPACING;

    #[test]
    fn receipt_box_starts_past_the_betrag_label() {
        let base_x = Mm(5.0);
        let rect = amount_box_geometry(SlipPart::Receipt, base_x, Mm(100.0), Mm(30.0), Mm(10.0));

        // Must clear CURRENCY_WIDTH_RC (where the "Betrag" label starts) -
        // this box sits *beside* the label, not below it (see box_top_y:
        // Receipt is flush with the label row).
        assert!(rect.x > base_x + CURRENCY_WIDTH_RC);
        assert_eq!(rect.x, base_x + CURRENCY_WIDTH_RC + Mm(10.0));
        assert_eq!(rect.width, Mm(30.0));
        assert_eq!(rect.height, Mm(10.0));
    }

    #[test]
    fn payment_part_box_starts_before_the_betrag_label() {
        let base_x = Mm(67.0);
        let rect = amount_box_geometry(SlipPart::PaymentPart, base_x, Mm(100.0), Mm(40.0), Mm(15.0));

        // Unlike the Receipt, this box starts *before* CURRENCY_WIDTH_PP
        // (where "Betrag" starts) - it sits below both labels, not beside
        // one of them. box_top_y accounts for that by anchoring the
        // PaymentPart box to the value row instead of the label row.
        assert!(rect.x < base_x + CURRENCY_WIDTH_PP);
        assert_eq!(rect.x, base_x + Mm(11.0));
        assert_eq!(rect.width, Mm(40.0));
        assert_eq!(rect.height, Mm(15.0));
    }

    /// `SpacerBlock { min_height: Mm(260) }` (in `ReceiptLayout`/
    /// `PaymentPartLayout`) is a bare literal, not derived from
    /// `AMOUNT_SECTION_TOP` in code - the two must be kept numerically
    /// consistent by hand. This pins that relationship down so a future
    /// change to either constant fails loudly here instead of silently
    /// desyncing the cursor from the spec's absolute section-top position.
    #[test]
    fn amount_section_top_matches_spacer_block_clamp_height() {
        assert_eq!(A4_PAGE_HEIGHT - AMOUNT_SECTION_TOP, Mm(260.0));
        assert_eq!(A4_PAGE_HEIGHT - PP_AMOUNT_SECTION_TOP, Mm(260.0));
    }

    #[test]
    fn payment_part_box_sits_one_row_below_the_receipt_reference_point() {
        // Both parts share the same AMOUNT_SECTION_TOP anchor; the
        // PaymentPart's box_top_y formula adds one amount-row-spacing on
        // top of it (see box_top_y in `render` above), the Receipt's
        // doesn't. Assert that relationship directly so a future edit that
        // makes them symmetric again (as happened earlier this project) is
        // caught by a test instead of a rendered PDF.
        let receipt_top = A4_PAGE_HEIGHT - AMOUNT_SECTION_TOP;
        let payment_part_top = A4_PAGE_HEIGHT - PP_AMOUNT_SECTION_TOP + PP_AMOUNT_LINE_SPACING.to_mm();
        assert_eq!(payment_part_top, receipt_top + PP_AMOUNT_LINE_SPACING.to_mm());
        assert!(payment_part_top > receipt_top);
    }
}
