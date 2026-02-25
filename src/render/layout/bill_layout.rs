/*
 * Copyright (c) 2026 Thomas Prosser
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */

use crate::{BillData, Language, ReferenceType};
use crate::language::*;
use crate::render::layout::geometry::*;
use crate::render::layout::spacing::*;
pub(crate) use crate::layout::block::{Column, LayoutBlock};

///
/// This is a module specific trait.
/// It may depend on the `support::traits` module
/// but it does not feel right to hand imports back and forth.
/// maybe some later refactoring will fix this.
///


pub struct LayoutCursor {
    pub x: Mm,
    pub y: Mm,
}

/// Configuration for which sections to render.
pub struct BillLayoutConfig {
    pub has_acceptance_point: bool,
    pub max_height: Mm,
    pub debtor_box_height: Mm,
    pub amount_section_top: Mm,
}

/// Base layout engine shared by Payment Part and Receipt
pub struct BillLayout<'a> {
    pub bill_data: &'a BillData,
    pub config: BillLayoutConfig,

    // geometry
    pub horizontal_offset: Mm,
    pub top_start: Mm,

    // language
    pub language: Language,

    // typography
    pub label_font_size: Pt,
    pub text_font_size: Pt,
    pub label_ascender: Mm,
    pub text_ascender: Mm,

    // spacing (computed)
    pub line_spacing: Mm,
    pub extra_spacing: Mm,
}

impl<'a> BillLayout<'a> {
    pub fn compute_spacing(&mut self) -> bool {
        let mut text_lines = 0usize;
        let mut extra_blocks = 0usize;
        let mut fixed_height = Mm(0.0);

        text_lines += 1 + self.bill_data.creditor_address.to_lines().len();

        match &self.bill_data.reference_type {
            ReferenceType::NoRef => {}
            _ => {
                extra_blocks += 1;
                text_lines += 2;
            }
        }
        extra_blocks += 1;

        match &self.bill_data.debtor_address {
            Some(address) => text_lines += 1 + address.to_lines().len(),
            None => {
                text_lines += 1;
                fixed_height = Mm(fixed_height.0 + self.config.debtor_box_height.0);
            }
        }
        extra_blocks += 1;

        let spacing = compute_spacing(self.config.max_height, fixed_height, text_lines, extra_blocks, self.line_spacing);

        self.line_spacing = spacing.line_spacing;
        self.extra_spacing = spacing.extra_spacing;

        spacing.extra_spacing.0 / spacing.line_spacing.0 < 0.8
    }
}
