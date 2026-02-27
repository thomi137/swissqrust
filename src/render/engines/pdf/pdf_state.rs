/*
 * Copyright (c) 2026 Thomas Prosser
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */

// pdf_state.rs
use pdf_writer::{Content, Pdf, Ref};
use pdf_writer::writers::Page;
use crate::{DrawOp, FontLibrary};

pub struct PdfState {
    pub pdf: Pdf,
    pub next_id: Ref,
    pub ops: Vec<DrawOp>,
    pub fonts: FontLibrary,
}

impl PdfState {
    pub fn new(fonts: FontLibrary) -> Self {
        Self {
            pdf: Pdf::new(),
            next_id: Ref::new(1),
            ops: Vec::new(),
            fonts,
        }
    }
}