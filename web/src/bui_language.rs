/*
 * Copyright (c) 2026 Thomas Prosser
 * Licensed under MIT License
 * https://opensource.org/licenses/MIT
 */
use std::cmp::PartialEq;

use swiss_qrust::{LabelKey, Language, LanguageError};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum GuiLabelKey {
    Name,
    Street,
    HouseNo,
    City,
    PostalCode,
    Country,
}

const GUI_LABELS: &[(GuiLabelKey, Language, &str)] = &[

    // Address Labels
    (GuiLabelKey::Name, Language::De, "Name"),
    (GuiLabelKey::Name, Language::Fr, "Nom"),
    (GuiLabelKey::Name, Language::It, "Nome"),
    (GuiLabelKey::Name, Language::En, "Name"),

    (GuiLabelKey::Street, Language::De, "Strasse"),
    (GuiLabelKey::Street, Language::Fr, "Rue"),
    (GuiLabelKey::Street, Language::It, "Via"),
    (GuiLabelKey::Street, Language::En, "Street"),

    (GuiLabelKey::HouseNo, Language::De, "Nr"),
    (GuiLabelKey::HouseNo, Language::Fr, "N°"),
    (GuiLabelKey::HouseNo, Language::It, "N."),
    (GuiLabelKey::HouseNo, Language::En, "No"),

    (GuiLabelKey::City, Language::De, "Ort"),
    (GuiLabelKey::City, Language::Fr, "Localité"),
    (GuiLabelKey::City, Language::It, "Luogo"),
    (GuiLabelKey::City, Language::En, "Town"),

    (GuiLabelKey::PostalCode, Language::De, "PLZ"),
    (GuiLabelKey::PostalCode, Language::Fr, "NPA"),
    (GuiLabelKey::PostalCode, Language::It, "CAP"),
    (GuiLabelKey::PostalCode, Language::En, "Postal Code"),

    (GuiLabelKey::Country, Language::De, "Land"),
    (GuiLabelKey::Country, Language::Fr, "Pays"),
    (GuiLabelKey::Country, Language::It, "Paese"),
    (GuiLabelKey::Country, Language::En, "Country")

];

pub enum Translatable {
    Lib(LabelKey),
    Gui(GuiLabelKey),
}

pub fn get_gui_label(key: GuiLabelKey, lang: Language) -> &'static str {
    GUI_LABELS
        .iter()
        .find(|&&(k, l, _)| k == key && l == lang)
        .map(|&(_, _, val)| val)
        // Fallback logic
        .or_else(|| GUI_LABELS.iter().find(|&&(k, l, _)| k == key && l == Language::En).map(|&(_, _, v)| v))
        .unwrap_or("Label not found")
}