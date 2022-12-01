use async_graphql::Object;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TranslatedString {
    /// English description
    #[serde(default)]
    en: String,
    de: Option<String>,
    es: Option<String>,
    fr: Option<String>,
    it: Option<String>,
    ja: Option<String>,
    ko: Option<String>,
    pl: Option<String>,
    pt: Option<String>,
    ru: Option<String>,
    zh: Option<String>,
    tr: Option<String>,
}

/// TranslatedString will always have an English description, but may have other languages.
/// It will always return the English description if the language is not found.
#[Object]
impl TranslatedString {
    async fn en(&self) -> &str {
        &self.en
    }

    async fn de(&self) -> String {
        self.de.as_ref().unwrap_or(&self.en).to_string()
    }

    async fn es(&self) -> String {
        self.es.as_ref().unwrap_or(&self.en).to_string()
    }

    async fn fr(&self) -> String {
        self.fr.as_ref().unwrap_or(&self.en).to_string()
    }

    async fn it(&self) -> String {
        self.it.as_ref().unwrap_or(&self.en).to_string()
    }

    async fn ja(&self) -> String {
        self.ja.as_ref().unwrap_or(&self.en).to_string()
    }

    async fn ko(&self) -> String {
        self.ko.as_ref().unwrap_or(&self.en).to_string()
    }

    async fn ru(&self) -> String {
        self.ru.as_ref().unwrap_or(&self.en).to_string()
    }

    async fn zh(&self) -> String {
        self.zh.as_ref().unwrap_or(&self.en).to_string()
    }

    async fn pt(&self) -> String {
        self.pt.as_ref().unwrap_or(&self.en).to_string()
    }

    async fn pl(&self) -> String {
        self.pl.as_ref().unwrap_or(&self.en).to_string()
    }

    async fn tr(&self) -> String {
        self.tr.as_ref().unwrap_or(&self.en).to_string()
    }
}
