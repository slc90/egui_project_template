use std::{collections::HashMap, fs};

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

use crate::{
    config::config::CONFIG,
    utils::constants::{CHINESE_TRANSLATION, ENGLISH_TRANSLATION},
};

#[derive(Debug, Deserialize, Serialize)]
pub struct LanguageConfig {
    pub language: Language,
}

#[derive(Debug, Deserialize, Serialize, Hash, PartialEq, Eq)]
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

/// 程序中使用的UI显示的字符串的Key,用来找对应的翻译
#[derive(Deserialize, Hash, PartialEq, Eq, Display, EnumString)]
pub enum LanguageKey {
    HomePage,
    Settings,
    About,
    Language,
}

// 全局翻译存储：语言代码 -> (键 -> 翻译)
static TRANSLATIONS: Lazy<HashMap<Language, HashMap<LanguageKey, String>>> = Lazy::new(|| {
    let mut map = HashMap::new();
    // 读取中文翻译
    let zh_str = fs::read_to_string(CHINESE_TRANSLATION).unwrap();
    let zh: HashMap<LanguageKey, String> = toml::from_str(&zh_str).unwrap();
    map.insert(Language::Chinese, zh);
    // 读取英文翻译
    let en_str = fs::read_to_string(ENGLISH_TRANSLATION).unwrap();
    let en: HashMap<LanguageKey, String> = toml::from_str(&en_str).unwrap();
    map.insert(Language::English, en);
    map
});

// 提供一个翻译函数
pub fn translations(key: &LanguageKey) -> String {
    let config = CONFIG.lock().unwrap();
    TRANSLATIONS
        .get(&config.language_config.language)
        .and_then(|m| m.get(key))
        .cloned()
        // 没找到就返回 key 本身
        .unwrap_or_else(|| key.to_string())
}
