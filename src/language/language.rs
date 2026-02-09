use thiserror::Error;

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Language {
    De,
    Fr,
    It,
    #[default]
    En,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum LabelKey {
    PaymentPart,
    AccountPayableTo,
    Reference,
    AdditionalInformation,
    Currency,
    Amount,
    Receipt,
    AcceptancePoint,
    SeparateBeforePayingIn,
    PayableBy,
    PayableByNameAddress,
    InFavourOf,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Error)]
pub enum LanguageError {
    #[error("No Label Found for key: {0:?}")]
    LabelNotFound(LabelKey),
}

pub struct LabelEntry {
    pub key: LabelKey,
    pub lang: Language,
    pub text: &'static str,
}

const LABELS: &[(LabelKey, Language, &str)] = &[
    (LabelKey::PaymentPart, Language::De, "Zahlteil"),
    (LabelKey::PaymentPart, Language::Fr, "Section paiement"),
    (LabelKey::PaymentPart, Language::It, "Sezione pagamento"),
    (LabelKey::PaymentPart, Language::En, "Payment part"),

    (LabelKey::AccountPayableTo, Language::De, "Konto / Zahlbar an"),
    (LabelKey::AccountPayableTo, Language::Fr, "Compte / Payable à"),
    (LabelKey::AccountPayableTo, Language::It, "Conto / Pagabile a"),
    (LabelKey::AccountPayableTo, Language::En, "Account / Payable to"),

    (LabelKey::Reference, Language::De, "Referenz"),
    (LabelKey::Reference, Language::Fr, "Référence"),
    (LabelKey::Reference, Language::It, "Riferimento"),
    (LabelKey::Reference, Language::En, "Reference"),

    (LabelKey::AdditionalInformation, Language::De, "Zusätzliche Informationen"),
    (LabelKey::AdditionalInformation, Language::Fr, "Informations supplémentaires"),
    (LabelKey::AdditionalInformation, Language::It, "Informazioni supplementari"),
    (LabelKey::AdditionalInformation, Language::En, "Additional information"),

    (LabelKey::Currency, Language::De, "Währung"),
    (LabelKey::Currency, Language::Fr, "Monnaie"),
    (LabelKey::Currency, Language::It, "Valuta"),
    (LabelKey::Currency, Language::En, "Currency"),

    (LabelKey::Amount, Language::De, "Betrag"),
    (LabelKey::Amount, Language::Fr, "Montant"),
    (LabelKey::Amount, Language::It, "Importo"),
    (LabelKey::Amount, Language::En, "Amount"),

    (LabelKey::Receipt, Language::De, "Empfangsschein"),
    (LabelKey::Receipt, Language::Fr, "Récépissé"),
    (LabelKey::Receipt, Language::It, "Ricevuta"),
    (LabelKey::Receipt, Language::En, "Receipt"),

    (LabelKey::AcceptancePoint, Language::De, "Annahmestelle"),
    (LabelKey::AcceptancePoint, Language::Fr, "Point de dépôt"),
    (LabelKey::AcceptancePoint, Language::It, "Punto di accettazione"),
    (LabelKey::AcceptancePoint, Language::En, "Acceptance point"),

    (LabelKey::SeparateBeforePayingIn, Language::De, "Vor der Einzahlung abzutrennen"),
    (LabelKey::SeparateBeforePayingIn, Language::Fr, "A détacher avant le versement"),
    (LabelKey::SeparateBeforePayingIn, Language::It, "Da staccare prima del versamento"),
    (LabelKey::SeparateBeforePayingIn, Language::En, "Separate before paying in"),

    (LabelKey::PayableBy, Language::De, "Zahlbar durch"),
    (LabelKey::PayableBy, Language::Fr, "Payable par"),
    (LabelKey::PayableBy, Language::It, "Pagabile da"),
    (LabelKey::PayableBy, Language::En, "Payable by"),

    (LabelKey::PayableByNameAddress, Language::De, "Zahlbar durch (Name/Adresse)"),
    (LabelKey::PayableByNameAddress, Language::Fr, "Payable par (nom/adresse)"),
    (LabelKey::PayableByNameAddress, Language::It, "Pagabile da (nome/indirizzo)"),
    (LabelKey::PayableByNameAddress, Language::En, "Payable by (name/address)"),

    (LabelKey::InFavourOf, Language::De, "Zugunsten"),
    (LabelKey::InFavourOf, Language::Fr, "En faveur de"),
    (LabelKey::InFavourOf, Language::It, "A favore di"),
    (LabelKey::InFavourOf, Language::En, "In favour of"),

];

pub fn label(key: LabelKey, lang: Language) -> Result<&'static str, LanguageError> {
    // Try requested language
    LABELS
        .iter()
        .find(|&&(k, l, _)| k == key && l == lang)
        .map(|&(_, _, val)| val)
        // Fallback to English
        .or_else(|| {
            LABELS
                .iter()
                .find(|&&(k, l, _)| k == key && l == Language::default())
                .map(|&(_, _, val)| val)
        })
        // If still not found, return an error
        .ok_or(LanguageError::LabelNotFound(key))
}

#[macro_export]
macro_rules! label {
    ($key:ident) => {
        $crate::language::label($crate::language::LabelKey::$key, $crate::language::Language::default())
    };
    ($key:ident, $lang:expr) => {
        $crate::language::label($crate::language::LabelKey::$key, $lang)
    };
}
