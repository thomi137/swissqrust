/*
 * Copyright (c) 2026 Thomas Prosser
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */

use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(from = "CountryHelper")]
struct CountryRaw {
    cca2: String,
    cca3: String,
    ccn3: Option<String>,
    name: String,
    name_de: String,
    name_fr: String,
    name_it: String,
    flag: Option<String>,
    status: Option<String>,
}

#[derive(Deserialize)]
struct CountryHelper {
    cca2: String,
    cca3: String,
    ccn3: Option<String>,
    name: Name,
    translations: Map<String, Translation>,
    flag: Option<String>,
    status: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Name {
    common: String,
}

#[derive(Deserialize)]
struct Translation { common: String }

type Map<K, V> = std::collections::HashMap<K, V>;

impl From<CountryHelper> for CountryRaw {
    fn from(helper: CountryHelper) -> Self {
        Self {
            cca2: helper.cca2,
            cca3: helper.cca3,
            ccn3: helper.ccn3,
            name: helper.name.common,
            // Access nested fields safely
            name_de: helper.translations.get("deu").map(|t| t.common.clone()).unwrap_or_default(),
            name_fr: helper.translations.get("fra").map(|t| t.common.clone()).unwrap_or_default(),
            name_it: helper.translations.get("ita").map(|t| t.common.clone()).unwrap_or_default(),
            flag: helper.flag,
            status: helper.status,
        }
    }
}

pub fn generate() {

    let dest_path = Path::new("src/generated/countries.rs");

    // ensure parent directory exists
    if let Some(parent) = dest_path.parent() {
        fs::create_dir_all(parent).expect("failed to create generated directory");
    }

    // rerun if build_functions script changes
    println!("cargo:rerun-if-changed=build_functions.rs");
    // tell cargo to rerun if file has changed.
    println!("cargo:rerun-if-changed=assets/data/countries.json");

    let json = fs::read_to_string("assets/data/countries.json")
        .expect("failed to read countries.json");

    let countries: Vec<CountryRaw> =
        serde_json::from_str(&json).expect("invalid countries.json");

    // output path inside src/generated for IDE visibility
    let dest_path = Path::new("src/generated/countries.rs");

    let mut enum_variants = String::new();
    let mut meta_arms = String::new();
    let mut from_str_arms = String::new();

    for c in countries {
        let alpha2 = c.cca2;
        let variant = &alpha2;

        let flag_str = c.flag
            .as_ref()
            .map(|f| format!("Some({:?})", f))
            .unwrap_or("None".into());


        enum_variants.push_str(&format!("    {},\n", variant));
        meta_arms.push_str(&format!(
            "            Country::{variant} => CountryMeta {{\n\
             \t\t\talpha2: \"{a2}\",\n\
             \t\t\talpha3: \"{a3}\",\n\
             \t\t\tnumeric: {num},\n\
             \t\t\tname: \"{name}\",\n\
             \t\t\tname_de: \"{name_de}\",\n\
             \t\t\tname_fr: \"{name_fr}\",\n\
             \t\t\tname_it: \"{name_it}\",\n\
             \t\t\tflag: {flag},\n\
             \t\t\tstatus: {status},\n\
             \t\t}},\n",
            a2 = alpha2,
            a3 = c.cca3,
            num = c
                .ccn3
                .as_ref()
                .map(|n| format!("Some(\"{n}\")"))
                .unwrap_or("None".into()),
            name = c.name,
            name_de = c.name_de,
            name_fr = c.name_fr,
            name_it = c.name_it,
            flag = flag_str,
            status = c
                .status
                .as_ref()
                .map(|s| format!("Some(\"{s}\")"))
                .unwrap_or("None".into()),
        ));

        from_str_arms.push_str(&format!(
            "            \"{a2}\" => std::result::Result::Ok(Country::{variant}),\n",
            a2 = alpha2
        ));

    }

    let generated = format!(
        r#"// AUTO-GENERATED FROM mledoze/countries (ODbL 1.0)
// https://github.com/mledoze/countries
// DO NOT EDIT BY HAND

use strum::{{
    IntoEnumIterator,
    Display,
    EnumIter,
}};

#[derive(Debug, thiserror::Error)]
pub enum CountryParseError {{
    #[error("'{{0}}' is not a valid ISO country code")]
    InvalidCode(String),
}}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, EnumIter, Display)]
pub enum Country {{
{enum_variants}
}}

#[derive(Debug)]
pub struct CountryMeta {{
    pub alpha2: &'static str,
    pub alpha3: &'static str,
    pub numeric: Option<&'static str>,
    pub name: &'static str,
    pub name_de: &'static str,
    pub name_fr: &'static str,
    pub name_it: &'static str,
    pub flag: Option<&'static str>,
    pub status: Option<&'static str>,
}}

impl Country {{
    pub fn meta(self) -> CountryMeta {{
        match self {{
{meta_arms}
        }}
    }}
}}

impl std::str::FromStr for Country {{
    type Err = CountryParseError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {{
        match s.to_ascii_uppercase().as_str() {{
{from_str_arms}
            _ => ::core::result::Result::Err(CountryParseError::InvalidCode(s.to_string())),
        }}
    }}
}}
"#
    );

    let mut file = File::create(dest_path).expect("failed to create countries.rs");
    file.write_all(generated.as_bytes())
        .expect("failed to write countries.rs");
}
