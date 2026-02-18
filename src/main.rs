
use swiss_qrust::cli::*;
use clap::Parser;
use pdf_writer::{Content, Pdf, Rect, Ref, Finish};
use swiss_qrust::{pdf, qr_bill, Address, Currency, Language, QRCountry, ReferenceType};
use swiss_qrust::BillData;
use swiss_qrust::pdf::*;
use swiss_qrust::layout::*;
use swiss_qrust::pdf_builder::execute_receipt_ops;
use swiss_qrust::qr_bill::QrBill;
use swiss_qrust::receipt_part_layout::ReceiptLayout;
use swiss_qrust::svg::{add_swiss_cross, render_qr_svg};

fn main() -> Result<(), Box<dyn std::error::Error>> {

    const OUTPUT_PATH: &str = "examples/";

    let  creditor = Address::new(
        "Health insurance fit&kicking",
        Some("Am Wasser"),
        Some("1"),
        "3000",
        "Bern",
        "CH"
    ).unwrap();

    let debtor =  Address::new(
        "Sarah Beispiel",
        Some("Mustergasse"),
        Some("1"),
        "3600",
        "Thun",
        "CH").unwrap();

let bill_data = BillData::new(
    "CH64 3196 1000 0044 2155 7".to_string(),
    creditor,
    Some(debtor),
    QRCountry::CH,
    Currency::CHF,
    Some(String::from("111.00")),
    ReferenceType::infer("000008207791225857421286694")?,
    Some(String::from("Premium calculation July 2020")),
    None
    ).unwrap();

    create_test_receipt_pdf("./examples/test_receipt.pdf", &bill_data);

    // pdf_builder::base_layout(&format!("{}/test_pdf.pdf", OUTPUT_PATH));
    
    /*
    let qr_bill = QrBill::new(bill_data).unwrap();
    let qr_text = QrBill::create_qr_text(&qr_bill).unwrap();
    let qr_code = qr_bill::encode_text_to_qr_code(&qr_text).unwrap();
    let doc = render_qr_svg(qr_code);
    let doc_cross = add_swiss_cross(doc);
    render_svg_to_png(&doc_cross, format!("{}/swiss_qr.png", OUTPUT_PATH))
        .expect("Failed to write File");

     */

    Ok(())
}

pub fn create_test_receipt_pdf(path: &str, bill_data: &BillData) {
    let mut pdf = Pdf::new();
    let mut next_id = Ref::new(1);

    let catalog_id = next_id.bump();
    let page_tree_id = next_id.bump();
    let page_id = next_id.bump();
    let content_id = next_id.bump();

    // 1. Initialize Fonts
    let fonts = FontLibrary::new(&mut pdf, &mut next_id);

    // 2. Generate Layout Ops
    let mut ops = Vec::new();
    let mut layout = ReceiptLayout {
        bill_data,
        horizontal_offset: Mm(5.0),
        top_start: Mm(100.0), // 105mm total - 5mm margin
        label_font_size: Pt(6.0),
        text_font_size: Pt(8.0),
        label_ascender: Mm(2.1), // Derived from ttf-parser
        text_ascender: Mm(2.8),
        language: Language::De,
        line_spacing: Mm(3.5),
        extra_spacing: Mm(2.0),
    };
    layout.render(&mut ops, &fonts);

    // 3. Write Page and Resources
    pdf.catalog(catalog_id).pages(page_tree_id);
    pdf.pages(page_tree_id).kids([page_id]).count(1);

    let mut page = pdf.page(page_id);
    page.media_box(Rect::new(0.0, 0.0, 595.28, 842.89)); // A4
    page.parent(page_tree_id);
    page.contents(content_id);

    let mut res = page.resources();
    let mut f_dict = res.fonts();
    f_dict.pair(name(FontStyle::Regular), fonts.regular.type0_ref);
    f_dict.pair(name(FontStyle::Bold), fonts.bold.type0_ref);
    f_dict.finish();
    res.finish();
    page.finish();

    // 4. Execute Ops to Content Stream
    let mut content = Content::new();
    // Add the separation line
    content.set_dash_pattern([3.0, 3.0], 0.0);
    content.move_to(62.0 * 72.0 / 25.4, 0.0);
    content.line_to(62.0 * 72.0 / 25.4, 105.0 * 72.0 / 25.4);
    content.stroke();

    execute_receipt_ops(&mut content, &fonts, ops);
    pdf.stream(content_id, &content.finish());

    // 5. Finalize with Catalog at Ref(1)
    std::fs::write(path, pdf.finish()).expect("Failed to write PDF");
}

