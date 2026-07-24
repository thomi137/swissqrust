/*
 * Copyright (c) 2026 Thomas Prosser
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */

//! Locks in an explicit, deliberate design decision from earlier this
//! project: scissors icons and the perforation guide are print/PDF-only
//! (Style Guide p.7 - they're cut marks for a physical printout, meaningless
//! in an on-screen preview). The SVG engine keeps only a plain divider line
//! between receipt and payment part; the PDF engine draws the real thing.
//! This was previously verified by eye against screenshots; this pins the
//! structural difference down so it can't silently regress in either
//! direction (scissors leaking into SVG, or disappearing from PDF).

use std::convert::TryFrom;
use swiss_qrust::pdf::render_bill_to_pdf;
use swiss_qrust::svg::render_bill_to_svg;
use swiss_qrust::{BillData, InputBill, Language};

fn sample_bill() -> BillData {
    let toml = std::fs::read_to_string("tests/data/valid_input/normal_slip_valid.toml").unwrap();
    let input: InputBill = toml::from_str(&toml).unwrap();
    BillData::try_from(input).unwrap()
}

#[test]
fn svg_output_is_well_formed_and_has_no_print_only_elements() {
    let bill = sample_bill();
    let svg = render_bill_to_svg(&bill, Language::De).unwrap();

    assert!(svg.starts_with("<svg"), "expected an SVG document");
    assert!(svg.trim_end().ends_with("</svg>"), "SVG document was not closed");

    // Both parts must actually be present.
    assert!(svg.contains("Empfangsschein"), "missing receipt title");
    assert!(svg.contains("Zahlteil"), "missing payment part title");

    // Exactly the plain divider (`add_perforation_marks`), nothing else
    // dashed - no scissors symbol (Zapf Dingbats), no separate top
    // perforation line.
    assert_eq!(svg.matches("<line").count(), 1, "expected exactly one divider line in the SVG preview");
    assert_eq!(svg.matches("stroke-dasharray").count(), 1);
    assert!(
        svg.contains(r#"x1="62" x2="62" y1="0" y2="105""#),
        "expected the divider to be the vertical receipt/payment-part separator"
    );
    let lowercase = svg.to_lowercase();
    assert!(!lowercase.contains("zapf"), "SVG must not reference the scissors symbol's font");
    assert!(!lowercase.contains("scissor"), "SVG must not contain a scissors element");
}

#[test]
fn pdf_output_has_the_scissors_and_perforation_marks_the_svg_omits() {
    let bill = sample_bill();
    let pdf_bytes = render_bill_to_pdf(&bill, Language::De).unwrap();

    assert!(pdf_bytes.starts_with(b"%PDF"), "expected a PDF document");
    assert!(
        pdf_bytes.windows(4).any(|w| w == b"Zapf"),
        "expected the PDF to reference the Zapf Dingbats scissors font \
         (Style Guide p.7) - the print-only counterpart to the SVG's plain divider"
    );
}
