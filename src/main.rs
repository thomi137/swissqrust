
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
    Some(String::from("32111.00")),
    ReferenceType::infer("000008207791225857421286694")?,
    Some(String::from("Premium calculation July 2020")),
    None
    ).unwrap();

    create_test_receipt_pdf("./examples/test_receipt.pdf", &bill_data);


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

    let zapf_id = next_id.bump();
    pdf.type1_font(zapf_id).base_font(pdf_writer::Name(b"ZapfDingbats"));

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
    f_dict.pair(pdf_writer::Name(b"Zapf"), zapf_id);
    f_dict.pair(name(FontStyle::Regular), fonts.regular.type0_ref);
    f_dict.pair(name(FontStyle::Bold), fonts.bold.type0_ref);
    f_dict.finish();
    res.finish();
    page.finish();

    //  Execute Ops to Content Stream
    let mut content = Content::new();

    // 1. Horizontal Perforation (Top of Bill)
    content.save_state();
    content.set_dash_pattern([3.0, 3.0], 0.0); // No need for manual .op("d")
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

    execute_receipt_ops(&mut content, &fonts, ops);
    pdf.stream(content_id, &content.finish());

    // 5. Finalize with Catalog at Ref(1)
    std::fs::write(path, pdf.finish()).expect("Failed to write PDF");
}

fn draw_scissors_official(content: &mut Content, x: f32, y: f32, rotation_deg: f32) {
    content.save_state();

    // 1. Move to the line coordinate
    content.transform([1.0, 0.0, 0.0, 1.0, x, y]);

    // 2. Apply rotation (if 90, it points UP along the vertical line)
    let rad = rotation_deg.to_radians();
    content.transform([rad.cos(), rad.sin(), -rad.sin(), rad.cos(), 0.0, 0.0]);

    // 3. Draw the White Mask (SVG .st1: 23.1pt wide, 18.3pt high)
    content.set_fill_rgb(1.0, 1.0, 1.0);
    // Center the mask on (0,0)
    content.rect(-11.5, -9.0, 11.5, 9.5);
    content.fill_nonzero();

    // 4. Draw the Scissors (SVG .st2: ZapfDingbats 26px)
    content.set_fill_rgb(0.0, 0.0, 0.0);
    content.begin_text();
    content.set_font(pdf_writer::Name(b"Zapf"), 13.0); // Size from your SVG

    // Offset to center the glyph ✂ (approx -8pt horizontally, -9pt vertically)
    // These offsets align the center of the 'X' in the scissors with (0,0)
    content.set_text_matrix([1.0, 0.0, 0.0, 1.0, -13.5, -4.5]);
    content.show(pdf_writer::Str(b"\x22"));
    content.end_text();

    content.restore_state();
}
