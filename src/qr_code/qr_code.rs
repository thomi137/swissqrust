use bincode::serialize;
use qrcode::{QrCode, Color};
use svg::{Document, Node};
use std::fs;
use svg::node::element::{Rectangle, Group};
use crate::bill::*;

use crate::address::Address;
pub fn binary_address(address: Address) -> bincode::Result<Vec<u8>> {
    return serialize(&address);
}

pub fn create_qr_svg(doc: &mut Document, data: &[u8], origin: (f64, f64)){
    let code = QrCode::new(data).expect("QR generation failed");

    let scale = QR_CODE_WIDTH as f64 / code.width() as f64;
    let mut grp = Group::new();
    for y in 0..code.width(){
        for x in 0..code.width() {
            if code[(x, y)] == Color::Dark {
                let rect = Rectangle::new()
                    .set("x", origin.0 + x as f64 * scale)
                    .set("y", origin.1 + y as f64 * scale)
                    .set("width", scale)
                    .set("height", scale)
                    .set("fill", "black");
                grp = grp.add(rect);
            }
        }
    }
    doc.append(grp);
}
