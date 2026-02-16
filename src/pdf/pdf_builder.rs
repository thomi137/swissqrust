/*
 * Copyright (c) 2026 Thomas Prosser
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */

use pdf_writer::{Content, Finish, Name, Pdf, Rect, Ref, Str, TextStr};
use pdf_writer::types::{ActionType, AnnotationType, BorderType};
use crate::pdf::fonts::{embed_ttf_font, LIBERATION_SANS_BOLD_TTF, LIBERATION_SANS_REGULAR_TTF};

pub fn base_layout(path: &str) {
    let mut pdf = Pdf::new();
    let mut next_id = Ref::new(1);

    let catalog_id = next_id.bump();
    let page_tree_id = next_id.bump();
    let page_id = next_id.bump();



    // let regular_font_id = fonts.regular;
    // let bold_font_id = fonts.bold;
    let content_id = next_id.bump();
    let annotation_id = next_id.bump();

    let regular_font_name = Name(b"LiberationSansRegular");
    let bold_font_name = Name(b"LiberationSansBold");

    let reg_font = embed_ttf_font(&mut pdf, &mut next_id, regular_font_name, LIBERATION_SANS_REGULAR_TTF);
    let bold_font = embed_ttf_font(&mut pdf, &mut next_id, bold_font_name, LIBERATION_SANS_BOLD_TTF);

    pdf.catalog(catalog_id).pages(page_tree_id);
    // Write the page tree with a single child page.
    pdf.pages(page_tree_id).kids([page_id]).count(1);

    let mut page = pdf.page(page_id);
    page.media_box(Rect::new(0.0, 0.0, 595.0, 842.0));
    page.parent(page_tree_id);
    page.contents(content_id);
    page.annotations([annotation_id]);

    let mut res = page.resources();
    let mut fonts = res.fonts();
    fonts.pair(regular_font_name, reg_font.type0_ref);
    fonts.pair(bold_font_name, bold_font.type0_ref);
    fonts.finish();
    res.finish();

    page.finish();

    let mut annotation = pdf.annotation(annotation_id);
    annotation.subtype(AnnotationType::Link);
    annotation.rect(Rect::new(215.0, 730.0, 251.0, 748.0));
    annotation.contents(TextStr("Link to the Rust project web page"));
    annotation.color_rgb(0.0, 0.0, 1.0);

    annotation
        .action()
        .action_type(ActionType::Uri)
        .uri(Str(b"https://www.rust-lang.org/"));

    // Set border and style for the link annotation.
    annotation.border_style().width(2.0).style(BorderType::Underline);
    annotation.finish();

    let face = ttf_parser::Face::parse(LIBERATION_SANS_REGULAR_TTF, 0).unwrap();
    let text = "Hello World from Rust!";
    let mut encoded_bytes = Vec::new();

    for c in text.chars() {
        // Look up the Glyph ID for the character
        let gid = face.glyph_index(c).map(|g| g.0).unwrap_or(0);
        // Identity-H uses 2-byte Big-Endian
        encoded_bytes.push((gid >> 8) as u8);
        encoded_bytes.push((gid & 0xFF) as u8);
    }



    let mut content = Content::new();
    content.begin_text();
    content.set_font(regular_font_name, 14.0);
    content.next_line(108.0, 734.0);
    content.show(Str(&encoded_bytes));
    content.end_text();
    pdf.stream(content_id, &content.finish());

    let buf: Vec<u8> = pdf.finish();

    // Write the thing to a file.
    std::fs::write(path, buf).expect("TODO: panic message");

}
