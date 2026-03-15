/*
 * Copyright (c) 2026 Thomas Prosser 
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */

use leptos::prelude::*;
use leptos::{component, view, IntoView};

use crate::state::AppState;


use swiss_qrust::{label, Language};
use swiss_qrust::pdf::render_bill_to_pdf;
use crate::components::address_component::AddressComponent;
use crate::components::widgets::{CountingTextArea, ToggleSwitch};
use crate::utils::trigger_browser_download;

#[component]
pub fn Sidebar() -> impl IntoView {

    // 1. Access the global state
    let state = use_context::<AppState>().expect("State context missing");

    let (is_expanded, set_is_expanded) = signal(false);

    // 1.1 Just local state. No need to put it into app state
    let (has_debtor, set_has_debtor) = signal(false);

    Effect::new(move |_| {
        if has_debtor.get() {
            // Delay slightly or wait for transition
            set_is_expanded.set(true);
        } else {
            set_is_expanded.set(false);
        }
    });

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

    let mask = move |val: &str| {
        val.as_bytes()
            .chunks(4)
            .map(|c| std::str::from_utf8(c).unwrap())
            .collect::<Vec<_>>()
            .join(" ")
    };

    let iban_error = Memo::new(move |_| {
        state.bill.with(|data| {
            if data.iban.is_empty() {
                Some(swiss_qrust::validators::IbanError::IncorrectLength {
                    expected: 21,
                    actual: 0,
                })
            } else if let Err(e) = swiss_qrust::is_valid_iban(&data.iban) {
                Some(swiss_qrust::validators::IbanError::InvalidIban)
            } else {
                None
            }
        })
    });

    let section_card = "p-6 my-8 bg-slate-50 border border-slate-200 rounded-2xl shadow-sm hover:shadow-md transition-shadow duration-300";

    view! {
    <div class="mb-10">
        <h1 class="text-4xl font-black text-slate-900 leading-tight">"Swiss QR"</h1>
        <p class="text-red-600 font-bold uppercase tracking-tighter text-sm">"Enterprise Edition"</p>
    </div>
    <div class=section_card>
        <div class="group flex flex-col mb-6">
            <label class="text-xs font-black text-slate-400 group-focus-within:text-red-600 transition-colors">"IBAN"</label>
            <input
                type="text"
                class=move || format!(
                "w-full p-2 border rounded font-mono outline-none {};",
                    if iban_error.get().is_some() { "border-red-500 bg-red-50" } else { "border-slate-300" }
                )
                prop:value= move || mask(&*state.bill.get().iban)
                 on:input=move |ev| {
                    let val = event_target_value(&ev).replace(" ", "").to_uppercase();
                    // Simple mask: Add spaces every 4 chars
                    let masked = mask(&val);
                    state.bill.update(|b| b.iban = masked);
                }
                />
        </div>
            <AddressComponent
                title=move || label!(AccountPayableTo, state.lang.get())
                address=Signal::derive(move || state.bill.get().creditor_address)
                on_change=move |new_addr| state.bill.update(|b| b.creditor_address = new_addr)
            />
    </div>
        // Reference gets in here

    // Additional Information
    <div class=section_card>
    <CountingTextArea
        label=move || label!(AdditionalInformation, state.lang.get())
        value=Signal::derive(move || {
            state.bill.get().unstructured_message.unwrap_or_default()
        })
        on_input=move |new_info| {
            state.bill.update(|b| {
                // Write: Update the field inside the signal
                b.unstructured_message = if new_info.is_empty() { None } else { Some(new_info) };
            });
        }
        max_length=140
    />
    </div>

    <ToggleSwitch
        has_debtor=has_debtor
        set_has_debtor=set_has_debtor/>

    <div class=move || {
        let grid_style = if has_debtor.get() {
            "grid-rows-1 opacity-100"
        } else {
            "grid-rows-0 opacity-0"
        };

        format!(
            "grid transition-all duration-500 ease-in-out {}",grid_style)
        }>
    <div class=move || format!("{} {}",
        if is_expanded.get() { "overflow-visible" } else { "overflow-hidden" },
        "transition-all"
    )>
        <div class=format!("{section_card}")>
        <AddressComponent
            title=label!(PayableBy, state.lang.get())
            address=Signal::derive(move || state.bill.get().debtor_address.unwrap_or_default())
            on_change=move |new_addr| state.bill.update(|b| b.debtor_address = Some(new_addr))
        />
        </div>
    </div>
    </div>

    // Amount Section
    <div class=section_card>
        <div class="grid grid-cols-2 gap-4">
            <div class="flex flex-col gap-1">
                <label class="text-[10px] font-black text-slate-400 uppercase">
                    {move || label!(Amount, state.lang.get())}
                </label>
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
            </div>// column 1
        <div class="flex flex-col gap-1">
            <label class="text-[10px] font-black text-slate-400 uppercase">
                {move || label!(Currency, state.lang.get())}
            </label>
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
        </div> // column 2
    </div> // grid
    </div> // section card

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
    </div> // Amount section

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
    }
}