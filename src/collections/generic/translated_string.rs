use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TranslatedString {
    /// English description
    #[serde(default)]
    en: String,

    #[serde(flatten)]
    extra: HashMap<String, String>,
}

impl TranslatedString {
    pub fn lang(&self, lang: String) -> String {
        if lang == "en" {
            self.en.clone()
        } else {
            self.extra.get(&lang).unwrap_or(&self.en).clone()
        }
    }
}
