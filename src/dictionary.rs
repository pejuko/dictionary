mod language;
mod reader;
mod writer;

use std::collections::{HashMap, HashSet};
use std::error::Error;

use reader::{gnu_fdl, pronunciation, wiki};
use writer::kindle;

use crate::cli_config::CliConfig;
use crate::dictionary::language::LanguageProcessor;

pub struct Dictionary {
    source_language: String,
    target_language: String,
    title: String,
    author: String,

    terms: HashMap<String, Term>,
    language_processor: Option<Box<dyn LanguageProcessor>>,
}


#[derive(Debug)]
pub struct Term {
    headword: String,
    inflections: HashSet<String>,
    pronunciations: HashMap<String, PronunciationType>,
    classes: HashMap<WordClass, MeaningType>,
}

type PronunciationType = Vec<String>;
type MeaningType = HashMap<String, Meaning>;

#[derive(Eq, Hash, PartialEq, PartialOrd, Ord, Debug, Clone)]
pub enum WordClass {
    Verb,
    Noun,
    Adjective,
    Adverb,
    Preposition,
    Determiner,
    Pronoun,
    LinkingWord,
    Unknown,
}


#[derive(Debug)]
pub struct Meaning {
    order: usize,
    description: String,
    translations: TranslationType,
}

type TranslationType = HashSet<String>;

impl Dictionary {
    pub fn new(
        source_language: &str, target_language: &str, title: &str, author: &str,
    ) -> Dictionary {
        Dictionary {
            source_language: source_language.to_string(),
            target_language: target_language.to_string(),
            title: title.to_string(),
            author: author.to_string(),
            terms: HashMap::new(),
            language_processor: language::get_language_processor(source_language),
        }
    }

    pub fn build(cfg: &CliConfig) -> Result<Dictionary, Box<dyn Error>> {
        let mut dict = Dictionary::new(
            cfg.source_language.as_str(),
            cfg.target_language.as_str(),
            cfg.title.as_str(),
            cfg.author.as_str(),
        );

        if let Some(path) = &cfg.input_file_path {
            match format!("{}-{}", dict.source_language, dict.target_language).as_str() {
                "en-cs" => gnu_fdl::read_czech(&mut dict, path)?,
                lng => Err(format!("Unsupported language combination: {}", lng))?,
            }
        }

        for (name, file_name) in &cfg.pronunciation_files {
            pronunciation::read_pronunciation(&mut dict, name, file_name)?;
        }

        if let Some(wiki_file_path) = &cfg.wiki_file_path {
            if let Some(wiki_prefix) = &cfg.wiki_prefix {
                wiki::read_wiki(&mut dict, wiki_file_path, wiki_prefix)?;
            } else {
                Err(String::from("No wiki prefix specified."))?;
            }
        }

        Ok(dict)
    }

    pub fn reverse(&self, reversed_title: &str) -> Dictionary {
        let mut dict = Dictionary::new(
            self.target_language.as_str(),
            self.source_language.as_str(),
            reversed_title,
            self.author.as_str(),
        );

        for (_headword_key, term) in self.terms.iter() {
            for (word_class, meanings) in term.classes.iter() {
                for (_, meaning) in meanings.iter() {
                    for translation in meaning.translations.iter() {
                        let mut m = Meaning::new(meaning.description.as_str());
                        m.add_translation(&term.headword);
                        dict.add_meaning(translation, word_class, &m);
                    }
                }
            }
        }

        dict
    }

    pub fn add_pronunciation(&mut self, headword: &str, name: &str, pronunciation: &str) {
        let entry = self.terms.entry(Self::word_to_key(headword)).or_insert(Term::new(headword));
        let pron_entry = entry.pronunciations.entry(name.to_string()).or_default();
        pron_entry.push(pronunciation.to_string());
    }

    pub fn get_meaning_position(&self, headword: &str, word_class: &WordClass, meaning: &Meaning) -> usize {
        let key = Self::word_to_key(headword);
        if let Some(term) = self.terms.get(&key) {
            if let Some(class) = term.classes.get(word_class) {
                let key = Self::word_to_key(&meaning.description);
                if let Some(m) = class.get(&key) {
                    return m.order;
                } else {
                    return class.len();
                }
            }
        }

        0
    }

    pub fn add_meaning(&mut self, headword: &str, word_class: &WordClass, meaning: &Meaning) {
        let order = self.get_meaning_position(headword, word_class, meaning);

        let entry = self.terms
            .entry(Self::word_to_key(headword))
            .or_insert(Term::new(headword));

        if let Some(language_processor) = &self.language_processor {
            let inflections = language_processor.inflect(headword, word_class);
            entry.inflections.extend(inflections);
        }

        let class_entry = entry.classes
            .entry(word_class.clone())
            .or_default();

        let meaning_entry = class_entry
            .entry(Self::word_to_key(meaning.description.as_str()))
            .or_insert(Meaning::new(meaning.description.as_str()));

        meaning_entry.translations.extend(meaning.translations.clone());
        meaning_entry.order = order;
    }


    pub fn lookup(&self, word: &str) -> Option<&Term> {
        self.terms.get(Self::word_to_key(word).as_str())
    }

    pub fn len(&self) -> usize {
        self.terms.len()
    }
    
    pub fn non_empty_len(&self) -> usize {
        self.terms.iter().filter(|(_, term)| !term.is_empty()).count()
    }
 
    pub fn translations_len(&self) -> usize {
        self.terms.iter().filter(|(_, term)| 
            term.classes.iter().filter(|(_, meaning)| 
                meaning.iter().filter(|(_, m)| 
                    !m.translations.is_empty()
                ).count() > 0
            ).count() > 0
        ).count()
    }

    /*
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    */

    pub fn to_kindle(&self, output_path: &str, force: bool) -> Result<(), Box<dyn Error>> {
        kindle::to_kindle(self, output_path, force)
    }

    pub fn word_to_key(word: &str) -> String {
        word.to_lowercase()
    }
}

impl Term {
    pub fn new(headword: &str) -> Term {
        Term {
            headword: headword.to_string(),
            inflections: HashSet::new(),
            pronunciations: HashMap::new(),
            classes: HashMap::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        if self.headword.is_empty() {
            return true;
        }

        if /* self.pronunciations.is_empty() && */ self.classes.is_empty() {
                return true;
        }

        false
    }
}

impl WordClass {
    pub fn as_str(&self) -> &'static str {
        match self {
            WordClass::Noun => "noun",
            WordClass::Verb => "verb",
            WordClass::Adjective => "adjective",
            WordClass::Adverb => "adverb",
            WordClass::Determiner => "determiner",
            WordClass::LinkingWord => "linking",
            WordClass::Preposition => "preposition",
            WordClass::Pronoun => "pronoun",
            WordClass::Unknown => "other",
        }
    }
}

impl Meaning {
    pub fn new(description: &str) -> Meaning {
        Meaning {
            order: 0,
            description: description.to_string(),
            translations: TranslationType::new(),
        }
    }

    pub fn add_translation(&mut self, translation: &str) {
        self.translations.insert(translation.to_string());
    }

    pub fn is_empty(&self) -> bool {
        self.description.is_empty() && self.translations.is_empty()
    }
}
