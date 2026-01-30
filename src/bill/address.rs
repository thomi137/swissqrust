/*
 * Copyright (c) 2026 Thomas Prosser 
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */

use serde::{Serialize, Deserialize};

/// No need for unstructured address
/// as deprecated on Nov 21, 2025
#[derive(Debug, Serialize, Deserialize)]
pub struct Address {
    name: String,
    street: Option<String>,
    house_num: Option<String>,
    plz: String,
    city: String,
    country: String,
}

impl Address {
    pub fn new(
        name: &str,
        street: Option<&str>,
        house_num: Option<&str>,
        plz: &str,
        city: &str,
        country: &str,
    ) -> Result<Self, &'static str> {

        let name = name.trim().to_string();
        if !(1..=70).contains(&name.len()) {
            return Err("Name must be 1–70 chars".into());
        }

        let street = street
            .map(|s| s.trim())
            .filter(|s|!s.is_empty())
            .map(|s| s.to_string());

        if let Some(ref s) = street {
            if s.len() > 70 {
                return Err("Street max 70 characters");
            }
        }

        let house_num = house_num
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string());

        if let Some(ref h) = house_num {
            if h.len() > 16 {
                return Err("House number max 16 characters");
            }
        }

        let plz = plz.trim().to_string();
        if plz.is_empty() || plz.len() > 16 {
            return Err("Invalid postal code".into());
        }

        let city = city.trim().to_string();
        if city.is_empty() || city.len() > 35 {
            return Err("Invalid city".into());
        }

        let country = country.trim().to_uppercase();
        if country.len() != 2 {
            return Err("Country must be ISO-2".into());
        }

        Ok(Self {
            name,
            street,
            house_num,
            plz,
            city,
            country,
        })
    }
}
