/*
 * Copyright (c) 2026 Thomas Prosser 
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */

use leptos::prelude::*;
use leptos::{component, view, IntoView};
use leptos::prelude::{use_context, Memo};
use swiss_qrust::BillError::IbanError;
use swiss_qrust::svg::render_bill_to_svg;

use crate::state::AppState;

#[component]
pub fn Preview() -> impl IntoView {

    let state = use_context::<AppState>().expect("state missing");
    
    let preview_svg = Memo::new(move |_| {
        state.bill.with(|data| {
            render_bill_to_svg(data, state.lang.get()).unwrap_or_else(|e| format!("<text y='20' fill='red'>Render Error: {}</text>", e))
        })
    });

    view! {
        <div class="relative bg-white shadow-[0_35px_60px_-15px_rgba(0,0,0,0.3)] w-full max-w-[210mm] aspect-[2/1] transition-all duration-300 transform hover:scale-[1.01]">
            <div class="w-full h-full" inner_html=move || preview_svg.get() />
            <div class="absolute top-0 left-0 w-8 h-8 border-t-2 border-l-2 border-slate-100" />
            <div class="absolute bottom-0 right-0 w-8 h-8 border-b-2 border-r-2 border-slate-100" />
        </div>
    }
}