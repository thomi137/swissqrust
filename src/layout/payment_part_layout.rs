/*
 * Copyright (c) 2026 Thomas Prosser
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */
use usvg::tiny_skia_path::f32x2;
use crate::{label, Language, QRLayoutRect};
use crate::layout::geometry::{
  Baseline,
  Mm,
  Pt,
  DrawOp,
};

const SLIP_WIDTH: Mm = Mm(210f32); // mm
const SLIP_HEIGHT: Mm = Mm(105f32); // mm
const MARGIN: Mm = Mm(5f32); // mm
const RECEIPT_WIDTH: Mm = Mm(62f32); // mm
const AMOUNT_SECTION_TOP: Mm = Mm(37f32); // mm (from bottom)


pub fn layout_payment_part_title(
    ops: &mut Vec<DrawOp>,
    text: &str,
    ascender_mm: Mm,
) {
    let font_size = Pt(11f32);
    let baseline = Baseline {
        x: MARGIN,
        y: Mm(SLIP_HEIGHT.0 - MARGIN.0 - ascender_mm.0),
    };

    ops.push(DrawOp::Text {
        text: text.to_string(),
        at: baseline,
        size: font_size,
        bold: true,
    });
}

/// Result of vertical spacing calculation.
pub struct Spacing {
    pub line_spacing: Mm,
    pub extra_spacing: Mm,
}

/// Attributes:
///
/// max_height: total available vertical space
/// fixed_height: height occupied by non-text elements
/// text_lines: total number of text lines
/// extra_blocks: number of gaps between text blocks
/// text_font_size_mm: line height in mm
pub fn compute_spacing(
    max_height: Mm,
    fixed_height: Mm,
    text_lines: usize,
    extra_blocks: usize,
    text_line_height: Mm,
) -> Spacing {
    let total_text_height = Mm(text_line_height.0 * text_lines as f32);
    let remaining = Mm(
        max_height.0 - fixed_height.0 - total_text_height.0
    );

    let mut extra = if extra_blocks > 0 {
        Mm(remaining.0 / extra_blocks as f32)
    } else {
        Mm(0.0)
    };
    if extra.0 < 0.0 {
        extra = Mm(0.0);
    }
    if extra.0 > text_line_height.0 {
        extra = text_line_height;
    }

    Spacing {
        line_spacing: text_line_height,
        extra_spacing: extra,
    }

}

