mod reader;
mod writer;

use std::collections::{HashMap, HashSet};
use std::error::Error;

use reader::{gnu_fdl, pronunciation, wiki};
use regex::Regex;
use writer::kindle;

use crate::cli_config::CliConfig;

pub struct Dictionary {
    source_language: String,
    target_language: String,
    title: String,
    author: String,

    terms: HashMap<String, Term>,
    re: WordRegex,
    irregular_verbs: IrregularVerbType
}

type IrregularVerbType = HashMap<String, Vec<String>>;

#[derive(Debug)]
pub struct WordRegex {
    en_re_e: Regex,
    en_re_ie: Regex,
    en_re_s: Regex,
    en_re_o: Regex,
    en_re_y: Regex,
    en_re_y_with_consonant: Regex,
    en_re_verb_ends_with_vowel_and_consonant: Regex,
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


#[derive(Debug, Clone)]
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
            re: WordRegex::new(),
            irregular_verbs: Self::generate_irregular_verbs(source_language),
        }
    }

    pub fn generate_irregular_verbs(source_language: &str) -> IrregularVerbType {
        match source_language {
            "en" => Self::en_get_irregular_verbs(),
            _ => IrregularVerbType::new()
        }
    }

    pub fn en_get_irregular_verbs() -> IrregularVerbType {
        let mut verbs = IrregularVerbType::new();

        verbs.insert("arise".to_string(), vec!["arose".to_string(), "arisen".to_string()]);
        verbs.insert("be".to_string(), vec!["was".to_string(), "were".to_string(), "been".to_string(), "am".to_string(), "are".to_string(), "is".to_string()]);
        verbs.insert("bear".to_string(), vec!["bore".to_string(), "borne".to_string()]);

        verbs
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

    pub fn add_meaning(&mut self, headword: &str, word_class: WordClass, meaning: Meaning) {
        let order = self.get_meaning_position(headword, &word_class, &meaning);

        let entry = self.terms
            .entry(Self::word_to_key(headword))
            .or_insert(Term::new(headword));

        let inflections = Self::inflect(&self.source_language, headword, word_class.clone(), &self.re, &self.irregular_verbs);

        entry.inflections.extend(inflections);

        let class_entry = entry.classes
            .entry(word_class)
            .or_default();

        let meaning_entry = class_entry
            .entry(Self::word_to_key(meaning.description.as_str()))
            .or_insert(Meaning::new(meaning.description.as_str()));

        meaning_entry.translations.extend(meaning.translations);
        meaning_entry.order = order;
    }

    fn inflect(source_language: &str, headword: &str, word_class: WordClass, re: &WordRegex, irregular: &IrregularVerbType) -> Vec<String> {
        let mut inflections = vec![];

        match word_class {
            WordClass::Noun => Self::pluralize(&mut inflections, source_language, headword, re),
            WordClass::Verb => Self::inflect_verb(&mut inflections, source_language, headword, re, irregular),
            _ => (),
        }

        inflections
    }

    fn pluralize(inflections: &mut Vec<String>, source_language: &str, headword: &str, re: &WordRegex) {
        match source_language {
            "en" => Self::en_add_s(inflections, headword, re),
            _ => (),
        }
    }

    fn inflect_verb(inflections: &mut Vec<String>, source_language: &str, headword: &str, re: &WordRegex, irregular: &IrregularVerbType) {
        match source_language {
            "en" => {
                Self::en_add_s(inflections, headword, re);
                Self::en_add_ing(inflections, headword, re);
                Self::en_add_ed(inflections, headword, re, irregular);
            },
            _ => (),
        }
    }

    fn en_add_s(inflections: &mut Vec<String>, headword: &str, re: &WordRegex) {
        let mut new_word = headword.to_string();

        if re.en_re_s.is_match(headword) {
            new_word.push_str("es");
        } else if re.en_re_o.is_match(headword) {
            match headword {
                "hero"
                | "potato"
                | "tomato"
                | "go"
                | "do" => new_word.push_str("es"),
                _ => new_word.push('s'),
            }
        } else if re.en_re_y.is_match(headword) {
            if let Some(captures) = re.en_re_y_with_consonant.captures(headword) {
                new_word = captures.get(1).unwrap().as_str().to_string();
                new_word.push_str("ies");
            } else {
                new_word.push('s');
            }
        } else {
            new_word.push('s');
        }

        inflections.push(new_word);
    }

    fn en_add_ing(inflections: &mut Vec<String>, headword: &str, re: &WordRegex) {
        let mut new_word = headword.to_string();

        if headword.ends_with("ee") {
            new_word.push_str("ing");
        } else if let Some(captures) = re.en_re_e.captures(headword) {
            new_word = format!("{}ing", captures.get(1).unwrap().as_str());
        } else if let Some(captures) = re.en_re_ie.captures(headword) {
            new_word = format!("{}ying", captures.get(1).unwrap().as_str());
        } else if let Some(captures) = re.en_re_verb_ends_with_vowel_and_consonant.captures(headword) {
            let ending = captures.get(1).unwrap().as_str();
            new_word.push_str(ending);
            new_word.push_str("ing");
        } else {
            new_word.push_str("ing");
        }

        inflections.push(new_word);
    }

    fn en_add_ed(inflections: &mut Vec<String>, headword: &str, re: &WordRegex, irregular: &IrregularVerbType) {
        let mut new_word = headword.to_string();

        if irregular.contains_key(&Self::word_to_key(headword)) {
            let forms = irregular.get(&Self::word_to_key(headword)).unwrap();
            for form in forms {
                inflections.push(form.clone());
            }
            return;
        }

        if headword.ends_with("e") {
            new_word.push('d');
        } else if let Some(captures) = re.en_re_y_with_consonant.captures(headword) {
            new_word = captures.get(1).unwrap().as_str().to_string();
            new_word.push_str("ied");
        } else if let Some(captures) = re.en_re_verb_ends_with_vowel_and_consonant.captures(headword) {
            let ending = captures.get(1).unwrap().as_str();
            new_word.push_str(ending);
            new_word.push_str("ed");
        } else {
            new_word.push_str("ed");
        }

        inflections.push(new_word);
    }

    pub fn lookup(&self, word: &str) -> Option<&Term> {
        self.terms.get(Self::word_to_key(word).as_str())
    }

    pub fn len(&self) -> usize {
        self.terms.len()
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

impl WordRegex {
    pub fn new() -> WordRegex {
        WordRegex {
            en_re_e: Regex::new("(.*)e$").unwrap(),
            en_re_ie: Regex::new("(.*)ie$").unwrap(),
            en_re_s: Regex::new("(s|sh|ch|x)$").unwrap(),
            en_re_o: Regex::new("o$").unwrap(),
            en_re_y: Regex::new("y$").unwrap(),
            en_re_y_with_consonant: Regex::new("(.*[bcdfghjklmnpqrstvwxyz])y$").unwrap(),
            en_re_verb_ends_with_vowel_and_consonant: Regex::new(".*[aeiou]([bcdfghjklmnpqrstvwxyz])$").unwrap(),
        }
    }
}

impl Default for WordRegex {
    fn default() -> Self {
        Self::new()
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

        if self.pronunciations.is_empty() && self.classes.is_empty() {
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
