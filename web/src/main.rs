/*
 * Copyright (c) 2026 Thomas Prosser
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */
mod components;
mod state;
mod utils;

use leptos::prelude::*;
use leptos::{component, ev};
use leptos::mount::mount_to_body;
use leptos::task::spawn_local;
use wasm_bindgen::JsCast;
use web_sys::{Blob, HtmlAnchorElement, HtmlInputElement, Url};

use swiss_qrust::{BillData, Language};
use crate::components::*;
use crate::utils::trigger_browser_download;

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
            match (&bill, selected_lang.get()) {
                Ok(bytes) => {
                    trigger_browser_download(bytes);
                    set_status.set("Bill generated!".into());
                }
                Err(e) => set_status.set(format!("Error: {e}")),
            }
        }
    };

    // 1. The Reactive Store (State)
    let bill = RwSignal::new(swiss_qrust::get_mock_bill());
    let (lang, set_lang) = signal(Language::De);

    // 2. The Realtime Preview Memo
    // This re-runs the layout engine every time 'bill' or 'lang' changes.
    let preview_svg = Memo::new(move |_| {
        bill.with(|data| {
            // Your layout engine -> Vec<DrawOp> -> SVG String
            render_bill_to_svg(data, lang.get())
        })
    });
    
    view! {
        <main class="min-h-screen bg-slate-100 flex flex-col lg:flex-row h-screen overflow-hidden font-sans">
            <Sidebar />
            <Preview />
        </main>
    }
}

