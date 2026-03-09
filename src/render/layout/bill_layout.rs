/*
 * Copyright (c) 2026 Thomas Prosser
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */

use crate::{BillData, DrawOp, FontStyle, Language};
use crate::render::FontMetrics;
use crate::render::layout::geometry::*;

/// This is a module specific trait.
/// It may depend on the `support::traits` module
pub struct LayoutCursor {
    pub x: Mm,
    pub y: Mm,
}

/// Trait for layout font metric strategies.
pub trait LayoutStrategy<T: FontMetrics> {

    const LABEL_SIZE: Pt;
    const TEXT_SIZE: Pt;
    const TITLE_SIZE: Pt;
    const MAX_HEIGHT: Mm;

    fn render(&mut self,
              bill_data: &BillData,
              language: Language,
              metrics: &T,
              ops: &mut Vec<DrawOp>
    );

}

/// Configuration for which sections to render.
pub struct BillLayoutConfig {
    pub has_acceptance_point: bool,
    pub max_height: Mm,
    pub debtor_box_height: Mm,
    pub amount_section_top: Mm,
}

/// Base layout information shared by Payment Part and Receipt Part
pub struct RenderContext<'a, T: FontMetrics> {
    pub bill_data: &'a BillData,
    pub language: Language,
    pub metrics: &'a T,
    pub label_size: Pt,
    pub text_size: Pt,
    pub title_size: Pt,
    pub line_spacing: Mm,
    pub label_ascender: Mm,
    pub title_ascender: Mm,
    pub text_ascender: Mm,
    pub extra_spacing: Mm, // Calculated by your spacing logic
}

impl<'a, T: FontMetrics> RenderContext<'a, T> {
    pub fn for_strategy<S: LayoutStrategy<T>>(
        bill_data: &'a BillData,
        language: Language,
        metrics: &'a T
    ) -> Self {
        Self {
            bill_data,
            language,
            metrics,
            label_size: S::LABEL_SIZE,
            text_size: S::TEXT_SIZE,
            title_size: S::TITLE_SIZE,
            line_spacing: metrics.line_height_mm(FontStyle::Regular, S::TEXT_SIZE),
            label_ascender: metrics.ascender_mm(FontStyle::Bold, S::LABEL_SIZE),
            text_ascender: metrics.ascender_mm(FontStyle::Regular, S::TEXT_SIZE),
            title_ascender: metrics.ascender_mm(FontStyle::Bold, S::TITLE_SIZE),
            extra_spacing: Mm(0.0), // Logic for Swiss spacing goes here
        }
    }
}
