/*
 * Copyright (c) 2026 Thomas Prosser 
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */

use wasm_bindgen::JsCast;
use web_sys::{Blob, HtmlAnchorElement, Url};

pub fn trigger_browser_download(bytes: Vec<u8>) {

    let blob_parts = js_sys::Array::new();
    blob_parts.push(&js_sys::Uint8Array::from(&bytes[..]));

    // In newer web-sys, options are a separate bag
    let mut options = web_sys::BlobPropertyBag::new();
    options.set_type("application/pdf");

    let blob = Blob::new_with_u8_array_sequence_and_options(&blob_parts, &options).unwrap();
    let url = Url::create_object_url_with_blob(&blob).unwrap();

    let document = web_sys::window().unwrap().document().unwrap();
    let link = document.create_element("a").unwrap().unchecked_into::<HtmlAnchorElement>();

    link.set_href(&url);
    link.set_download("Swiss-QR-Bill.pdf");
    link.click();

    let _ = Url::revoke_object_url(&url);
}

