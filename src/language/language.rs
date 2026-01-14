
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Language {
    De,
    Fr,
    It,
}

pub fn label(key: &str, lang: Language) -> &str {
    match (key, lang) {
        ("Payment part", Language::De) => "Zahlteil",
        ("Payment part", Language::Fr) => "Section paiement",
        ("Payment part", Language::It) => "Sezione pagamento",

        ("Account / Payable to", Language::De) => "Konto / Zahlbar an",
        ("Account / Payable to", Language::Fr) => "Compte / Payable à",
        ("Account / Payable to", Language::It) => "Conto / Pagabile a",

        ("Reference", Language::De) => "Referenz",
        ("Reference", Language::Fr) => "Référence",
        ("Reference", Language::It) => "Riferimento",

        ("Additional information", Language::De) => "Zusätzliche Informationen",
        ("Additional information", Language::Fr) => "Informations supplémentaires",
        ("Additional information", Language::It) => "Informazioni supplementari",

        ("Currency", Language::De) => "Währung",
        ("Currency", Language::Fr) => "Monnaie",
        ("Currency", Language::It) => "Valuta",

        ("Amount", Language::De) => "Betrag",
        ("Amount", Language::Fr) => "Montant",
        ("Amount", Language::It) => "Importo",

        ("Receipt", Language::De) => "Empfangsschein",
        ("Receipt", Language::Fr) => "Récépissé",
        ("Receipt", Language::It) => "Ricevuta",

        ("Acceptance point", Language::De) => "Annahmestelle",
        ("Acceptance point", Language::Fr) => "Point de dépôt",
        ("Acceptance point", Language::It) => "Punto di accettazione",

        ("Separate before paying in", Language::De) => "Vor der Einzahlung abzutrennen",
        ("Separate before paying in", Language::Fr) => "A détacher avant le versement",
        ("Separate before paying in", Language::It) => "Da staccare prima del versamento",

        ("Payable by", Language::De) => "Zahlbar durch",
        ("Payable by", Language::Fr) => "Payable par",
        ("Payable by", Language::It) => "Pagabile da",

        ("Payable by (name/address)", Language::De) => "Zahlbar durch (Name/Adresse)",
        ("Payable by (name/address)", Language::Fr) => "Payable par (nom/adresse)",
        ("Payable by (name/address)", Language::It) => "Pagabile da (nome/indirizzo)",

        ("In favour of", Language::De) => "Zugunsten",
        ("In favour of", Language::Fr) => "En faveur de",
        ("In favour of", Language::It) => "A favore di",

        // fallback: return the English key itself
        _ => key,
    }
}

