/*
 * Copyright (c) 2026 Thomas Prosser 
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */

include!("generated/countries.rs");
include!("generated/cross.rs");
include!("generated/corner_marks_amount.rs");
include!("generated/corner_marks_payable_by.rs");
include!("generated/scissors.rs");

pub mod bill;
pub mod render;
pub mod constants;
pub mod language;
pub mod utils;
pub mod validators;
pub mod cli;

pub use bill::*;
pub use render::*;
pub use constants::*;
pub use language::*;
pub use utils::SliceExt;

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

    pub trait SliceExt<T> {
    fn all_but_last(&self) -> &[T];
}

    impl<T> SliceExt<T> for [T] {
        fn all_but_last(&self) -> &[T] {
            if self.is_empty() {
                &[]
            } else {
                &self[..self.len() - 1]
            }
        }
    }
    
    

}


