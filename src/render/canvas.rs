/*
 * Copyright (c) 2026 Thomas Prosser
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */

use pdf_writer::{Content, Name, Str};
use qrcodegen::QrCode;
use crate::{DrawOp, Layout, Mm};

pub trait PdfCanvas {
    fn draw_text(&mut self, text: &str, x: f32, y: f32, size: f32, bold: bool);
    fn draw_rect(&mut self, x: f32, y: f32, w: f32, h: f32);
    fn draw_line(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, width: f32);
    fn draw_qr(&mut self, x: f32, y: f32, size: f32);
}

pub struct PdfWriterCanvas<'a> {
    pub content: &'a mut Content,
    pub page_height: f32,
    pub qr_payload: String,
}


/*
impl PdfCanvas for PdfWriterCanvas<'_> {
    fn draw_text(&mut self, text: &str, x: f32, y: f32, size: f32, _bold: bool) {
        let y_pdf = self.page_height - y;

        self.content.begin_text();

        // TODO Lucida Sans
        self.content.set_font(Name(b"F1"), size);
        self.content.set_text_matrix([1.0, 0.0, 0.0, 1.0, x, y_pdf]);
        self.content.show(Str(text.as_bytes()));
        self.content.end_text();
    }

    fn draw_rect(&mut self, x: f32, y: f32, w: f32, h: f32) {
        let y_pdf = self.page_height - y - h;
        self.content.rect(x, y_pdf, w, h);
        self.content.stroke();
    }

    fn draw_line(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, width: f32) {
        let y1_pdf = self.page_height - y1;
        let y2_pdf = self.page_height - y2;

        self.content.set_line_width(width);
        self.content.move_to(x1, y1_pdf);
        self.content.line_to(x2, y2_pdf);
        self.content.stroke();
    }

    fn draw_qr(&mut self, x: f32, y: f32, size: f32) {
        let code = QrCode::new(&self.qr_payload).unwrap();
        let width = code.width();
        let module_size = size / width as f32;

        for row in 0..width {
            for col in 0..width {
                if code[(col, row)].is_dark() {
                    {
                        let px = x + col as f32 * module_size;
                        let py = y + row as f32 * module_size;
                        let py_pdf = self.page_height - py - module_size;

                        self.content.rect(px, py_pdf, module_size, module_size);
                        self.content.fill();
                    }
                }
            }
        }
    }

}

pub fn render(layout: &Layout, canvas: &mut dyn PdfCanvas) {
    for op in &layout.ops {
        match op {
            DrawOp::Text { text, at, size, bold } => {
                canvas.draw_text(
                    text,
                    at.x.to_pt().0,
                    at.y.to_pt().0,
                    size.0,
                    *bold,
                );
            }

            DrawOp::TextLines { lines, start, size, leading } => {
                let mut current_y = start.y;

                for line in lines {
                    canvas.draw_text(
                        line,
                        start.x.to_pt().0,
                        current_y.to_pt().0,
                        size.0,
                        false,
                    );

                    current_y = Mm(current_y.0 - leading.0);
                }
            }

            DrawOp::Box { rect } => {
                canvas.draw_rect(
                    rect.x.to_pt().0,
                    rect.y.to_pt().0,
                    rect.width.to_pt().0,
                    rect.height.to_pt().0,
                );
            }

            DrawOp::Line { from, to, width } => {
                canvas.draw_line(
                    from.0.to_pt().0,
                    from.1.to_pt().0,
                    to.0.to_pt().0,
                    to.1.to_pt().0,
                    width.to_pt().0,
                );
            }

            DrawOp::QrCodeSpace { at, size } => {
                canvas.draw_qr(
                    at.0.to_pt().0,
                    at.1.to_pt().0,
                    size.to_pt().0,
                );
            }
        }
    }

 */



