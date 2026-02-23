/*
 * Copyright (c) 2026 Thomas Prosser
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */

/// Formatting strings for printing in PDF
/// The (SIX Specificationà)[https://www.six-group.com/dam/download/banking-services/standardization/qr-bill/ig-qr-bill-v2.3-de.pdf]
/// expects these to be adhered to.
pub trait SwissQRFormatter {
    fn format_iban(&self) -> String;
    fn format_qr_reference(&self) -> String;
    fn format_scor_reference(&self) -> String;
    fn format_amount(&self) -> String;
}

impl SwissQRFormatter for str {

    ///
    /// ```
    /// # use swiss_qrust::traits::SwissQRFormatter;
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
    fn all_but_first(&self) -> &[T];
}

impl<T> SliceExt<T> for [T] {
    fn all_but_last(&self) -> &[T] {
        self.get(..self.len().saturating_sub(1)).unwrap_or(&[])
    }

    fn all_but_first(&self) -> &[T] {
        self.get(1..).unwrap_or(&[])
    }
}


