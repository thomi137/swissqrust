/*
 * Copyright (c) 2026 Thomas Prosser
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */

pub mod sidebar_component;
pub mod preview_component;
pub mod address_component;

pub use sidebar_component::*;
pub use preview_component::*;


pub mod widgets {

    use leptos::prelude::*;
    use leptos::component;
    use leptos::svg::set;

    #[component]
    pub fn FormField<F>(
        label: &'static str,
        #[prop(into)] value: Signal<String>,
        on_input: F,
        #[prop(into)] error: Memo<Option<String>>,
        #[prop(optional, into)] class: String,
        #[prop(optional)] placeholder: &'static str,
    ) -> impl IntoView
    where F: Fn(String) + 'static
    {
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
    pub fn ToggleSwitch (
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
} // widgets