pub struct PaymentPartLayout<'a> {

    pub language: Language,
    pub ops: Vec<DrawOp>,


    // typography
    pub label_font_size: Pt,
    pub text_font_size: Pt,
    pub label_ascender: Mm,
    pub text_ascender: Mm,

    // spacing
    pub line_spacing: Mm,
    pub extra_spacing: Mm,

    // content
    pub payable_to_lines: &'a [String],
    pub reference: Option<String>,
    pub additional_info_lines: &'a Option<&'a[String]>,
    pub payable_by_lines: &'a Option<&'a[String]>,

    // geometry
    pub horizontal_offset: Mm,
    pub top_start: Mm,

    pub amount: Option<String>,
    pub currency: String,
}
impl PaymentPartLayout<'_> {
    const PP_INFO_SECTION_MAX_HEIGHT: Mm = Mm(85.0);
    const DEBTOR_BOX_HEIGHT_PP: Mm = Mm(25f32); // mm


    /// Layouts the "Currency / Amount" block
    ///
    /// Coordinates are relative to the payment part origin.
    /// AMOUNT_SECTION_TOP is measured from bottom of slip.
    pub fn layout_amount_section(&mut self) {
        let amount = match self.amount {
            Some(ref amount) => amount.as_str(),
            None => return
        };

        const CURRENCY_WIDTH_PP: f32 = 15.0;
        const AMOUNT_BOX_WIDTH_PP: f32 = 40.0;
        const AMOUNT_BOX_HEIGHT_PP: f32 = 15.0;
        const AMOUNT_SECTION_TOP: f32 = 37.0;

        let currency_width = Mm(CURRENCY_WIDTH_PP);
        let amount_box_width = Mm(AMOUNT_BOX_WIDTH_PP);
        let amount_box_height = Mm(AMOUNT_BOX_HEIGHT_PP);
        let section_top = Mm(AMOUNT_SECTION_TOP);
        let currency_label_y =
            Mm(section_top.0 - self.label_ascender.0);

        // Currency Label
        self.ops.push(DrawOp::Text {
            text: "Currency".into(),
            at: Baseline {
                x: self.horizontal_offset,
                y: currency_label_y,
            },
            size: self.label_font_size,
            bold: true,
        });

        // Currency value

        let currency_value_y =
            Mm(currency_label_y.0 - self.text_font_size.to_mm().0 - Pt(3.0).to_mm().0);

        self.ops.push(DrawOp::Text {
            text: "CHF".into(),
            at: Baseline {
                x: self.horizontal_offset,
                y: currency_value_y,
            },
            size: self.text_font_size,
            bold: false,
        });

        // Amount Label
        let amount_label_y =
            Mm(section_top.0 - self.label_ascender.0);

        self.ops.push(DrawOp::Text {
            text: "Amount".into(),
            at: Baseline {
                x: Mm(self.horizontal_offset.0 + currency_width.0),
                y: amount_label_y,
            },
            size: self.label_font_size,
            bold: true,
        });

        match &self.amount {
            Some(amount_text) => {
                let amount_value_y =
                    Mm(amount_label_y.0 - self.text_font_size.to_mm().0 - Pt(3.0).to_mm().0);

                self.ops.push(DrawOp::Text {
                    text: amount_text.to_string(),
                    at: Baseline {
                        x: Mm(self.horizontal_offset.0 + currency_width.0),
                        y: amount_value_y,
                    },
                    size: self.text_font_size,
                    bold: false,
                });
            }

            None => {
                // Empty amount box
                let box_y =
                    Mm(section_top.0 - amount_box_height.0);

                self.ops.push(DrawOp::Box {
                    rect: QRLayoutRect {
                        x: Mm(self.horizontal_offset.0 + currency_width.0),
                        y: box_y,
                        width: amount_box_width,
                        height: amount_box_height,
                    },
                });
            }
        }
    }

    pub fn layout_information_section(&mut self) {
        const DEBTOR_BOX_WIDTH_PP: Mm = Mm(65f32); // mm

        let mut y = Mm(self.top_start.0 - self.label_ascender.0);
        let x = self.horizontal_offset;

        // Account / Payable to
        self.draw_label("Account / Payable to".into(), x, &mut y);
        self.draw_text_lines(&self.payable_to_lines, x, &mut y);

        // Reference No. if applicable
        if let Some(reference) = self.reference.clone() {
            self.draw_label("Reference".into(), x, &mut y);
            self.draw_single_line(reference, x, &mut y);
        }

        // Additional information
        if let Some(lines) = self.additional_info_lines {
            self.draw_label("Additional information".into(), x, &mut y);
            self.draw_text_lines(lines, x, &mut y);
        }


        match self.payable_by_lines {
            Some(lines) => {
                self.draw_label("Payable by".into(), x, &mut y);
                self.draw_text_lines(&lines, x, &mut y);
            },


            // TOSO: Just draw the svg.
            None => {
                self.draw_label("Payable by (name/address)".into(), x, &mut y);

                // space down for box
                y = Mm(y.0 - DEBTOR_BOX_WIDTH_PP.0);

                self.ops.push(DrawOp::Box {
                    rect: QRLayoutRect {
                        x,
                        y,
                        width: DEBTOR_BOX_WIDTH_PP,
                        height: DEBTOR_BOX_WIDTH_PP,
                    },
                });

                y = Mm(y.0 - self.extra_spacing.0);
            }
        }
    }

    pub fn compute_information_spacing(&mut self) -> bool {
        let mut text_lines = 0usize;
        let mut extra_blocks = 0usize;
        let mut fixed_height = Mm(0.0);

        // Payable to
        text_lines += 1;
        text_lines += self.payable_to_lines.len();

        // Reference
        if self.reference.is_some() {
            extra_blocks += 1;
            text_lines += 1; // label
            text_lines += 1; // value
        }

        // Additional info
        if let Some(lines) = self.additional_info_lines {
            extra_blocks += 1;
            text_lines += 1; // label
            text_lines += lines.len();
        }

        extra_blocks += 1;

        match self.payable_by_lines {
            Some(lines) => {
                text_lines += 1; // label
                text_lines += lines.len();
            }
            None => {
                text_lines += 1; // label only
                fixed_height = Mm(fixed_height.0 + Self::DEBTOR_BOX_HEIGHT_PP.0);
            }
        }

        let spacing = compute_spacing(
            Self::PP_INFO_SECTION_MAX_HEIGHT,
            fixed_height,
            text_lines,
            extra_blocks,
            self.line_spacing,
        );

        self.extra_spacing = spacing.extra_spacing;
        self.line_spacing = spacing.line_spacing;

        spacing.extra_spacing.0 / spacing.line_spacing.0 < 0.8
    }

    fn draw_label(&mut self, text: String, x: Mm, y: &mut Mm) {
        self.ops.push(DrawOp::Text {
            text,
            at: Baseline { x, y: *y },
            size: self.label_font_size,
            bold: true,
        });

        *y = Mm(y.0 - self.line_spacing.0);
    }

    fn draw_single_line(&mut self, text: String, x: Mm, y: &mut Mm) {
        self.ops.push(DrawOp::Text {
            text,
            at: Baseline { x, y: *y },
            size: self.text_font_size,
            bold: false,
        });

        *y = Mm(y.0 - self.line_spacing.0 - self.extra_spacing.0);
    }

    fn draw_text_lines(&mut self, lines: &[String], x: Mm, y: &mut Mm) {
        for line in lines {
            self.ops.push(DrawOp::Text {
                text: line.to_string(),
                at: Baseline { x, y: *y },
                size: self.text_font_size,
                bold: false,
            });

            *y = Mm(y.0 - self.line_spacing.0);
        }

        *y = Mm(y.0 - self.extra_spacing.0);
    }
}
