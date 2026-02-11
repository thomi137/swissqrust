/*
 * Copyright (c) 2026 Thomas Prosser
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */
use crate::layout::geometry::{
    Layout,
    Mm
};

use crate::layout::payment_part_layout::{
    layout_payment_part_title
};

pub struct QRBillLayout {
    pub payment_title: String,
}

pub struct BillLayout {
    pub payment_title: String,
}

impl BillLayout {
    pub fn layout(&self) -> Layout {
        let mut ops = Vec::new();

        // TODO: inject ascender properly
        layout_payment_part_title(
            &mut ops,
            self.payment_title.as_str(),
            Mm(3.0),
        );

        Layout { ops }
    }
}

