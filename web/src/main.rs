/*
 * Copyright (c) 2026 Thomas Prosser
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */

use leptos::prelude::*;
use leptos::{component, ev};
use leptos::mount::mount_to_body;
use leptos::task::spawn_local;
use wasm_bindgen::JsCast;
use web_sys::{Blob, HtmlAnchorElement, HtmlInputElement, Url};

use swiss_qrust::{BillData, Language};
use swiss_qrust::render_bill::render_bill_to_bytes;

fn trigger_browser_download(bytes: Vec<u8>) {

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


fn main() {
    {
        console_error_panic_hook::set_once();
        mount_to_body(|| view! { <App /> });
    }
}

#[component]
fn App() -> impl IntoView {
    let (selected_lang, set_lang) = signal(Language::De);
    let (bill_data, set_bill_data) = signal(None::<BillData>);
    let (status, set_status) = signal("Ready".to_string());

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
            match swiss_qrust::render_bill::render_bill_to_bytes(&bill, selected_lang.get()) {
                Ok(bytes) => {
                    trigger_browser_download(bytes);
                    set_status.set("Bill generated!".into());
                }
                Err(e) => set_status.set(format!("Error: {e}")),
            }
        }
    };

    view! {
        <main class="min-h-screen bg-slate-100 flex items-center justify-center p-6">
            <div class="bg-white shadow-2xl rounded-3xl p-10 w-full max-w-md border-t-8 border-red-600">
                <h1 class="text-4xl font-black text-slate-800 mb-8">"Swiss QR"</h1>

                <div class="space-y-6">
                    // File Picker
                    <div class="flex flex-col gap-2">
                        <label class="text-xs font-bold text-slate-500 uppercase tracking-widest">"1. Load Configuration"</label>
                        <input type="file" accept=".toml,.json" on:change=on_file_change
                            class="text-sm file:mr-4 file:py-2 file:px-4 file:rounded-full file:border-0 file:bg-red-50 file:text-red-700 hover:file:bg-red-100 cursor-pointer w-full"
                        />
                    </div>

                    // Language Picker
                    <div class="flex flex-col gap-2">
                        <label class="text-xs font-bold text-slate-500 uppercase tracking-widest">"2. Document Language"</label>
                        <select
                            class="w-full p-3 bg-slate-50 border-2 border-slate-200 rounded-xl focus:border-red-600 outline-none transition-all"
                            on:change=move |ev| {
                                let val = event_target_value(&ev);
                                set_lang.set(match val.as_str() {
                                    "Fr" => Language::Fr,
                                    "It" => Language::It,
                                    "En" => Language::En,
                                    _ => Language::De,
                                });
                            }
                        >
                            <option value="De">"Deutsch"</option>
                            <option value="Fr">"Français"</option>
                            <option value="It">"Italiano"</option>
                            <option value="En">"English"</option>
                        </select>
                    </div>

                    // Generate Button
                    <button
                        on:click=on_generate
                        prop:disabled=move || bill_data.get().is_none()
                        class="w-full bg-red-600 hover:bg-red-700 text-white font-black py-4 rounded-2xl shadow-xl transition-all active:scale-95 disabled:bg-slate-300 disabled:shadow-none mt-4"
                    >
                        "GENERATE PDF"
                    </button>

                    <p class="text-center text-slate-400 text-xs italic mt-4">
                        {move || status.get()}
                    </p>
                </div>
            </div>
        </main>
    }
}
