/*
 * Copyright (c) 2026 Thomas Prosser
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */

use std::str::FromStr;

use leptos::prelude::*;
use swiss_qrust::{label, Address, BillData, Country, Currency, Language, ReferenceType};
use crate::bui_language::{get_gui_label, Translatable};

#[derive(Copy, Clone)]
pub struct AppState {
    pub bill: RwSignal<BillData>,
    pub lang: RwSignal<Language>,
    pub status: RwSignal<String>,
    pub creditor_address: RwSignal<Address>,
    pub debtor_address: RwSignal<Option<Address>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            bill: RwSignal::new(seed_bill()),
            lang: RwSignal::new(Language::De),
            status: RwSignal::new("Ready".to_string()),
            creditor_address: RwSignal::new(seed_bill().creditor_address),
            debtor_address: RwSignal::new(None),
        }
    }

    pub fn t(&self, key: Translatable) -> Signal<String> {
        let lang = self.lang;

        // Signal::derive takes a move closure and makes it a Signal
        Signal::derive(move || {
            match key {
                Translatable::Lib(k) => label(k, lang.get()).unwrap_or("Error").to_string(),
                Translatable::Gui(k) => get_gui_label(k, lang.get()).to_string(),
            }
        })
    }}

pub fn seed_bill() -> BillData {

    let iban = "CH9300762011623852957";
    let currency = Currency::from_str("CHF").unwrap();
    // let reference = "210000000003139471430009017";

    let creditor_address = Address {
        address_type: "S".to_string(),
        name: "Robert Schneider AG".to_string(),
        street: None,
        house_num: None,
        plz:  "2501".to_string(),
        city:  "Biel".to_string(),
        country: Country::CH,
    };

    BillData {
        iban: iban.to_string(),
        currency,
        amount: None,
        reference_type: ReferenceType::NoRef,
        unstructured_message: None,
        bill_information: None,
        creditor_address,
        debtor_address: None,
        alternative_schemes: [None, None],
    }
}
