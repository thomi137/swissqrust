/*
 * Copyright (c) 2026 Thomas Prosser
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */

use pdf_writer::Finish;
use pdf_writer::{Content, Ref, Rect, Pdf};
use anyhow::Result;

use crate::{name, BillData, BillLayout, DrawOp, FontLibrary, FontStyle, Language, Mm, PP_LABEL_PREF_FONT_SIZE, PP_TEXT_PREF_FONT_SIZE, PT_PER_MM};

pub struct PdfPainter<'a> {
    pub content: &'a mut Content,
    pub fonts: &'a FontLibrary,
}

 pub struct PDFBuilder {
     pub pdf: Pdf,
     pub ops: Vec<DrawOp>,
     pub next_id: Ref,
     pub content: Content,
     pub content_id: Ref,
     pub fonts: FontLibrary,
}

impl PDFBuilder {
     pub fn new() -> Self {

         let mut pdf = Pdf::new();
         let mut ops: Vec<DrawOp> = Vec::new();
         let mut next_id = Ref::new(1);
         let content_id = next_id.bump();
         let mut content = Content::new();
         let fonts = FontLibrary::new(&mut pdf, &mut next_id);

         Self {
             pdf,
             ops,
             next_id,
             content,
             content_id,
             fonts
         }
     }

    pub fn setup_pdf(&mut self) -> Result<()> {

        let catalog_id = self.next_id.bump();
        let page_tree_id = self.next_id.bump();
        let page_id = self.next_id.bump();
        let content_id = self.next_id.bump();

        // I will have to use Zapf Dingbats for Scissors Symbol
        let zapf_id = self.next_id.bump();
        self.pdf.type1_font(zapf_id).base_font(pdf_writer::Name(b"ZapfDingbats"));

        self.pdf.catalog(catalog_id).pages(page_tree_id);
        self.pdf.pages(page_tree_id).kids([page_id]).count(1);
        let mut page = self.pdf.page(page_id);


        // Setup the resources.
        let mut res = page.resources();
        let mut f_dict = res.fonts();
        f_dict.pair(pdf_writer::Name(b"Zapf"), zapf_id);
        f_dict.pair(name(FontStyle::Regular), self.fonts.regular.type0_ref);
        f_dict.pair(name(FontStyle::Bold), self.fonts.bold.type0_ref);
        f_dict.finish();
        res.finish();

        // Create A4 Page
        page.media_box(Rect::new(0.0, 0.0, 595.28, 842.89)); // A4
        page.parent(page_tree_id);
        page.contents(content_id);
        page.finish();

        Ok(())
     }

    fn draw_perforation_horizonal(&mut self) {
       self.content.save_state();
       self.content.set_dash_pattern([3.0, 3.0], 0.0);
       self.content.set_line_width(0.75);

        let y_sep = 105.0 * PT_PER_MM;
        self.content.move_to(0.0, y_sep);
        self.content.line_to(210.0 * PT_PER_MM, y_sep);
        self.content.stroke();

        draw_scissors_official(&mut self.content, 64.8, y_sep, 180.0);
        self.content.restore_state();
    }

    fn draw_perforation_vertical(&mut self) {
        self.content.save_state();
        self.content.set_dash_pattern([3.0, 3.0], 0.0);
        self.content.set_line_width(0.75);

        let x_sep = 62.0 * PT_PER_MM;
        self.content.move_to(x_sep, 0.0);
        self.content.line_to(x_sep, 105.0 * PT_PER_MM);
        self.content.stroke();

        draw_scissors_official(&mut self.content, x_sep, 80.0 * PT_PER_MM, 90.0);
        self.content.restore_state();

    }

    pub fn painter(&mut self) -> PdfPainter<'_> {
        PdfPainter {
            content: &mut self.content,
            fonts: &self.fonts,
        }
    }

 }

// Rendering pipeline (DO NOT REORDER):
// 1. setup_pdf()
// 2. compute layout spacing
// 3. layout blocks → Vec<DrawOp>
// 4. execute DrawOps into Content
// 5. attach content stream
// 6. write PDF

pub fn create_pdf(path: &str, bill_data: &BillData) -> anyhow::Result<()> {


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
