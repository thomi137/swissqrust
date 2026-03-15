/*
 * Copyright (c) 2026 Thomas Prosser
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */

use leptos::prelude::*;
use leptos::{component, html, ev};
use leptos::logging::log;

use swiss_qrust::{Country, LabelKey};
use strum::IntoEnumIterator;
use swiss_qrust::LabelKey::AdditionalInformation;
use crate::bui_language::Translatable;
use crate::state::AppState;

#[component]
pub fn FormField<F>(
    #[prop(into)] label: Signal<String>,
    #[prop(into)] value: Signal<String>,
    on_input: F,
    #[prop(into)] error: Memo<Option<String>>,
    #[prop(optional, into)] class: String,
    #[prop(optional)] placeholder: &'static str,
) -> impl IntoView
where
    F: Fn(String) + 'static
{
    let state = use_context::<AppState>().unwrap();
    let has_error = move || error.get().is_some();

    view! {
    <div class=format!("flex flex-col gap-1 {}", class)>
        <label class="text-[10px] font-bold text-slate-500 uppercase tracking-wider">
            {label}
        </label>
        <div class="relative group">
            <input
                type="text"
                placeholder=placeholder
                class=move || format!(
                    "w-full p-2.5 text-sm font-medium border rounded-lg outline-none transition-all duration-200 \
                    {} group-hover:border-slate-400 focus:ring-2",
                    if has_error() {
                        "border-red-400 bg-red-50 focus:ring-swiss-red"
                    } else {
                        "border-slate-200 bg-white focus:ring-blue-100 focus:border-blue-500"
                    }
                )
                prop:value=value
                on:input=move |ev| on_input(event_target_value(&ev))
            />

            // Error Icon
            <Show when=has_error>
                <div class="absolute right-3 top-1/2 -translate-y-1/2 text-red-500 animate-in zoom-in">
                    <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                              d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
                    </svg>
                </div>
            </Show>
        </div>

        // Error Message
        <div class="h-4"> // Fixed height prevents layout shift
            <Show when=has_error>
                <p class="text-[10px] text-swiss-red font-medium animate-in slide-in-from-top-1">
                    {move || error.get()}
                </p>
            </Show>
        </div>
    </div>
    }
}

#[component]
pub fn ToggleSwitch(
    #[prop(into)] has_debtor: Signal<bool>,
    #[prop(into)] set_has_debtor: SignalSetter<bool>
) -> impl IntoView {
    view! {
        <div class="flex items-center justify-between p-4 bg-slate-50 rounded-lg border border-slate-200">
            <div class="flex flex-col">
                <span class="text-sm font-bold text-slate-700">"Debtor Details"</span>
                <span class="text-xs text-slate-500">"Add an optional recipient address"</span>
            </div>
            <label class="relative inline-flex items-center cursor-pointer">
                <input type="checkbox" class="sr-only peer"
                    prop:checked=has_debtor.get()
                    on:change= move |_| set_has_debtor.set(!has_debtor.get()) />
                <div class="w-11 h-6 bg-slate-300 peer-focus:outline-none rounded-full peer
                                peer-checked:after:translate-x-full peer-checked:after:border-white
                                after:content-[''] after:absolute after:top-[2px] after:left-[2px]
                                after:bg-white after:border-slate-300 after:border after:rounded-full
                                after:h-5 after:w-5 after:transition-all peer-checked:bg-swiss-red">
                </div>
            </label>
        </div>
    }
}

