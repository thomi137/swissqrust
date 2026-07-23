/*
 * Copyright (c) 2026 Thomas Prosser 
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */
use crate::{Mm, Pt};

/// Style Guide p.15 "Beschriftung als Muster": Zeilenabstand Zahlteil.
pub const PP_LINE_SPACING: Pt = Pt(11f32);
/// Style Guide p.15: Zeilenabstand "Betrag Z" (differs from the general 11pt).
pub const PP_AMOUNT_LINE_SPACING: Pt = Pt(13f32);
/// Style Guide p.15: Zeilenabstand "Weitere Informationen Z" (Alternative procedures).
pub const PP_FURTHER_INFO_LINE_SPACING: Pt = Pt(8f32);

pub const PAYMENT_PART_HORI_OFFSET: Mm = Mm(62f32);
pub const QR_CODE_HEIGHT: Mm = Mm(46f32);
pub const QR_CODE_WIDTH: Mm = Mm(46f32);
pub const PP_INFO_SECTION_HORI_OFFSET: Mm = Mm(51f32);
pub const RC_INFO_SECTION_HORI_OFFSET: Mm = Mm(5f32);
pub const PP_AMOUNT_SECTION_TOP: Mm = Mm(37f32);
pub const CURRENCY_WIDTH_PP: Mm = Mm(15f32);
pub const DEBTOR_BOX_WIDTH_PP: Mm = Mm(65f32);
pub const DEBTOR_BOX_HEIGHT: Mm = Mm(25f32);
pub const AMOUNT_BOX_WIDTH_PP: Mm = Mm(40f32);
pub const AMOUNT_BOX_HEIGHT_PP: Mm = Mm(15f32);
pub const PAYMENT_PART_MAX_HEIGHT: Mm = Mm(95f32);
pub const RECEIPT_PART_MAX_HEIGHT: Mm = Mm(95f32);

/// Top edge of "Bereich Weitere Informationen" (Further information
/// section), measured in mm from the bottom of the page - matches the debug
/// overlay's `rect_mm(67, 5, 138, 10)` box. Payment part only.
pub const PP_FURTHER_INFO_SECTION_TOP: Mm = Mm(15f32);
/// Style Guide 3.4/3.5.5: font size for the Alternative procedures lines.
pub const PP_FURTHER_INFO_TEXT_SIZE: Pt = Pt(7f32);
