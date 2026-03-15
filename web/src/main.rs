/*
 * Copyright (c) 2026 Thomas Prosser
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */

use leptos::prelude::*;
use leptos::{component};
use leptos::mount::mount_to_body;

use wasm_bindgen::JsCast;

mod components;
mod state;
mod utils;
pub mod bui_language;

use crate::components::*;

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
        <main class="min-h-screen bg-slate-50 flex justify-center p-8">
        <div class="w-full max-w-7xl flex flex-col lg:flex-row">

            <aside class="lg:w-[450px] p-8 overflow-y-auto bg-white shadow-2xl border-r-4 border-swiss-red z-10">
                <Sidebar />
            </aside>


            <section class="flex-1 relative">
                <div class="sticky top-8 flex flex-col items-center">
                    <Preview />
                </div>
            </section>
            </div>
        </main>
    }
}

