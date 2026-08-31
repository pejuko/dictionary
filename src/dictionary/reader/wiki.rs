extern crate bzip2;

use quick_xml::events::Event;
use quick_xml::reader::Reader;
use regex::Regex;

use crate::dictionary::{Dictionary, Meaning, WordClass};
use crate::error::{Error, Result};

#[derive(Debug)]
struct Page {
    title: String,
    content: String,
}

impl Page {
    fn empty() -> Page {
        Page {
            title: "".to_string(),
            content: "".to_string(),
        }
    }
}

enum State {
    None, Page, Title, Content,
}

struct Re {
    data: Regex,
    translations_title: Regex,
    language: Regex,
    translation_name: Regex,
    prefix: Regex,
}

impl Re {
    fn new(lang_prefix: &str) -> Result<Re> {
        let data = Regex::new(r"\{\{(.*?)}}")
            .map_err(|source| Error::InvalidPattern { pattern: r"\{\{(.*?)}}".to_string(), source })?;
        let translations_title = Regex::new(r"^([^/]+)/translations$")
            .map_err(|source| Error::InvalidPattern { pattern: r"^([^/]+)/translations$".to_string(), source })?;
        let language = Regex::new(r"^==([^=]+)==$")
            .map_err(|source| Error::InvalidPattern { pattern: r"^==([^=]+)==$".to_string(), source })?;
        let translation_name = Regex::new(r"^\* ([^:]+):")
            .map_err(|source| Error::InvalidPattern { pattern: r"^\* ([^:]+):".to_string(), source })?;
        let prefix = format!("^\\*.?\\s{}:", lang_prefix);
        let prefix = Regex::new(&prefix)
            .map_err(|source| Error::InvalidPattern { pattern: prefix.clone(), source })?;

        Ok(Re {
            data,
            translations_title,
            language,
            translation_name,
            prefix,
        })
    }
}

pub fn read_wiki(dict: &mut Dictionary, path: &str, prefix: &str) -> Result<()> {
    let file = std::fs::File::open(path)?;
    let file = std::io::BufReader::new(file);
    let file = bzip2::bufread::MultiBzDecoder::new(file);
    let file = std::io::BufReader::new(file);
    let mut reader = Reader::from_reader(file);

    let mut buf = Vec::new();
    let mut page = Page::empty();
    let mut state = State::None;
    let re = Re::new(prefix)?;

    loop {
        let event = reader.read_event_into(&mut buf)
            .map_err(|e| Error::Xml { position: reader.error_position(), source: e })?;

        match event {
            Event::Eof => break,

            Event::Start(e) => {
                match e.name().as_ref() {
                    b"page" => {
                        state = State::Page;
                        page = Page::empty();
                    }

                    b"title" => {
                        state = State::Title;
                        page.title = "".to_string();
                    }

                    b"text" => {
                        state = State::Content;
                        page.content = "".to_string()
                    }

                    _ => (),
                }
            }

            Event::End(e) => {
                match e.name().as_ref() {
                    b"page" => {
                        state = State::None;
                        if !page.title.contains(":") {
                            read_wiki_page(dict, &page, &re)?;
                        }
                    }

                    b"title" => {
                        state = State::Page;
                    }

                    b"text" => {
                        state = State::Page;
                    }

                    _ => ()
                }
            }

            Event::Text(e) => {
                let text = e.decode()
                    .map(|t| t.into_owned())
                    .map_err(Error::TextDecode)?;
                match state {
                    State::Title => {
                        page.title.push_str(&text);
                    }

                    State::Content => {
                        page.content.push_str(&text);
                    }

                    _ => ()
                }
            }

            // There are several other `Event`s we do not consider here
            _ => (),
        }
        // if we don't keep a borrow elsewhere, we can clear the buffer to keep memory usage low
        buf.clear();
    }

    Ok(())
}

fn read_wiki_page(dict: &mut Dictionary, page: &Page, re: &Re) -> Result<()> {
    let mut headword: &str = page.title.trim();
    if let Some(captures) = re.translations_title.captures(headword) {
        if let Some(captured) = captures.get(1) {
            headword = captured.as_str();
        }
    }

    let mut current_word_class = WordClass::Unknown;
    let mut current_meaning = Meaning::new("");
    let mut current_language = String::from("");
    let mut current_translation_name = String::from("");

    for line in page.content.lines() {
        if let Some(captures) = re.language.captures(line) {
            if let Some(captured) = captures.get(1) {
                current_language = captured.as_str().to_lowercase();
            }
        }

        if !current_language.is_empty() && current_language != "english" {
            break;
        }

        if let Some(captures) = re.translation_name.captures(line) {
            if let Some(captured) = captures.get(1) {
                current_translation_name = captured.as_str().to_lowercase();
            }
        }

        for data in re.data.find_iter(line) {
            let Some(captures) = re.data.captures(data.as_str()) else {
                continue;
            };
            let Some(first) = captures.get(1) else {
                continue;
            };
            let parts = first.as_str().split("|").collect::<Vec<&str>>();
            let control = parts[0].trim();
            match control {
                "IPA" => {
                    if parts.len() > 1 && parts[1] != dict.source_language {
                        continue;
                    }

                    if current_language != "english" {
                        continue;
                    }

                    for part in parts.iter().skip(2) {
                        let pronunciation = part.trim();
                        if !pronunciation.starts_with("/") {
                            continue;
                        }
                        dict.add_pronunciation(headword, "wiki", pronunciation);
                    }
                }

                "trans-top" => {
                    if !current_meaning.is_empty() {
                        dict.add_meaning(headword, &current_word_class, &current_meaning);
                    }

                    if parts.len() < 2 {
                        current_meaning = Meaning::new("");
                        continue;
                    }

                    if current_language == "english" {
                        current_meaning = Meaning::new(parts[1].trim());
                    } else {
                        current_meaning = Meaning::new("");
                    }
                }

                "trans-bottom" => {
                    if !current_meaning.is_empty() {
                        dict.add_meaning(headword, &current_word_class, &current_meaning);
                        current_meaning = Meaning::new("");
                    }
                }

                "en-noun" => current_word_class = WordClass::Noun,
                "en-pron" => current_word_class = WordClass::Pronoun,
                "en-adv" => current_word_class = WordClass::Adverb,
                "en-det" => current_word_class = WordClass::Determiner,
                "en-con" => current_word_class = WordClass::LinkingWord,
                "en-verb" => current_word_class = WordClass::Verb,
                "en-adj" => current_word_class = WordClass::Adjective,
                "en-prep" => current_word_class = WordClass::Preposition,

                _ => if parts.len() > 2 && re.prefix.is_match(line) {
                    if dict.target_language == "sh" && current_translation_name != "serbo-croatian" {
                        continue;
                    }
                    let translation = parts[2].trim();
                    current_meaning.add_translation(translation);
                }
            }
        }
    }

    if !current_meaning.is_empty() {
        dict.add_meaning(headword, &current_word_class, &current_meaning);
    }

    Ok(())
}
