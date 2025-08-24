use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct LanguageConfig {
    pub language: Language,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Language {
    Chinese,
    English,
}

impl Default for LanguageConfig {
    fn default() -> Self {
        LanguageConfig {
            language: Language::Chinese,
        }
    }
}
