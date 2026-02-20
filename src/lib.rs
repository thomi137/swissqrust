include!("generated/countries.rs");
include!("generated/cross.rs");
include!("generated/corner_marks_amount.rs");
include!("generated/corner_marks_payable_by.rs");

pub mod cli;
pub mod validators;
pub mod utils;
pub mod bill;
pub mod language;
pub mod render;
pub mod pdf;
pub mod layout;

pub use bill::*;
pub use language::*;
pub use layout::*;
pub use pdf::*;

pub mod shapes {
    #[derive(Debug, Copy, Clone)]
    pub struct Rect {
        pub x: f64,
        pub y: f64,
        pub width: f64,
        pub height: f64,
    }

    #[derive(Debug)]
    pub struct Polygon {
        pub points: &'static [(f64, f64)],
    }
}

pub mod constants {
    use crate::{Mm, Pt, MM_PER_PT};

    pub const FONT_SIZE_TITLE: Pt = Pt(11f32);
    pub const PP_LABEL_PREF_FONT_SIZE: Pt = Pt(8f32); // pt
    pub const PP_TEXT_PREF_FONT_SIZE: Pt = Pt(10f32); // pt
    pub const PP_TEXT_MIN_FONT_SIZE: Pt = Pt(8f32); // pt


    const PAYMENT_PART_WIDTH: u32 = 148; // mm
    const PP_AMOUNT_SECTION_WIDTH: u32 = 46; // mm
    const PP_INFO_SECTION_WIDTH: u32 = 87; // mm
    const BOX_TOP_PADDING: f32= 2f32 * MM_PER_PT; // mm


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

pub mod formatters {
    pub trait SwissQRFormatter {
        fn format_iban(&self) -> String;
        fn format_qr_reference(&self) -> String;
        fn format_scor_reference(&self) -> String;
        fn format_amount(&self) -> String;
    }

    impl SwissQRFormatter for str {

        ///
        /// ```
        /// # use swiss_qrust::formatters::SwissQRFormatter;
        /// assert_eq!("CH6431961000004421557".format_iban(), "CH64 3196 1000 0044 2155 7")
        /// ```
        fn format_iban(&self) -> String {
            let cleaned: String = self.chars().filter(|c| !c.is_whitespace()).collect();
            cleaned.as_bytes()
                .chunks(4)
                .map(|c| std::str::from_utf8(c).unwrap())
                .collect::<Vec<&str>>()
                .join(" ")
        }

        fn format_qr_reference(&self) -> String {
            let cleaned: String = self.chars().filter(|c| !c.is_whitespace()).collect();
            cleaned.as_bytes()
                .rchunks(5)
                .rev()
                .map(|c| std::str::from_utf8(c).unwrap())
                .collect::<Vec<&str>>()
                .join(" ")
        }

        fn format_scor_reference(&self) -> String {
            let cleaned: String = self.chars().filter(|c| !c.is_whitespace()).collect();
            cleaned.as_bytes()
                .chunks(4)
                .map(|c| std::str::from_utf8(c).unwrap())
                .collect::<Vec<&str>>()
                .join(" ")
        }

        fn format_amount(&self) -> String {
            let parts: Vec<&str> = self.split('.').collect();
            let int_part = parts[0];
            let formatted_int = int_part.as_bytes()
                .rchunks(3)
                .rev()
                .map(|c| std::str::from_utf8(c).unwrap())
                .collect::<Vec<&str>>()
                .join(" ");

            let dec_part = if parts.len() > 1 { parts[1] } else { "00" };
            format!("{}.{}", formatted_int, dec_part)
        }

    }

}


