
use swiss_qrust::cli::*;
use clap::Parser;
use swiss_qrust::{qr_bill, Address, Currency, QRCountry, ReferenceType};
use swiss_qrust::BillData;
use swiss_qrust::qr_bill::QrBill;
use swiss_qrust::render::render_svg_to_png;
use swiss_qrust::svg::{add_swiss_cross, render_qr_svg};

fn main() -> Result<(), Box<dyn std::error::Error>> {

    const OUTPUT_PATH: &str = "examples/";

    let  creditor = Address::new(
        "Health insurance fit&kicking",
        Some("Am Wasser"),
        Some("1"),
        "3000",
        "Bern",
        "CH"
    ).unwrap();

    let debtor =  Address::new(
        "Sarah Beispiel",
        Some("Mustergasse"),
        Some("1"),
        "3600",
        "Thun",
        "CH").unwrap();

let bill_data = BillData::new(
    "CH64 3196 1000 0044 2155 7".to_string(),
    creditor,
    Some(debtor),
    QRCountry::CH,
    Currency::CHF,
    Some(String::from("111.00")),
    ReferenceType::infer("000008207791225857421286694")?,
    Some(String::from("Premium calculation July 2020")),
    None
    ).unwrap();

    let qr_bill = QrBill::new(bill_data).unwrap();
    let qr_text = QrBill::create_qr_text(&qr_bill).unwrap();
    let qr_code = qr_bill::encode_text_to_qr_code(&qr_text).unwrap();
    let doc = render_qr_svg(qr_code);
    let doc_cross = add_swiss_cross(doc);
    render_svg_to_png(&doc_cross, format!("{}/swiss_qr.png", OUTPUT_PATH))
        .expect("Failed to write File");

    Ok(())
}
