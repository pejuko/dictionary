use std::collections::HashMap;

use crate::dictionary::WordClass;

pub mod en;

pub type IrregularVerbType = HashMap<String, Vec<String>>;

pub trait LanguageProcessor {
    fn get_language_code(&self) -> String;
    fn inflect(&self, headword: &str, word_class: &WordClass) -> Vec<String>;
}
