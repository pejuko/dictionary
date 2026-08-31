use crate::dictionary::{Dictionary, Meaning, WordClass};
use crate::error::Result;

pub fn read_czech(dict: &mut Dictionary, path: &str) -> Result<()> {
    let lines = super::read_tab_file(path)?;

    for line in lines {
        if line.len() < 3 {
            continue;
        }

        let headword = line[0].trim();
        let translation = line[1].trim();

        if headword.is_empty() || translation.is_empty() {
            continue;
        }

        let mut meaning = Meaning::new("");
        meaning.add_translation(translation);

        let word_class = match line[2].trim() {
            "n:" => WordClass::Noun,
            "v:" => WordClass::Verb,
            "adv:" => WordClass::Adverb,
            "adj:" => WordClass::Adjective,
            "pron:" => WordClass::Pronoun,
            "prep:" => WordClass::Preposition,
            _ => WordClass::Unknown
        };

        dict.add_meaning(headword, &word_class, &meaning);
    }

    Ok(())
}
