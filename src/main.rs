/*
 * Copyright (c) 2026 Thomas Prosser
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */

use anyhow::Result;
use swiss_qrust::build_bill::buid_bill;
use swiss_qrust::create_pdf;
use swiss_qrust::render::layout::*;

fn main() -> Result<()> {

    let test_output_path = "./examples/test_receipt.pdf";
    let bill = buid_bill()?;
    create_pdf(test_output_path, &bill)?;
    Ok(())

}
