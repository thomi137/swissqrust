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
struct CountryRaw {
    cca2: String,
    cca3: String,
    ccn3: Option<String>,
    name: Name,
    flag: Option<String>,
    status: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Name {
    common: String,
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
            name = c.name.common,
            flag = flag_str,
            status = c
                .status
                .as_ref()
                .map(|s| format!("Some(\"{s}\")"))
                .unwrap_or("None".into()),
        ));

        from_str_arms.push_str(&format!(
            "            \"{a2}\" => Ok(Country::{variant}),\n",
            a2 = alpha2
        ));

    }

    let generated = format!(
        r#"// AUTO-GENERATED FROM mledoze/countries (ODbL 1.0)
// https://github.com/mledoze/countries
// DO NOT EDIT BY HAND

use std::str::FromStr;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Country {{
{enum_variants}
}}

#[derive(Debug)]
pub struct CountryMeta {{
    pub alpha2: &'static str,
    pub alpha3: &'static str,
    pub numeric: Option<&'static str>,
    pub name: &'static str,
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
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {{
        match s.to_ascii_uppercase().as_str() {{
{from_str_arms}
            _ => Err(()),
        }}
    }}
}}
"#
    );

    let mut file = File::create(dest_path).expect("failed to create countries.rs");
    file.write_all(generated.as_bytes())
        .expect("failed to write countries.rs");
}