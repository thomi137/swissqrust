/*
 * Copyright (c) 2026 Thomas Prosser
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */
use leptos::attr::{data, value};
use leptos::prelude::*;
use leptos::component;
use swiss_qrust::{Address, AddressError, Country};
use crate::bui_language::{GuiLabelKey, Translatable};
use crate::components::widgets::{CountrySelector, FormField};
use crate::state;
use crate::state::AppState;
use crate::utils::fetch_city_by_plz;

#[component]
pub fn AddressComponent<F>(
    #[prop(into)] title: Signal<String>,
    address: Signal<Address>, // Pass the signal from your state
    on_change: F,
    #[prop(optional, into)] class: String,
) -> impl IntoView
where F: Fn(Address) + Clone + Send + Sync + 'static
{
    let state = use_context::<AppState>().expect("State context missing");
    let update = on_change.clone();

    let city_fetcher = LocalResource::new(move || {
        let plz = address.get().plz;
        async move {
            if plz.len() == 4 {
                fetch_city_by_plz(Country::CH, plz).await
            } else {
                None
            }
        }
    });

    let value = update.clone();
    Effect::new( move |_| {
        let data = city_fetcher.read();
        if let Some(Some(fetched_city)) = data.as_ref() {
            // Only update if the city is currently empty (don't overwrite user)
            if address.get().city.is_empty() {
                let mut a = address.get();
                a.city = fetched_city.to_string();
                value(a);
            }
        }
    });

    view! {
        <div class=format!("space-y-1 {}", class)>
            <h3 class="text-xs font-black text-slate-400 uppercase mb-2 border-b border-slate-100 pb-1">{title}</h3>

            <FormField
                label=state.t(Translatable::Gui(GuiLabelKey::Name))
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
                label=state.t(Translatable::Gui(GuiLabelKey::Street))
                value=Signal::derive(move || address.get().street.unwrap_or_default())
                on_input={
                    let update = update.clone();
                    move |v| { let mut a = address.get(); a.street = Some(v); update(a); }
                }
                error=Memo::new(|_| None) // Add library validation here
            />

            <FormField
                class="col-span-1"
                label=state.t(Translatable::Gui(GuiLabelKey::HouseNo))
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
                    label=state.t(Translatable::Gui(GuiLabelKey::PostalCode))
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
                    label=state.t(Translatable::Gui(GuiLabelKey::City))
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

            <CountrySelector
                value=Signal::derive(move || address.get().country)
                on_change=Callback::new( move |new_country| {
                    let mut a = address.get();
                    a.country = new_country;
                    update(a);
                })
            />

        </div>
    }
}
