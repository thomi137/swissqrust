use serde::{Serialize, Deserialize};

/// No need for line1 or 2 since
/// Deprecated on Nov 21, 2025
#[derive(Debug, Serialize, Deserialize)]
pub struct Address {
    pub name: String,
    pub street: String,
    pub house_num: String,
    pub plz: String,
    pub city: String,
    pub country: String,
}

impl Address {
    pub fn new(
        name: &str,
        street: &str,
        house_num: &str,
        plz: &str,
        city: &str,
        country: &str,
    ) -> Result<Self, String> {

        let name = name.trim().to_string();
        if !(1..=70).contains(&name.len()) {
            return Err("Name must be 1–70 chars".into());
        }

        let street = street.trim().to_string();
        if street.len() > 70 {
            return Err("Street max 70 chars".into());
        }

        let house_num = house_num.trim().to_string();
        if house_num.len() > 16 {
            return Err("House number max 16 chars".into());
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