#[component]
pub fn CountingTextArea<F>(
    #[prop(into)] label: Signal<String>,
    #[prop(into)] value: Signal<String>,
    on_input: F,
    max_length: usize,
) -> impl IntoView
where
    F: Fn(String) + 'static
{
    let state = use_context::<AppState>().expect("State context missing");

    let current_len = move || value.get().len();
    let is_over = move || current_len() > max_length;

    view! {
    <div class="relative group">
        <textarea
            class=move || format!(
                "peer w-full min-h-[100px] p-4 pt-6 text-sm border-2 rounded-xl outline-none transition-all \
                {} group-hover:border-slate-300 focus:border-red-600 focus:ring-0 resize-none",
                if is_over() { "border-red-500 bg-red-50" } else { "border-slate-100 bg-white" }
            )
            placeholder=" " // Required for the CSS peer-placeholder-shown trick
            prop:value=value
            on:input=move |ev| on_input(event_target_value(&ev))
        />

        // The Floating Label
        <label class="absolute left-4 top-2 text-[10px] font-black uppercase text-slate-400 \
                      transition-all duration-200 pointer-events-none \
                      peer-placeholder-shown:top-6 peer-placeholder-shown:text-sm \
                      peer-placeholder-shown:font-bold peer-focus:top-2 peer-focus:text-[10px] \
                      peer-focus:text-red-600">
            {label}
        </label>

        // Character Counter
        <div class=move || format!(
            "absolute bottom-2 right-4 text-[10px] font-bold tracking-widest {}",
            if is_over() { "text-red-500" } else { "text-slate-300" }
        )>
            {move || format!("{}/{}", current_len(), max_length)}
        </div>
    </div>
}
}

#[component]
pub fn CountrySelector(
    #[prop(into)] value: Signal<Country>,
    on_change: Callback<Country>,
) -> impl IntoView {

    // Setup local state.
    let (search, set_search) = signal(String::new());
    let (is_open, set_is_open) = signal(false);

    let container_ref = NodeRef::<html::Div>::new();

    let _ = window_event_listener(ev::click, move |ev| {
        let Some(container) = container_ref.get() else { return; };

        // Check if the click target is NOT inside our container
        let target = event_target::<web_sys::Element>(&ev);
        if !container.contains(Some(&target)) {
            set_is_open.set(false);
            set_search.set(String::new());
        }
    });

    let all_countries = Memo::new(move |_| {
        let mut list: Vec<Country> = Country::iter().collect();
        // Custom sort: CH and LI first, then alphabetical by readable name
        list.sort_by(|a, b| {
            let priority = |c: &Country| match c {
                Country::CH => 0,
                Country::LI => 1,
                _ => 2,
            };
            priority(a).cmp(&priority(b)).then_with(|| a.to_string().cmp(&b.to_string()))
        });
        list
    });

    let filtered_countries = Memo::new(move |_| {
        let s = search.get().to_lowercase();
        log!("Searching for {}", s);
        all_countries.get().into_iter()
            .filter(|c| {
                let code = c.to_string().to_lowercase();
                let full_name = c.meta().name.to_lowercase();
                code.contains(&s) || full_name.contains(&s)
            })
            .collect::<Vec<_>>()
    });

    view! {
        <div node_ref=container_ref class="relative w-full z-50">

            <div
                class="border p-2 rounded cursor-pointer flex justify-between bg-white"
                on:click=move |_| set_is_open.update(|v| *v = !*v)
            >
                <span>{move || value.get().to_string()}</span>
                <span class="text-slate-400">"▾"</span>
            </div>

            <Show when= move || is_open.get()>
                <div class="absolute z-[100] w-full mt-1 bg-white border rounded shadow-lg max-h-60 overflow-y-auto\
                            animate-in fade-in-0 zoom-in-95 duration-200 origin-top">
                    <input
                        type="text"
                        class="w-full p-2 border-b sticky top-0 bg-slate-50 outline-none"
                        placeholder="Search country..."
                        prop:value=search
                        on:input=move |ev| set_search.set(event_target_value(&ev))
                        on:click=move |ev| ev.stop_propagation() // Prevent closing when clicking input
                    />

                <For
                    each=move || filtered_countries.get()
                    key=|c| c.to_string()
                    children=move |c| {
                        let c_clone = c.clone().meta();
                        view! {
                            <div
                                class="p-2 hover:bg-blue-50 cursor-pointer text-sm"
                                on:click=move |_| {
                                    on_change.run(c.clone());
                                    set_is_open.set(false);
                                    set_search.set(String::new());
                                }
                            >
                                {format!("{} - {}", c_clone.flag.unwrap(), c_clone.name)}
                            </div>
                        }
                    }
                />
                </div>
            </Show>
        </div>
    }
}
