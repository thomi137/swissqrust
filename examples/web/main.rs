/*
 * Copyright (c) 2026 Thomas Prosser
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */

use leptos::prelude::*;
use leptos::mount::mount_to_body;

fn main() {
    #[cfg(feature = "web")]
    {
        console_error_panic_hook::set_once();
        mount_to_body(|| view! { <App /> });
    }
}

#[component]
fn App() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-gray-100 flex flex-col items-center justify-center p-6">
            <div class="bg-white rounded-xl shadow-2xl p-8 max-w-md w-full border-t-8 border-red-600">
                <h1 class="text-4xl font-black text-gray-800 mb-2">"Swiss QR"</h1>
                <p class="text-red-600 font-bold tracking-widest uppercase text-sm mb-6">"Generator"</p>

                <button class="w-full bg-red-600 hover:bg-red-700 text-white font-bold py-3 px-6 rounded-lg transition duration-200 shadow-lg active:transform active:scale-95">
                    "Generate PDF"
                </button>
            </div>
        </div>
    }
}


