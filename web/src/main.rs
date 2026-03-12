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
        <main class="flex h-screen overflow-hidden bg-slate-50">

            <aside class="lg:w-[450px] p-8 overflow-y-auto bg-white shadow-2xl border-r-4 border-swiss-red z-10">
                <Sidebar />
            </aside>


            <section class="flex-1 overflow-y-auto bg-slate-100 p-12">
                <div class="sticky top-0 flex flex-col items-center gap-8">
                    <Preview />
                    <div class="px-6 py-3 bg-white/80 backdrop-blur rounded-full shadow-sm border border-slate-200 text-xs font-bold text-slate-500 uppercase tracking-widest">
                        {move || state.status.get()}
                    </div>
                </div>
            </section>

        </main>
    }
}

