/*
 * Copyright (c) 2026 Thomas Prosser
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum CountryStatus {
    Official,
    Deprecated { until: u16 },
    Reserved,
}

#[derive(Debug)]
pub struct CountryMeta {
    pub alpha2: &'static str,
    pub name: &'static str,
    pub status: CountryStatus,
}

