/*
 * Copyright (c) 2026 Thomas Prosser
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */

#[cfg(feature = "pdf-debug")]
use pdf_writer::{Content, Name};
#[cfg(feature = "pdf-debug")]
use crate::{PDFBuilder, PT_PER_MM};

#[cfg(feature = "pdf-debug")]
pub fn draw_debug_overlay(builder: &mut PDFBuilder) {

    let content = &mut builder.content;

    draw_grid(content);
    draw_page_border(content);
    draw_spec_boxes(content);
    draw_baselines(content);
    draw_payment_blocks(content);
    draw_receipt_blocks(content);
}

#[cfg(feature = "pdf-debug")]
fn draw_grid(content: &mut Content) {
    let w = 210.0 * PT_PER_MM;
    let h = 297.0 * PT_PER_MM;
    let step = 5.0 * PT_PER_MM;

    content.save_state();
    content.set_stroke_rgb(1.0, 0.6, 0.0); // amber
    content.set_line_width(0.3);

    let mut x = 0.0;
    while x <= w {
        content.move_to(x, 0.0);
        content.line_to(x, h);
        x += step;
    }

    let mut y = 0.0;
    while y <= h {
        content.move_to(0.0, y);
        content.line_to(w, y);
        y += step;
    }

    content.stroke();
    content.restore_state();
}

#[cfg(feature = "pdf-debug")]
fn draw_page_border(content: &mut Content) {
    let w = 210.0 * PT_PER_MM;
    let h = 297.0 * PT_PER_MM;

    content.save_state();
    content.set_stroke_rgb(1.0, 0.0, 0.0);
    content.set_line_width(1.0);
    content.rect(0.0, 0.0, w, h);
    content.stroke();
    content.restore_state();
}

#[cfg(feature = "pdf-debug")]
fn draw_spec_boxes(content: &mut Content) {
    content.save_state();
    content.set_stroke_rgb(0.0, 0.5, 1.0); // blue
    content.set_line_width(0.8);

    // Receipt area
    rect_mm(content, 0.0, 0.0, 62.0, 105.0);

    // Payment part
    rect_mm(content, 62.0, 0.0, 148.9, 105.0);

    // QR Code
    rect_mm(content, 62.0 + 5.0, 42.0, 46.0, 46.0);

    content.stroke();
    content.restore_state();
}

#[cfg(feature = "pdf-debug")]
fn draw_payment_blocks(content: &mut Content) {
    content.save_state();
    content.set_stroke_rgb(0.8, 0.0, 0.8); // magenta
    content.set_line_width(0.8);

    rect_mm(content, 67.0, 93.0, 51.0, 7.0);
    draw_debug_text_label_mm(content, 67.5, 93.5, "title".to_uppercase().as_bytes());
    rect_mm(content, 67.0, 15.0, 51.0, 22.0);
    draw_debug_text_label_mm(content, 67.5, 15.5, "amount".to_uppercase().as_bytes());rect_mm(content, 67.0, 15.0, 51.0, 22.0);
    rect_mm(content, 118.0, 15.0, 87.0, 85.0);
    draw_debug_text_label_mm(content, 118.5, 15.5, "information".to_uppercase().as_bytes());
    rect_mm(content, 67.0, 5.0, 138.0, 10.0);
    draw_debug_text_label_mm(content, 67.5, 5.5, "further information".to_uppercase().as_bytes());

}

#[cfg(feature = "pdf-debug")]
fn draw_receipt_blocks(content: &mut Content) {
    content.save_state();
    content.set_stroke_rgb(0.8, 0.0, 0.8); // magenta
    content.set_line_width(0.8);

    rect_mm(content, 5.0, 93.0, 52.0, 7.0);
    draw_debug_text_label_mm(content, 5.5, 93.5, "title".to_uppercase().as_bytes());
    rect_mm(content, 5.0, 37.0, 52.0, 56.0);
    draw_debug_text_label_mm(content, 5.5, 37.5, "information".to_uppercase().as_bytes());
    rect_mm(content, 5.0, 23.0, 52.0, 14.0);
    draw_debug_text_label_mm(content, 5.5, 23.5, "amount".to_uppercase().as_bytes());
    rect_mm(content, 5.0, 5.0, 52.0, 18.0);
    draw_debug_text_label_mm(content, 5.5, 5.5, "acceptance point".to_uppercase().as_bytes());


    content.stroke();
    content.restore_state();
}

#[cfg(feature = "pdf-debug")]
fn rect_mm(content: &mut Content, x: f32, y: f32, w: f32, h: f32) {
    content.rect(
        x * PT_PER_MM,
        y * PT_PER_MM,
        w * PT_PER_MM,
        h * PT_PER_MM,
    );
}

#[cfg(feature = "pdf-debug")]
fn draw_baselines(content: &mut Content) {
    content.save_state();
    content.set_stroke_rgb(0.0, 0.8, 0.0);
    content.set_line_width(0.4);

    let w = 210.0 * PT_PER_MM;
    let mut y = 0.0;

    while y <= 297.0 {
        let py = y * PT_PER_MM;
        content.move_to(0.0, py);
        content.line_to(w, py);
        y += 5.0;
    }

    content.stroke();
    content.restore_state();
}

#[cfg(feature = "pdf-debug")]
fn draw_debug_text_label_mm(content: &mut Content, x: f32, y: f32, text: &[u8]) {

    let mm_x = x * PT_PER_MM;
    let mm_y = y * PT_PER_MM;

    content.begin_text();
    content.set_font(Name(b"Courier"), 5.0);
    content.set_fill_rgb(1.0, 0.0, 1.0);
    content.set_text_matrix([1.0, 0.0, 0.0, 1.0, mm_x, mm_y]);
    content.show(pdf_writer::Str(text));
    content.end_text();

}
