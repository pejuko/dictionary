use std::collections::HashMap;

use crate::dictionary::WordClass;

pub mod en;

pub type IrregularVerbType = HashMap<String, Vec<String>>;

pub trait LanguageProcessor {
    fn inflect(&self, headword: &str, word_class: &WordClass) -> Vec<String>;
}

pub fn get_language_processor(source_language: &str) -> Option<Box<dyn LanguageProcessor>> {
    match source_language {
        "en" => Some(Box::new(en::English::new())),
        _ => None
    }
}
