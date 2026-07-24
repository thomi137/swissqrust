/*
 * Copyright (c) 2026 Thomas Prosser
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */

use crate::block_elements::{Column, ColumnCursor, LayoutBlock};
use crate::constants::{A4_PAGE_HEIGHT, A4_PAGE_WIDTH, MARGIN, RECEIPT_WIDTH, PP_FURTHER_INFO_LINE_SPACING, PP_FURTHER_INFO_SECTION_TOP, PP_FURTHER_INFO_TEXT_SIZE};
use crate::pdf::coords::LayoutY;
use crate::render::FontMetrics;
use crate::{truncate_to_width, Baseline, DrawOp, FontStyle, Mm, RenderContext};

/// Style Guide 3.5.5 "Further information section" / "Weitere
/// Informationen": up to two "Alternative procedures" lines at the bottom
/// of the payment part, 7pt, with the (short) procedure name in bold. The
/// receipt has no such section.
pub struct FurtherInformationBlock;

impl<T: FontMetrics> LayoutBlock<T> for FurtherInformationBlock {
    fn column(&self) -> Column {
        Column::Absolute
    }

    fn render(&self, ctx: &RenderContext<'_, T>, ops: &mut Vec<DrawOp>, _cursor: &mut ColumnCursor) {
        let x = RECEIPT_WIDTH + MARGIN;
        let available_width = A4_PAGE_WIDTH - x - MARGIN;
        let row_top = A4_PAGE_HEIGHT - PP_FURTHER_INFO_SECTION_TOP
            + ctx.metrics.ascender_mm(FontStyle::Bold, PP_FURTHER_INFO_TEXT_SIZE);

        for (i, scheme) in ctx.bill_data.alternative_schemes.iter().flatten().enumerate() {
            let y = row_top + Mm(i as f32) * PP_FURTHER_INFO_LINE_SPACING.to_mm();
            draw_alt_procedure_line(ops, ctx, scheme, x, y, available_width);
        }
    }
}

/// Draws one Alternative-procedure line: the part up to and including the
/// first separator ("eBill/" in "eBill/B/simon.muster@example.com") in
/// bold, the rest in regular weight, truncated with "…" if it would
/// otherwise overflow `available_width`.
fn draw_alt_procedure_line<T: FontMetrics>(
    ops: &mut Vec<DrawOp>,
    ctx: &RenderContext<'_, T>,
    scheme: &str,
    x: Mm,
    y: Mm,
    available_width: Mm,
) {
    let (name, rest) = match scheme.find('/') {
        Some(idx) => scheme.split_at(idx + 1),
        None => (scheme, ""),
    };

    let name_width = ctx.metrics.text_width_mm(name, FontStyle::Bold, PP_FURTHER_INFO_TEXT_SIZE);
    let rest = truncate_to_width(
        ctx.metrics,
        rest,
        FontStyle::Regular,
        PP_FURTHER_INFO_TEXT_SIZE,
        available_width - name_width,
    );

    ops.push(DrawOp::Text {
        text: name.to_string(),
        at: Baseline { x, y: LayoutY(y) },
        size: PP_FURTHER_INFO_TEXT_SIZE,
        bold: true,
    });

    if !rest.is_empty() {
        ops.push(DrawOp::Text {
            text: rest,
            at: Baseline { x: x + name_width, y: LayoutY(y) },
            size: PP_FURTHER_INFO_TEXT_SIZE,
            bold: false,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::layout::payment_part::PaymentPartLayout;
    use crate::render::engines::svg::fonts::SvgFontLibrary;
    use crate::{BillData, InputBill, Language, RenderContext};

    fn ctx_and_fonts() -> (BillData, SvgFontLibrary) {
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
        (BillData::try_from(input).unwrap(), SvgFontLibrary::new())
    }

    #[test]
    fn splits_bold_name_from_regular_rest_at_first_slash() {
        let (bill, fonts) = ctx_and_fonts();
        let ctx = RenderContext::for_strategy::<PaymentPartLayout<SvgFontLibrary>>(&bill, Language::De, &fonts);

        let mut ops = Vec::new();
        draw_alt_procedure_line(&mut ops, &ctx, "eBill/B/simon.muster@example.com", Mm(0.0), Mm(0.0), Mm(100.0));

        assert_eq!(ops.len(), 2, "expected a bold name segment and a regular rest segment");
        let DrawOp::Text { text: name, bold: name_bold, .. } = &ops[0] else { panic!("expected Text") };
        let DrawOp::Text { text: rest, bold: rest_bold, .. } = &ops[1] else { panic!("expected Text") };
        assert_eq!(name, "eBill/");
        assert!(name_bold);
        assert_eq!(rest, "B/simon.muster@example.com");
        assert!(!rest_bold);
    }

    #[test]
    fn scheme_without_a_slash_is_drawn_as_bold_name_only() {
        let (bill, fonts) = ctx_and_fonts();
        let ctx = RenderContext::for_strategy::<PaymentPartLayout<SvgFontLibrary>>(&bill, Language::De, &fonts);

        let mut ops = Vec::new();
        draw_alt_procedure_line(&mut ops, &ctx, "NoSlashHere", Mm(0.0), Mm(0.0), Mm(100.0));

        // No second (regular-weight) segment should be pushed when there's
        // nothing left after the name - an empty DrawOp::Text would still
        // "work" but is a wasted/misleading draw op.
        assert_eq!(ops.len(), 1, "expected only the bold name segment, no empty rest segment");
        let DrawOp::Text { text: name, bold: name_bold, .. } = &ops[0] else { panic!("expected Text") };
        assert_eq!(name, "NoSlashHere");
        assert!(name_bold);
    }

    #[test]
    fn rest_is_truncated_to_fit_available_width() {
        let (bill, fonts) = ctx_and_fonts();
        let ctx = RenderContext::for_strategy::<PaymentPartLayout<SvgFontLibrary>>(&bill, Language::De, &fonts);

        let long_rest = "a-very-long-identifier-that-will-not-fit-in-a-narrow-column@example.com";
        let scheme = format!("eBill/{long_rest}");

        let mut ops = Vec::new();
        // Deliberately narrow: enough for the bold name, not for the rest.
        draw_alt_procedure_line(&mut ops, &ctx, &scheme, Mm(0.0), Mm(0.0), Mm(20.0));

        let DrawOp::Text { text: rest, .. } = &ops[1] else { panic!("expected a truncated rest segment") };
        assert!(rest.ends_with('…'), "expected truncation ellipsis, got {rest:?}");
        assert!(rest.len() < long_rest.len());
    }
}
