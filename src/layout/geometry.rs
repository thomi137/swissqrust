/*
 * Copyright (c) 2026 Thomas Prosser 
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */

/// A4 Width in mm
pub const A4_PORTRAIT_WIDTH: f32 = 210f32;

/// A4 Height in mm
pub const A4_PORTRAIT_HEIGHT: f32 = 297f32;

/// Width of a QR Slip is incidentally the same as the whole paper slip 🤣
pub const QR_BILL_SLIP_WIDTH: u16 = 210;

/// Now the height of the slip itself
pub const QR_BILL_HEIGHT: u16 = 105;

/// Then the width with scissors symbol and all... Surprise
pub const QR_BILL_WITH_ALL_WIDTH: u16 = 210;

/// Takes 5 mm more with all the scissors assets.
pub const QR_BILL_WITH_HORI_LINE_HEIGHT: u16 = 110;
pub const QR_BILL_RC_WIDTH: u16 = 62; // mm
pub const QR_BILL_PC_WIDTH: u16 = 148; // mm

/// QR Width in mm
pub const QR_CODE_OVERALL_WIDTH: u16 = 148;

// Payment Part Font Sizes
const PP_LABEL_PREF_FONT_SIZE: u8 = 8; // pt
const PP_TEXT_PREF_FONT_SIZE: u8 = 10; // pt
const PP_TEXT_MIN_FONT_SIZE: u8 = 8; // pt

const RC_LABEL_PREF_FONT_SIZE: u8 = 6; // pt
const RC_TEXT_PREF_FONT_SIZE: u8 = 8; // pt

/// Constants to convert mms to pts and vv.
///
/// 1 point = 1/72 inch
/// 1 inch = 25.4 mm
pub const MM_PER_PT: f32 = 25.4 / 72.0;
pub const PT_PER_MM: f32 = 72.0 / 25.4;


/// Conversion structs. We use const fns to
/// do compile time evaluation and structs to
/// facilitate type checking up front.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Mm(pub f32);
impl Mm {
    pub const fn new(v: f32) -> Self {
        Mm(v)
    }

    pub fn to_pt(self) -> Pt {
        Pt(self.0 * PT_PER_MM)
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Pt(pub f32);
    impl Pt {
        pub const fn new(v: f32) -> Self {
            Pt(v)
        }

        pub fn to_mm(self) -> Mm {
            Mm(self.0 * MM_PER_PT)
        }
    }

// Rectangular area
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct QRBillLayoutRect {
    pub x: Mm,
    pub y: Mm,
    pub width: Mm,
    pub height: Mm,
}

// Text Baseline
#[derive(Copy, Clone, Debug)]
pub struct Baseline {
    pub x: Mm,
    pub y: Mm,
}

const FONT_SIZE_TITLE: u32 = 11; // pt
const RECEIPT_TEXT_WIDTH: u32 = 52; // mm
const PAYMENT_PART_WIDTH: u32 = 148; // mm
const PP_AMOUNT_SECTION_WIDTH: u32 = 46; // mm
const PP_INFO_SECTION_WIDTH: u32 = 87; // mm
const BOX_TOP_PADDING: f32= 2f32 * MM_PER_PT; // mm
const DEBTOR_BOX_WIDTH_RC: u32 = 52; // mm
const DEBTOR_BOX_HEIGHT_RC: u32 = 20; // mm

// Drawing operations
pub enum DrawOp {
    Text {
        text: String,
        at: Baseline,
        size: Pt,
        bold: bool,
    },

    TextLines {
        lines: Vec<String>,
        start: Baseline,
        size: Pt,
        leading: Mm,
    },

    Box {
        rect: QRBillLayoutRect,
    },

    Line {
        from: (Mm, Mm),
        to: (Mm, Mm),
        width: Mm,
    },

    QrCodeSpace {
        at: (Mm, Mm),
        size: Mm,
    },
}

pub struct Layout {
    pub ops: Vec<DrawOp>,
}
