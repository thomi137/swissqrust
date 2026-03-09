/*
 * Copyright (c) 2026 Thomas Prosser
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */

use leptos::prelude::RwSignal;
use swiss_qrust::{BillData, Language};

#[derive(Copy, Clone)]
pub struct AppState {
    pub bill: RwSignal<BillData>,
    pub lang: RwSignal<Language>,
    pub status: RwSignal<String>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            bill: RwSignal::new(swiss_qrust::get_mock_bill()),
            lang: RwSignal<Language>::new(Language::De),
            status: RwSignal::new("Ready".to_string()),
        }
    }
}
