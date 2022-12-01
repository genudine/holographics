use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};

#[derive(SimpleObject, Serialize, Deserialize, Debug, Clone)]
pub struct TranslatedString {
    /// English description
    #[serde(default)]
    en: String,

    /// German description
    #[serde(default)]
    de: String,

    /// Spanish description
    #[serde(default)]
    es: String,

    /// French description
    #[serde(default)]
    fr: String,

    /// Italian description
    #[serde(default)]
    it: String,

    /// Turkish description
    #[serde(default)]
    tr: String,
}


