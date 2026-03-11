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

/* -- DUMP ---
 // --- 1. The File Picker Handler ---
    let on_file_change = move |ev: ev::Event| {
        let target = event_target::<HtmlInputElement>(&ev);
        if let Some(file) = target.files().and_then(|f| f.get(0)) {
            let name = file.name();
            let ext = name.split('.').last().unwrap_or("").to_string();
            set_status.set(format!("Reading {}...", name));

            spawn_local(async move {
                let text_js = wasm_bindgen_futures::JsFuture::from(file.text()).await.unwrap();
                let text = text_js.as_string().unwrap();

                // Call your shared lib parsing logic
                match swiss_qrust::parse_bill_data(&text, &ext) {
                    Ok(input_bill) => {
                        // Using your existing TryFrom logic
                        match BillData::try_from(input_bill) {
                            Ok(data) => {
                                set_bill_data.set(Some(data));
                                set_status.set(format!("Loaded: {}", name));
                            }
                            Err(e) => set_status.set(format!("Validation Error: {}", e)),
                        }
                    }
                    Err(e) => set_status.set(format!("Parse Error: {}", e)),
                }
            });
        }
    };

    // --- 2. The PDF Button Handler ---
    let on_generate = move |_| {
        if let Some(bill) = bill_data.get() {
            // CALL the function first, THEN match the Result
            match render_bill_to_pdf(&bill, selected_lang.get()) {
                Ok(bytes) => {
                    trigger_browser_download(bytes);
                    set_status.set("Bill generated!".into());
                }
                Err(e) => set_status.set(format!("Error: {e:?}")),
            }
        }
    };

 */

