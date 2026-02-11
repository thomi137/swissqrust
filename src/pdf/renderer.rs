/*
 * Copyright (c) 2026 Thomas Prosser
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */

use crate::{DrawOp, Layout};

pub trait PdfCanvas {
    fn draw_text(&mut self, text: &str, x: f32, y: f32, size: f32, bold: bool);
    fn draw_rect(&mut self, x: f32, y: f32, w: f32, h: f32);
    fn draw_line(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, width: f32);
    fn draw_qr(&mut self, x: f32, y: f32, size: f32);
}


pub fn render(layout: Layout, canvas: &mut dyn PdfCanvas) {
    for op in layout.ops {
        match op {
            DrawOp::Text { text, at, size, bold } => {
                canvas.draw_text(text.as_str(), at.x.0, at.y.0, size.0, bold);
            }
            DrawOp::Box { rect } => {
                canvas.draw_rect(rect.x.0, rect.y.0, rect.width.0, rect.height.0);
            }
            DrawOp::Line { from, to, width } => {
                canvas.draw_line(from.0.0, from.1.0, to.0.0, to.1.0, width.0);
            }
            DrawOp::QrCodeSpace { at, size } => {
                canvas.draw_qr(at.0.0, at.1.0, size.0);
            }
            _ => {}
        }
    }
}

