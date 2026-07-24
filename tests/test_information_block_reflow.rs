/*
 * Copyright (c) 2026 Thomas Prosser
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */

//! Style Guide p.15: "Da es keine fixen Platzierungen der einzelnen
//! Textblöcke gibt, rückt bei fehlenden Textblöcken jeweils alles nach
//! oben" - a block with no content is omitted entirely, not left as blank
//! space. This was previously only asserted by eye; these tests render
//! through the real `InformationBlock` and check the *observable*
//! consequence (how far the cursor moves, whether a box or text gets
//! drawn) instead of re-deriving the expected numbers by hand.

use swiss_qrust::block_elements::{Column, ColumnCursor, LayoutBlock};
use swiss_qrust::constants::*;
use swiss_qrust::information_block::InformationBlock;
use swiss_qrust::payment_part::PaymentPartLayout;
use swiss_qrust::svg::SvgFontLibrary;
use swiss_qrust::{BillData, Currency, DrawOp, Language, Mm, ReferenceType, RenderContext, SlipPart};

mod common;
use common::*;

fn bill_without_debtor() -> BillData {
    let creditor_address = crdt_address();
    let iban = "CH5800791123000889012";
    BillData::new(
        iban.to_string(),
        creditor_address,
        None,
        Currency::CHF,
        Some(String::from("100.00")),
        ReferenceType::infer("").unwrap(),
        None,
        None,
        [None, None],
    )
    .unwrap()
}

fn render_information_block(bill: &BillData) -> (Vec<DrawOp>, f32) {
    let fonts = SvgFontLibrary::new();
    let ctx = RenderContext::for_strategy::<PaymentPartLayout<SvgFontLibrary>>(bill, Language::De, &fonts);
    let mut ops = Vec::new();
    let mut cursor = ColumnCursor::new(Mm(0.0), Mm(0.0));

    let block = InformationBlock {
        part: SlipPart::PaymentPart,
        offset: PP_INFO_SECTION_HORI_OFFSET,
        payable_box_width: DEBTOR_BOX_WIDTH_PP,
        payable_box_height: DEBTOR_BOX_HEIGHT,
    };
    assert_eq!(<InformationBlock as LayoutBlock<SvgFontLibrary>>::column(&block), Column::Right);
    block.render(&ctx, &mut ops, &mut cursor);
    (ops, cursor.y.0)
}

#[test]
fn omitting_the_reference_moves_everything_below_it_up() {
    // Identical bills (same creditor/debtor/message) except one has a
    // reference and the other doesn't - isolates the reference block's
    // own contribution to the vertical flow.
    let with_ref = bill_data_scor_ref();
    let without_ref = bill_data_non_ref();
    assert_eq!(with_ref.debtor_address, without_ref.debtor_address);
    assert_eq!(with_ref.unstructured_message, without_ref.unstructured_message);
    assert!(matches!(with_ref.reference_type, ReferenceType::Creditor(_)));
    assert!(matches!(without_ref.reference_type, ReferenceType::NoRef));

    let (_, y_with_ref) = render_information_block(&with_ref);
    let (_, y_without_ref) = render_information_block(&without_ref);

    assert!(
        y_without_ref < y_with_ref,
        "omitting the reference block should move the cursor up (y_without_ref={y_without_ref} \
         should be < y_with_ref={y_with_ref}), not leave its space blank"
    );
}

#[test]
fn debtor_present_draws_address_text_not_a_placeholder_box() {
    let bill = bill_data_scor_ref(); // has Some(debtor_address)
    let (ops, _) = render_information_block(&bill);

    let has_debtor_text = ops.iter().any(|op| matches!(op, DrawOp::Text { text, .. } if text.contains("Sarah Beispiel")));
    let has_corner_marks = ops.iter().any(|op| matches!(op, DrawOp::Line { .. }));

    assert!(has_debtor_text, "expected the debtor's name to be drawn as text");
    assert!(!has_corner_marks, "a known debtor address should not draw a placeholder box");
}

#[test]
fn debtor_absent_draws_a_placeholder_box_not_address_text() {
    let bill = bill_without_debtor();
    let (ops, _) = render_information_block(&bill);

    let has_address_text = ops.iter().any(|op| matches!(op, DrawOp::Text { text, .. } if text.contains("Sarah Beispiel")));
    let has_corner_marks = ops.iter().any(|op| matches!(op, DrawOp::Line { .. }));

    assert!(!has_address_text, "there is no debtor address to draw");
    assert!(has_corner_marks, "expected a placeholder box (corner marks) when the debtor address is unknown");
}
