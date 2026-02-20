/*
 * Copyright (c) 2026 Thomas Prosser
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */

use pdf_writer::{Content};
use qrcodegen::QrCode;
use crate::{DrawOp, FontLibrary, FontStyle};
use crate::pdf::name;
use crate::render::qr_renderers::render_qr_pdf;
/*
 * ⚠️
 * pdf_writer is pretty low level, so what is coded here
 * is best understood with some PDF know how. A good introduction
 * can be found (here)[https://medium.com/@jberkenbilt/text-in-pdf-introduction-df3dd3dfa9ea]
 * and there is also a (cheat sheet)[https://pdfa.org/wp-content/uploads/2023/08/PDF-Operators-CheatSheet.pdf]
 */


pub fn execute_bill_ops (
    content: &mut Content,
    fonts: &FontLibrary,
    ops: Vec<DrawOp>,
    qr_data: Option<&QrCode>,
){
    for op in ops {
        match op {
            DrawOp::Text {text, at, size, bold} => {
                let style = if bold { FontStyle::Bold } else { FontStyle::Regular };
                let font_obj = if bold { &fonts.bold } else { &fonts.regular };

                // PDF requires glyphs to render, so we need to encode chars
                let gids = font_obj.encode(&text);
                content.begin_text();
                content.set_font(name(style), size.0);

                // Using a text matrix is more robust than a relative Td operator.
                // This one places text at the BaseLine.
                content.set_text_matrix([1.0, 0.0, 0.0, 1.0, at.x.to_pt().0, at.y.to_pt().0]);
                content.show(pdf_writer::Str(&gids));
                content.end_text();
            },
            DrawOp::Line { from, to, width } => {
                content.set_line_width(width.to_pt().0);
                content.move_to(from.0.to_pt().0, from.1.to_pt().0);
                content.line_to(to.0.to_pt().0, to.1.to_pt().0);
                content.stroke();
            }
            DrawOp::Box { rect } => {
                content.set_line_width(0.75); // SIX standard for corner/amount boxes
                content.rect(rect.x.to_pt().0, rect.y.to_pt().0, rect.width.to_pt().0, rect.height.to_pt().0);
                content.stroke();
            }
            DrawOp::QrCodeSpace { at, .. } => {
                if let Some(qr_data) = qr_data {
                    render_qr_pdf(content, qr_data, at.x.to_pt().0, at.y.to_pt().0);
                }
            }
        }
    }

}

