use std::error::Error;

use crate::dictionary::{Dictionary, Meaning, WordClass};

pub fn read_czech(dict: &mut Dictionary, path: &str) -> Result<(), Box<dyn Error>> {
    let lines = super::read_tab_file(path)?;

    for line in lines {
        if line.len() < 3 {
            continue;
        }

        let mut meaning = Meaning::new("");
        meaning.add_translation(line[1].trim());

        let word_class = match line[2].as_str() {
            "n:" => WordClass::Noun,
            "v:" => WordClass::Verb,
            "adv:" => WordClass::Adverb,
            "adj:" => WordClass::Adjective,
            "pron:" => WordClass::Pronoun,
            "prep:" => WordClass::Preposition,
            _ => WordClass::Unknown
        };

        dict.add_meaning(line[0].trim(), word_class, meaning);
    }

    Ok(())
}
