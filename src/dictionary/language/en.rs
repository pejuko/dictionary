use regex::Regex;

use crate::dictionary::{language::{IrregularVerbType, LanguageProcessor}, Dictionary, WordClass};


pub struct English {
    word_regex: WordRegex,
    irregular_verbs: IrregularVerbType,
}

impl English {
    pub fn new() -> English {
        let mut verbs = IrregularVerbType::new();

        verbs.insert("arise".to_string(), vec!["arose".to_string(), "arisen".to_string()]);
        verbs.insert("be".to_string(), vec!["was".to_string(), "were".to_string(), "been".to_string(), "am".to_string(), "are".to_string(), "is".to_string()]);
        verbs.insert("bear".to_string(), vec!["bore".to_string(), "borne".to_string()]);

        English {
            word_regex: WordRegex::new(),
            irregular_verbs: verbs,
        }
    }

    fn pluralize(&self, inflections: &mut Vec<String>, headword: &str) {
        self.add_s(inflections, headword);
    }

    fn inflect_verb(&self, inflections: &mut Vec<String>, headword: &str) {
        self.add_s(inflections, headword);
        self.add_ing(inflections, headword);
        self.add_ed(inflections, headword);
    }

    fn add_s(&self, inflections: &mut Vec<String>, headword: &str) {
        let mut new_word = headword.to_string();

        if self.word_regex.re_s.is_match(headword) {
            new_word.push_str("es");
        } else if self.word_regex.re_o.is_match(headword) {
            match headword {
                "hero"
                | "potato"
                | "tomato"
                | "go"
                | "do" => new_word.push_str("es"),
                _ => new_word.push('s'),
            }
        } else if self.word_regex.re_y.is_match(headword) {
            if let Some(captures) = self.word_regex.re_y_with_consonant.captures(headword) {
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

    fn add_ing(&self, inflections: &mut Vec<String>, headword: &str) {
        let mut new_word = headword.to_string();

        if headword.ends_with("ee") {
            new_word.push_str("ing");
        } else if let Some(captures) = self.word_regex.re_e.captures(headword) {
            new_word = format!("{}ing", captures.get(1).unwrap().as_str());
        } else if let Some(captures) = self.word_regex.re_ie.captures(headword) {
            new_word = format!("{}ying", captures.get(1).unwrap().as_str());
        } else if let Some(captures) = self.word_regex.re_verb_ends_with_vowel_and_consonant.captures(headword) {
            let ending = captures.get(1).unwrap().as_str();
            new_word.push_str(ending);
            new_word.push_str("ing");
        } else {
            new_word.push_str("ing");
        }

        inflections.push(new_word);
    }

    fn add_ed(&self, inflections: &mut Vec<String>, headword: &str) {
        let mut new_word = headword.to_string();

        if self.irregular_verbs.contains_key(&Dictionary::word_to_key(headword)) {
            let forms = self.irregular_verbs.get(&Dictionary::word_to_key(headword)).unwrap();
            for form in forms {
                inflections.push(form.clone());
            }
            return;
        }

        if headword.ends_with("e") {
            new_word.push('d');
        } else if let Some(captures) = self.word_regex.re_y_with_consonant.captures(headword) {
            new_word = captures.get(1).unwrap().as_str().to_string();
            new_word.push_str("ied");
        } else if let Some(captures) = self.word_regex.re_verb_ends_with_vowel_and_consonant.captures(headword) {
            let ending = captures.get(1).unwrap().as_str();
            new_word.push_str(ending);
            new_word.push_str("ed");
        } else {
            new_word.push_str("ed");
        }

        inflections.push(new_word);
    }

}

impl LanguageProcessor for English {
    fn inflect(&self, headword: &str, word_class: &WordClass) -> Vec<String> {
        let mut inflections = vec![];

        match word_class {
            WordClass::Noun => self.pluralize(&mut inflections, headword),
            WordClass::Verb => self.inflect_verb(&mut inflections, headword),
            _ => (),
        }

        inflections
    }
}


#[derive(Debug)]
pub struct WordRegex {
    re_e: Regex,
    re_ie: Regex,
    re_s: Regex,
    re_o: Regex,
    re_y: Regex,
    re_y_with_consonant: Regex,
    re_verb_ends_with_vowel_and_consonant: Regex,
}

impl WordRegex {
    pub fn new() -> WordRegex {
        WordRegex {
            re_e: Regex::new("(.*)e$").unwrap(),
            re_ie: Regex::new("(.*)ie$").unwrap(),
            re_s: Regex::new("(s|sh|ch|x)$").unwrap(),
            re_o: Regex::new("o$").unwrap(),
            re_y: Regex::new("y$").unwrap(),
            re_y_with_consonant: Regex::new("(.*[bcdfghjklmnpqrstvwxyz])y$").unwrap(),
            re_verb_ends_with_vowel_and_consonant: Regex::new(".*[aeiou]([bcdfghjklmnpqrstvwxyz])$").unwrap(),
        }
    }
}

impl Default for WordRegex {
    fn default() -> Self {
        Self::new()
    }
}

