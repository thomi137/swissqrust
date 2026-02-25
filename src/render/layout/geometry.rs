/*
 * Copyright (c) 2026 Thomas Prosser 
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */
use std::ops::{Add, Mul, Sub};

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

impl Add for Mm {
    type Output = Mm;

    fn add(self, rhs: Self) -> Self::Output {
        Mm(self.0 + rhs.0)
    }
}

impl Sub for Mm {
    type Output = Mm;

    fn sub(self, rhs: Self) -> Self::Output {
        Mm(self.0 - rhs.0)
    }
}

impl Mul for Mm {
    type Output = Mm;

    fn mul(self, rhs: Self) -> Self::Output {
        Mm(self.0 * rhs.0)
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
