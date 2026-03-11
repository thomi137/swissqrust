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
use web_sys::{HtmlInputElement};

mod components;
mod state;
mod utils;

use swiss_qrust::{BillData, Language};
use swiss_qrust::pdf::render_bill_to_pdf;
use crate::components::*;
use crate::utils::{trigger_browser_download};

fn main() {
    {
        console_error_panic_hook::set_once();
        mount_to_body(|| view! { <App /> });
    }
}

#[component]
fn App() -> impl IntoView {

    let state = state::AppState::new();
    provide_context(state);

    view! {
        <main class="min-h-screen bg-slate-100 flex flex-col lg:flex-row h-screen overflow-hidden font-sans">
            <Sidebar />
            <Preview />
        </main>
    }
}

