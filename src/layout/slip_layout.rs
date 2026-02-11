/*
 * Copyright (c) 2026 Thomas Prosser 
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */

use crate::layout::geometry::{Mm, DrawOp};
use crate::layout::payment_part_layout::PaymentPartLayout;
use crate::layout::receipt_part_layout::ReceiptLayout;

const SLIP_WIDTH: Mm = Mm(210.0);
const SLIP_HEIGHT: Mm = Mm(105.0);
const RECEIPT_WIDTH: Mm = Mm(62.0);
const MARGIN: Mm = Mm(5.0);

pub struct SlipLayout<'a> {
    pub ops: Vec<DrawOp>,
    pub payment_part: PaymentPartLayout<'a>,
    pub receipt: ReceiptLayout<'a>,
}
