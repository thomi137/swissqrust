/*
 * Copyright (c) 2026 Thomas Prosser
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */

use anyhow::Result;
use swiss_qrust::build_bill::buid_bill;
use swiss_qrust::{create_pdf, Language};

fn main() -> Result<()> {

    let language = Language::It;
    let test_output_path = "./test_output/test_receipt.pdf";
    let bill = buid_bill()?;
    create_pdf(test_output_path, language, &bill)?;
    Ok(())

}
