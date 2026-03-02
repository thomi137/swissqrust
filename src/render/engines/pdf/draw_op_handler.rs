/*
 * Copyright (c) 2026 Thomas Prosser
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */

use pdf_writer::Content;
use qrcodegen::QrCode;
use crate::{Mm, MARGIN, QR_CODE_HEIGHT};
use crate::render::types::DrawOp;
use crate::render::engines::pdf::FontLibrary;

/*
 * ⚠️ pdf-writer is not for the faint of heart
 * it basically abstracts low-level pdf commands and symbols
 * There is a good article at Medium:
 * https://medium.com/@jberkenbilt/text-in-pdf-introduction-df3dd3dfa9ea
 * And then, here is a cheat sheet by the pdf association:
 * https://pdfa.org/wp-content/uploads/2023/08/PDF-Operators-CheatSheet.pdf
 */

// Define handlers for each DrawOp type
trait DrawOpHandler {
    fn handle(&self, content: &mut Content, op: &DrawOp, qr_data: Option<&QrCode>, fonts: &FontLibrary);
}

struct TextHandler;
impl DrawOpHandler for TextHandler {
    fn handle(&self, content: &mut Content, op: &DrawOp, _: Option<&QrCode>, fonts: &FontLibrary) {
        if let DrawOp::Text { text, at, size, bold } = op {
            let style = if *bold { crate::pdf::FontStyle::Bold } else { crate::pdf::FontStyle::Regular };
            let font_obj = if *bold { &fonts.bold } else { &fonts.regular };
            let gids = font_obj.encode(text);

            let pdf_y = at.y.to_pdf().0;

            content.begin_text();
            content.set_font(crate::pdf::name(style), size.0);
            content.set_text_matrix([1.0, 0.0, 0.0, 1.0,
                at.x.to_pt().0, pdf_y.to_pt().0]);
            content.show(pdf_writer::Str(&gids));
            content.end_text();
        }
    }
}

struct LineHandler;
impl DrawOpHandler for LineHandler {
    fn handle(&self, content: &mut Content, op: &DrawOp, _: Option<&QrCode>, _: &FontLibrary) {
        if let DrawOp::Line { from, to, width } = op {

            let fy = from.1.to_pdf().0;
            let ty = to.1.to_pdf().0;

            content.set_line_width(width.to_pt().0);
            content.move_to(from.0.to_pt().0, fy.to_pt().0);
            content.line_to(to.0.to_pt().0, ty.to_pt().0);
            content.stroke();
        }
    }
}

struct BoxHandler;
impl DrawOpHandler for BoxHandler {
    fn handle(&self, content: &mut Content, op: &DrawOp, _: Option<&QrCode>, _: &FontLibrary) {
        if let DrawOp::Box { rect } = op {
            content.set_line_width(0.75);
            content.rect(rect.x.to_pt().0, rect.y.to_pt().0, rect.width.to_pt().0, rect.height.to_pt().0);
            content.stroke();
        }
    }
}

struct QrCodeHandler;
impl DrawOpHandler for QrCodeHandler {
    fn handle(&self, content: &mut Content, op: &DrawOp, qr_data: Option<&QrCode>, _: &FontLibrary) {

        if let DrawOp::QrCodeSpace { at, .. } = op {

            let mut y = at.y.to_pdf().0;
            //y = y - QR_CODE_HEIGHT; // anchor from top

            if let Some(qr) = qr_data {
                crate::render::engines::qr_renderers::render_qr_pdf(content, qr, at.x.to_pt().0, y.to_pt().0);
            }
        }
    }
}

pub fn execute_bill_ops(
    content: &mut Content,
    fonts: &FontLibrary,
    ops: Vec<DrawOp>,
    qr_data: Option<&QrCode>,
) {

    for op in ops.iter() {

        match &op {
            DrawOp::Text { .. } => TextHandler.handle(content, &op, qr_data, fonts),
            DrawOp::Line { .. } => LineHandler.handle(content, &op, qr_data, fonts),
            DrawOp::Box { .. } => BoxHandler.handle(content, &op, qr_data, fonts),
            DrawOp::QrCodeSpace { .. } => QrCodeHandler.handle(content, &op, qr_data, fonts),
        }

    }
}
