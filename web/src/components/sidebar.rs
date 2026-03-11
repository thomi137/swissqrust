/*
 * Copyright (c) 2026 Thomas Prosser 
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */

use leptos::prelude::*;
use leptos::{component, view, IntoView};

use crate::state::AppState;
use crate::trigger_browser_download;

use swiss_qrust::Language;
use swiss_qrust::pdf::render_bill_to_pdf;

#[component]
pub fn Sidebar() -> impl IntoView {

    // 1. Access the global state
    let state = use_context::<AppState>().expect("State context missing");

    // 2. Action for PDF Generation
    let on_download = move |_| {
        state.status.set("Generating PDF...".into());

        let bill_data = state.bill.get();
        let lang = state.lang.get();

        // Use your existing PDF engine
        match render_bill_to_pdf(&bill_data, lang) {
            Ok(bytes) => {
                trigger_browser_download(bytes);
                state.status.set("PDF Downloaded!".into());
            }
            Err(e) => state.status.set(format!("Error: {e}")),
        }
    };

    view! {
            <aside class="lg:w-[450px] p-8 overflow-y-auto bg-white shadow-2xl border-r-4 border-red-600 z-10">
                <div class="mb-10">
                    <h1 class="text-4xl font-black text-slate-900 leading-tight">"Swiss QR"</h1>
                    <p class="text-red-600 font-bold uppercase tracking-tighter text-sm">"Enterprise Edition"</p>
                </div>

                <div class="space-y-8">
                    <div class="group flex flex-col gap-2">
                        <label class="text-xs font-black text-slate-400 group-focus-within:text-red-600 transition-colors">"IBAN"</label>
                        <input
                            type="text"
                            class="p-4 bg-slate-50 rounded-2xl border-2 border-transparent focus:border-red-600 outline-none transition-all font-mono"
                            prop:value= move || state.bill.get().iban.clone()
                            on:input= move |ev| {
                                state.bill.update(|b| b.iban = event_target_value(&ev));
                            }
                        />
                    </div>

                    // INPUT: Amount
                    <div class="flex flex-col gap-2">
                        <label class="text-xs font-black text-slate-400">"AMOUNT"</label>
                        <input
                            type="number"
                            step="0.01"
                            class="p-4 bg-slate-50 rounded-2xl border-2 border-transparent focus:border-red-600 outline-none transition-all"
                            on:input=move |ev| {
                            let val = event_target_value(&ev).parse::<f64>().unwrap_or(0.0);
                                state.bill.update(|b| b.amount = Some(format!("{:.2}", val)));
                            }
                        />
                    </div>

                    <div class="grid grid-cols-2 gap-4">
                    <div class="flex flex-col gap-1">
                        <label class="text-[10px] font-black text-slate-400">"AMOUNT"</label>
                        <input
                            type="number"
                            step="0.01"
                            placeholder="0.00"
                            class="p-3 bg-slate-50 rounded-xl border-2 border-transparent focus:border-swiss-red outline-none transition-all font-bold"
                            on:input=move |ev| {
                                let val = event_target_value(&ev).parse::<f64>().unwrap_or(0.0);
                                state.bill.update(|b| b.amount = Some(format!("{:.2}", val)));
                            }
                        />
                    </div>
                    <div class="flex flex-col gap-1">
                        <label class="text-[10px] font-black text-slate-400">"CURRENCY"</label>
                        <select
                            class="p-3 bg-slate-50 rounded-xl border-2 border-transparent focus:border-swiss-red outline-none appearance-none font-bold"
                            on:change=move |ev| {
                                let val = event_target_value(&ev);
                                state.bill.update(|b| b.currency = val.parse().unwrap_or_default());
                            }
                            >
                            <option value="CHF">"CHF"</option>
                            <option value="EUR">"EUR"</option>
                        </select>
                    </div>
                </div>

                // Language Toggle
                <div class="flex flex-col gap-2">
                    <label class="text-[10px] font-black text-slate-400 uppercase">"Language"</label>
                    <div class="flex bg-slate-100 p-1 rounded-xl">
                        {vec![Language::De, Language::Fr, Language::It, Language::En].into_iter().map(|l| {
                            let is_active = move || state.lang.get() == l;
                            view! {
                                <button
                                    class=move || format!("flex-1 py-2 text-xs font-bold rounded-lg transition-all {}",
                                        if is_active() { "bg-white shadow text-slate-900" } else { "text-slate-400 hover:text-slate-600" })
                                    on:click=move |_| state.lang.set(l)
                                >
                                    {format!("{:?}", l)}
                                </button>
                            }
                        }).collect_view()}
                    </div>
                </div>

                // Action Button
                <div class="pt-6 border-t border-slate-100 mt-10">
                    <button
                        on:click=on_download
                        class="group relative w-full py-5 bg-slate-900 text-white font-black rounded-2xl shadow-xl hover:bg-swiss-red active:scale-95 transition-all overflow-hidden"
                    >
                        <span class="relative z-10 tracking-widest uppercase text-xs">"Download Official PDF"</span>
                        <div class="absolute inset-0 bg-swiss-red translate-y-full group-hover:translate-y-0 transition-transform duration-300" />
                    </button>

                    <p class="mt-4 text-center text-[10px] font-bold text-slate-300 uppercase tracking-widest">
                        {move || state.status.get()}
                    </p>
                </div>
            </div>
         </aside>
    }
}