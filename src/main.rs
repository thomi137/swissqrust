/*
 * Copyright (c) 2026 Thomas Prosser
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */

use pdf_writer::{Content, Pdf, Rect, Ref, Finish};

use swiss_qrust::{Address, Currency, Language, QRCountry, ReferenceType};
use swiss_qrust::BillData;
use swiss_qrust::render::engines::pdf::*;
use swiss_qrust::render::layout::*;
use swiss_qrust::constants::*;
use swiss_qrust::render::engines::pdf::builder::execute_bill_ops;
use swiss_qrust::qr_bill::{encode_text_to_qr_code, QrBill};
use swiss_qrust::render::layout::receipt_part::ReceiptLayout;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let  creditor = Address::new(
        "Health insurance fit&kicking",
        Some("Am Wasser"),
        Some("1"),
        "3000",
        "Bern",
        "CH"
    )?;

    let debtor = Address::new(
        "Sarah Beispiel",
        Some("Mustergasse"),
        Some("1"),
        "3600",
        "Thun",
        "CH"
    )?;

    let bill_data = BillData::new(
        "CH64 3196 1000 0044 2155 7".to_string(),
        creditor,
        None,//Some(debtor),
        QRCountry::CH,
        Currency::CHF,
        Some(String::from("32111.00")),
        ReferenceType::infer("000008207791225857421286694")?,
        Some(String::from("Premium calculation July 2020")),
        None
    )?;

    create_test_slip_pdf("./examples/test_receipt.pdf", &bill_data)?;

    Ok(())
}

pub fn create_test_slip_pdf(path: &str, bill_data: &BillData) -> Result<(), Box<dyn std::error::Error>> {
    let mut pdf = Pdf::new();
    let mut next_id = Ref::new(1);

    let catalog_id = next_id.bump();
    let page_tree_id = next_id.bump();
    let page_id = next_id.bump();
    let content_id = next_id.bump();

    // 1. Initialize Fonts
    let fonts = FontLibrary::new(&mut pdf, &mut next_id);
    let regular_ascender = fonts.regular.face.ascender() as f32 / fonts.regular.face.units_per_em() as f32;
    let bold_ascender = fonts.bold.face.ascender() as f32 / fonts.bold.face.units_per_em() as f32;

    let zapf_id = next_id.bump();
    pdf.type1_font(zapf_id).base_font(pdf_writer::Name(b"ZapfDingbats"));

    // Generate Layout Ops
    let mut ops = Vec::new();

    let mut payment_part_layout = PaymentPartLayout::new(
        bill_data,
        PAYMENT_PART_HORI_OFFSET + MARGIN,
        Mm(95.0),
        Language::De,
        PP_LABEL_PREF_FONT_SIZE,
        PP_TEXT_PREF_FONT_SIZE,
        Mm(bold_ascender),
        Mm(regular_ascender),
        Mm(3.5),
        Mm(2.0),
    );
    payment_part_layout.render(&mut ops, &fonts);

    let mut receipt_layout = ReceiptLayout::new(
        bill_data,
        Mm(5.0),
        Mm(95.0),
        Language::De,
        Pt(6.0),
        Pt(8.0),
        Mm(bold_ascender),
        Mm(regular_ascender),
        Mm(3.5),
        Mm(2.0),
    );
    receipt_layout.render(&mut ops, &fonts);

    // Write Page and Resources
    pdf.catalog(catalog_id).pages(page_tree_id);
    pdf.pages(page_tree_id).kids([page_id]).count(1);

    // Create A4 Page
    let mut page = pdf.page(page_id);
    page.media_box(Rect::new(0.0, 0.0, 595.28, 842.89)); // A4
    page.parent(page_tree_id);
    page.contents(content_id);

    let mut res = page.resources();
    let mut f_dict = res.fonts();
    f_dict.pair(pdf_writer::Name(b"Zapf"), zapf_id);
    f_dict.pair(name(FontStyle::Regular), fonts.regular.type0_ref);
    f_dict.pair(name(FontStyle::Bold), fonts.bold.type0_ref);
    f_dict.finish();
    res.finish();
    page.finish();

    let mut content = Content::new();

    // 1. Horizontal Perforation (Top of Bill)
    content.save_state();
    content.set_dash_pattern([3.0, 3.0], 0.0);
    content.set_line_width(0.75);

    let y_sep = 105.0 * PT_PER_MM;
    content.move_to(0.0, y_sep);
    content.line_to(210.0 * PT_PER_MM, y_sep);
    content.stroke();

    draw_scissors_official(&mut content, 64.8, y_sep, 180.0);
    content.restore_state();

    // 2. Vertical Separation (Receipt/Payment Part)
    content.save_state();
    content.set_dash_pattern([3.0, 3.0], 0.0);
    content.set_line_width(0.75);

    let x_sep = 62.0 * PT_PER_MM;
    content.move_to(x_sep, 0.0);
    content.line_to(x_sep, 105.0 * PT_PER_MM);
    content.stroke();

    draw_scissors_official(&mut content, x_sep, 80.0 * PT_PER_MM, 90.0);
    content.restore_state();

    let qr_code = QrBill::new(bill_data)
        .and_then(|b| b.create_qr_text())
        .and_then(|txt| encode_text_to_qr_code(&txt))
        .ok();

    execute_bill_ops(&mut content, &fonts, ops, qr_code.as_ref());
    pdf.stream(content_id, &content.finish());

    std::fs::write(path, pdf.finish())?;
    Ok(())
}

fn draw_scissors_official(content: &mut Content, x: f32, y: f32, rotation_deg: f32) {
    content.save_state();

    content.transform([1.0, 0.0, 0.0, 1.0, x, y]);

    let rad = rotation_deg.to_radians();
    content.transform([rad.cos(), rad.sin(), -rad.sin(), rad.cos(), 0.0, 0.0]);

    content.set_fill_rgb(1.0, 1.0, 1.0);
    content.rect(-11.5, -9.0, 11.5, 9.5);
    content.fill_nonzero();

    content.set_fill_rgb(0.0, 0.0, 0.0);
    content.begin_text();
    content.set_font(pdf_writer::Name(b"Zapf"), 13.0);
    content.set_text_matrix([1.0, 0.0, 0.0, 1.0, -13.5, -4.5]);
    content.show(pdf_writer::Str(b"\x22"));
    content.end_text();

    content.restore_state();
}
