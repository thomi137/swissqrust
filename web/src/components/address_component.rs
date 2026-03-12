/*
 * Copyright (c) 2026 Thomas Prosser
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */

use leptos::prelude::*;
use leptos::component;
use swiss_qrust::{Address, AddressError};

use crate::components::widgets::FormField;

#[component]
pub fn AddressComponent<F>(
    title: &'static str,
    address: Signal<Address>, // Pass the signal from your state
    on_change: F,
    #[prop(optional, into)] class: String,
) -> impl IntoView
where F: Fn(Address) + Clone + 'static
{
    let update = on_change;

    view! {
        <div class=format!("space-y-1 {}", class)>
            <h3 class="text-xs font-black text-slate-800 mb-2 border-b border-slate-100 pb-1">{title}</h3>

            <FormField
                label="Name"
                placeholder="Company or Person Name"
                value=Signal::derive(move || address.get().name)
                on_input={
                    let update = update.clone();
                    move |v| { let mut a = address.get(); a.name = v; update(a); }
                }
                error=Memo::new(move |_| {
                    if address.get().name.is_empty() { Some("Name is required".into()) } else { None }
                })
            />

        <div class="grid grid-cols-4 gap-2">

            <FormField
                class="col-span-3"
                label="Street"
                value=Signal::derive(move || address.get().street.unwrap_or_default())
                on_input={
                    let update = update.clone();
                    move |v| { let mut a = address.get(); a.street = Some(v); update(a); }
                }
                error=Memo::new(|_| None) // Add library validation here
            />

            <FormField
                class="col-span-1"
                label="No"
                value=Signal::derive(move || address.get().house_num.unwrap_or_default())
                on_input={
                    let update = update.clone();
                    move |v| { let mut a = address.get(); a.house_num = Some(v); update(a); }
                }
                error=Memo::new(|_| None)
            />

        </div>

            <div class="grid grid-cols-4 gap-2">
                <FormField
                    class="col-span-1"
                    label="ZIP"
                    value=Signal::derive(move || address.get().plz)
                    on_input={
                        let update = update.clone();
                        move |v| { let mut a = address.get(); a.plz = v; update(a); }
                    }
                    error=Memo::new(move |_| {
                      if address.get().name.is_empty() { Some("PLZ is required".into()) } else { None }
                     })
                />
                <FormField
                    class="col-span-3"
                    label="City"
                    value=Signal::derive(move || address.get().city)
                    on_input={
                        let update = update.clone();
                        move |v| { let mut a = address.get(); a.city = v; update(a); }
                    }
                    error=Memo::new(move |_| {
                      if address.get().name.is_empty() { Some("City is required".into()) } else { None }
                     })
                />
            </div>
        </div>
    }
}
