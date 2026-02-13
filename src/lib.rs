include!("generated/countries.rs");
include!("generated/cross.rs");

pub mod cli;
pub mod validators;
pub mod utils;
pub mod bill;
pub mod language;
pub mod svg;
pub mod render;
pub mod pdf;
mod layout;

pub use bill::*;
pub use language::*;
pub use layout::*;

pub mod constants {
    use crate::Mm;

    pub const CURRENCY_WIDTH: Mm = Mm(12f32);

    // General payment slip measurments
    pub const SLIP_WIDTH: Mm = Mm(210f32);
    pub const SLIP_HEIGHT: Mm = Mm(105f32);
    pub const RECEIPT_WIDTH: Mm = Mm(62f32);// Payment Part
    pub const MARGIN: Mm = Mm(5f32);
    pub const DEBTOR_BOX_WIDTH_PP: Mm = Mm(65f32);
    pub const DEBTOR_BOX_HEIGHT: Mm = Mm(25f32);

    pub const AMOUNT_BOX_WIDTH_PP: Mm = Mm(51f32);
    pub const AMOUNT_BOX_HEIGHT_PP: Mm = Mm(22f32);
    pub const PAYMENT_PART_MAX_HEIGHT: Mm = Mm(95f32);
    // Receipt
    pub const CURRENCY_WIDTH_RC: Mm = Mm(12f32);
    pub const RECEIPT_MAX_HEIGHT: Mm = Mm(95f32);
    pub const RECEIPT_TEXT_WIDTH: Mm = Mm(52f32);
    
    pub const AMOUNT_BOX_WIDTH_RC: Mm = Mm(30f32); // mm
    pub const  AMOUNT_BOX_HEIGHT_RC: Mm = Mm(10f32); // mm
    pub const AMOUNT_SECTION_TOP: Mm = Mm(37f32);


    pub const DEBTOR_BOX_WIDTH_RC: Mm = Mm(52f32);
    pub const DEBTOR_BOX_HEIGHT_RC: Mm = Mm(20f32);
    pub const ACCEPTANCE_POINT_SECTION_TOP: Mm = Mm(23f32);

}

