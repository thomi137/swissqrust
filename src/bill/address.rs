/*
 * Copyright (c) 2026 Thomas Prosser 
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */

use serde::{Serialize, Deserialize};
use thiserror::Error;
use crate::Country;
use crate::validators::is_valid_iso_3661_1_country;

const ADDRESS_TYPE: &'static str = "S";

#[derive(Error, Debug)]
pub enum AddressError {
    #[error("Invalid name: {0}")]
    AddressNameError(String),
    #[error("Invalid street address: {0}")]
    AddressStreetError(String),
    #[error("Invalid house nr.")]
    AddressHouseError,
    #[error("Invalid postal code")]
    AddressPostalCodeError,
    #[error("Invalid city character length")]
    AddressCityError,
    #[error("invalid country code: {0}")]
    AddressCountryError(String),
}

/// No need for unstructured address
/// as deprecated on Nov 21, 2025
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Address {
    address_type: String,
    name: String,
    street: Option<String>,
    house_num: Option<String>,
    plz: String,
    city: String,
    country: Country,
}

impl Address {
    pub fn new(
        name: &str,
        street: Option<&str>,
        house_num: Option<&str>,
        plz: &str,
        city: &str,
        country: &str,
    ) -> Result<Self, AddressError> {
        // Name
        let name = name.trim().to_string();
        if !(1..=70).contains(&name.len()) {
            return Err(AddressError::AddressNameError(
                "Name must be 1–70 chars".into(),
            ));
        }

        // Street
        let street = street
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string());
        if let Some(ref s) = street {
            if s.len() > 70 {
                return Err(AddressError::AddressStreetError(
                    "Street must not be longer than 70 chars".into(),
                ));
            }
        }

        // House number
        let house_num = house_num
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string());
        if let Some(ref h) = house_num {
            if h.len() > 16 {
                return Err(AddressError::AddressHouseError);
            }
        }

        // Postal code
        let plz = plz.trim().to_string();
        if plz.is_empty() || plz.len() > 16 {
            return Err(AddressError::AddressPostalCodeError);
        }

        // City
        let city = city.trim().to_string();
        if city.is_empty() || city.len() > 35 {
            return Err(AddressError::AddressCityError);
        }

        // Country
        let country = country.trim().to_uppercase();
        if country.len() != 2 {
            return Err(AddressError::AddressCountryError(
                "Country code must be 2 chars".into(),
            ));
        }

        // ISO validation
        let country_enum = is_valid_iso_3661_1_country(&country)
            .map_err(|e| AddressError::AddressCountryError(e.to_string()))?;

        Ok(Self {
            address_type: ADDRESS_TYPE.into(),
            name,
            street,
            house_num,
            plz,
            city,
            country: country_enum,
        })
    }
}
