/*
 * Copyright (c) 2026 Thomas Prosser
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */

//! Locks in the amount placeholder box's position relative to the
//! "Währung"/"Betrag" labels above it - the exact relationship that took a
//! long, screenshot-driven back-and-forth to get right (see git history
//! around the amount-box corner-mark work). Renders through the same
//! `LayoutBlock`s the real page layout uses (not a hand-computed
//! coordinate), so a regression here means the actual rendered output
//! changed, not just this test's assumptions.

use swiss_qrust::amount_block::AmountBlock;
use swiss_qrust::constants::*;
use swiss_qrust::payment_part::PaymentPartLayout;
use swiss_qrust::receipt_part::ReceiptLayout;
use swiss_qrust::spacer_block::SpacerBlock;
use swiss_qrust::svg::SvgFontLibrary;
use swiss_qrust::{BillData, ColumnCursor, DrawOp, InputBill, Language, LayoutBlock, Mm, RenderContext, SlipPart};

fn no_amount_bill() -> BillData {
    let toml = r#"
iban = "CH93 0076 2011 6238 5295 7"
currency = "CHF"

[creditor_address]
name = "Robert Schneider AG"
street = "Rue du Lac"
house_num = "1268"
plz = "2501"
city = "Biel"
country = "CH"
"#;
    let input: InputBill = toml::from_str(toml).unwrap();
    BillData::try_from(input).unwrap()
}

fn amount_label_baseline_y(ops: &[DrawOp]) -> f32 {
    ops.iter()
        .find_map(|op| match op {
            DrawOp::Text { text, at, .. } if text == "Betrag" => Some(at.y.0 .0),
            _ => None,
        })
        .expect("Amount label (\"Betrag\") was not drawn")
}

fn min_corner_mark_y(ops: &[DrawOp]) -> f32 {
    ops.iter()
        .filter_map(|op| match op {
            DrawOp::Line { from, to, .. } => Some([(from.1).0 .0, (to.1).0 .0]),
            _ => None,
        })
        .flatten()
        .fold(f32::INFINITY, f32::min)
        .min(f32::INFINITY)
}

#[test]
fn payment_part_amount_box_top_is_one_amount_row_below_the_section_top() {
    let bill = no_amount_bill();
    let fonts = SvgFontLibrary::new();
    let ctx = RenderContext::for_strategy::<PaymentPartLayout<SvgFontLibrary>>(&bill, Language::De, &fonts);

    let mut ops = Vec::new();
    let mut cursor = ColumnCursor::new(MARGIN, A4_PAGE_HEIGHT - Mm(100.0));
    SpacerBlock { min_height: Mm(260.0) }.render(&ctx, &mut ops, &mut cursor);
    AmountBlock {
        part: SlipPart::PaymentPart,
        amount_box_width: AMOUNT_BOX_WIDTH_PP,
        amount_box_height: AMOUNT_BOX_HEIGHT_PP,
    }
    .render(&ctx, &mut ops, &mut cursor);

    let box_top_y = min_corner_mark_y(&ops);

    // PaymentPart's box starts before CURRENCY_WIDTH_PP horizontally (see
    // amount_box_geometry), i.e. underneath both labels rather than beside
    // one of them - so it can't be flush with the label top like the
    // Receipt's box; it sits exactly one PP_AMOUNT_LINE_SPACING below the
    // section top instead (see box_top_y in amount_block.rs).
    let expected_top_y = (A4_PAGE_HEIGHT - PP_AMOUNT_SECTION_TOP + PP_AMOUNT_LINE_SPACING.to_mm()).0;
    // Tolerance covers the corner-mark artwork's own ~0.11mm inset from the
    // true corner point (its tip doesn't touch the exact corner) - not
    // measurement noise, but it means `min_corner_mark_y` is never quite
    // bit-exact to `box_top_y` itself.
    assert!(
        (box_top_y - expected_top_y).abs() < 0.15,
        "payment-part amount box top (y={box_top_y}) does not match \
         AMOUNT_SECTION_TOP + one amount-row-spacing (expected y={expected_top_y})"
    );

    // And the spec-meaningful consequence of that: it must not overlap the
    // "Währung"/"Betrag" labels, i.e. sit at or below their baseline.
    let label_baseline_y = amount_label_baseline_y(&ops);
    assert!(
        box_top_y >= label_baseline_y,
        "payment-part amount box top (y={box_top_y}) sits above the Betrag \
         label's baseline (y={label_baseline_y}) - it will overlap the label"
    );
}

#[test]
fn receipt_amount_box_top_is_flush_with_the_betrag_label_ascender() {
    let bill = no_amount_bill();
    let fonts = SvgFontLibrary::new();
    let ctx = RenderContext::for_strategy::<ReceiptLayout<SvgFontLibrary>>(&bill, Language::De, &fonts);

    let mut ops = Vec::new();
    let mut cursor = ColumnCursor::new(MARGIN, A4_PAGE_HEIGHT - Mm(100.0));
    SpacerBlock { min_height: Mm(260.0) }.render(&ctx, &mut ops, &mut cursor);
    AmountBlock {
        part: SlipPart::Receipt,
        amount_box_width: AMOUNT_BOX_WIDTH_RC,
        amount_box_height: AMOUNT_BOX_HEIGHT_RC,
    }
    .render(&ctx, &mut ops, &mut cursor);

    let label_baseline_y = amount_label_baseline_y(&ops);
    let box_top_y = min_corner_mark_y(&ops);

    // Style Guide requirement ("Fluchten mit der Currency top"): the
    // Receipt's box - which starts well past CURRENCY_WIDTH_RC, clear of
    // the "Betrag" label horizontally - must align exactly with the *top*
    // of the label glyphs, i.e. one ascender's worth above the baseline.
    // "somewhere above the baseline" isn't a strong enough assertion: this
    // pins the exact, spec-required offset instead of just its sign.
    let expected_top_y = label_baseline_y - ctx.label_ascender.0;
    // Tolerance covers the corner-mark artwork's own ~0.11mm inset from the
    // true corner point (its tip doesn't touch the exact corner) - not
    // measurement noise, but it means `min_corner_mark_y` is never quite
    // bit-exact to `box_top_y` itself.
    assert!(
        (box_top_y - expected_top_y).abs() < 0.15,
        "receipt amount box top (y={box_top_y}) is not flush with the \
         Betrag label's ascender top (expected y={expected_top_y})"
    );
}
