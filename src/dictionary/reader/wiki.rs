extern crate bzip2;

use regex::Regex;

use quick_xml::events::Event;
use quick_xml::reader::Reader;

use std::error::Error;

use crate::dictionary::{Dictionary, Meaning, WordClass};

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
    prefix: Regex,
}

impl Re {
    fn new(lang_prefix: &str) -> Re {
        let prefix = format!("^\\*.?\\s{}:", lang_prefix);
        Re {
            data: Regex::new(r"\{\{(.*?)}}").unwrap(),
            translations_title: Regex::new(r"^([^/]+)/translations$").unwrap(),
            language: Regex::new(r"^==([^=]+)==$").unwrap(),
            prefix: Regex::new(&prefix).unwrap(),
        }
    }
}

pub fn read_wiki(dict: &mut Dictionary, path: &str, prefix: &str) -> Result<(), Box<dyn Error>> {
    let file = std::fs::File::open(path).unwrap();
    let file = std::io::BufReader::new(file);
    let file = bzip2::bufread::MultiBzDecoder::new(file);
    let file = std::io::BufReader::new(file);
    let mut reader = Reader::from_reader(file);

    // let mut count = 0;
    // let mut txt = Vec::new();
    let mut buf = Vec::new();
    let mut page = Page::empty();
    let mut state = State::None;
    let re = Re::new(prefix);

    loop {
        match reader.read_event_into(&mut buf) {
            Err(e) => panic!("Error at position {}: {:?}", reader.error_position(), e),

            Ok(Event::Eof) => break,

            Ok(Event::Start(e)) => {
                match e.name().as_ref() {
                    b"page" => {
                        state = State::Page;
                        page = Page::empty();
                    },

                    b"title" => {
                        state = State::Title;
                        page.title = "".to_string();
                    },

                    b"text" => {
                        state = State::Content;
                        page.content = "".to_string()
                    },

                    _ => (),
                }
            },

            Ok(Event::End(e)) => {
                match e.name().as_ref() {
                    b"page" => {
                        state = State::None;
                        if !page.title.contains(":") {
                            read_wiki_page(dict, &page, &re);
                        }
                    },

                    b"title" => {
                        state = State::Page;
                    },

                    b"text" => {
                        state = State::Page;
                    }

                    _ => ()
                }
            }

            Ok(Event::Text(e)) => {
                let str = e.unescape().unwrap().into_owned();
                match state {
                    State::Title => {
                        page.title.push_str(&str);
                    },

                    State::Content => {
                        page.content.push_str(&str);
                    }

                    _ => ()
                }
            },

            // There are several other `Event`s we do not consider here
            _ => (),
        }
        // if we don't keep a borrow elsewhere, we can clear the buffer to keep memory usage low
        buf.clear();
    }

    Ok(())
}

fn read_wiki_page(dict: &mut Dictionary, page: &Page, re: &Re) {
    let mut headword = page.title.trim();
    if let Some(captures) = re.translations_title.captures(headword) {
        headword = captures.get(1).unwrap().as_str();
    }

    let mut current_word_class = WordClass::Unknown;
    let mut current_meaning = Meaning::new("");
    let mut current_language = String::from("");

    for line in page.content.lines() {
        if let Some(captures) = re.language.captures(line) {
            current_language = captures.get(1).unwrap().as_str().to_lowercase();
        }

        if !current_language.is_empty() && current_language != "english" {
            break;
        }

        for data in re.data.find_iter(line) {
            let Some(captures) = re.data.captures(data.as_str()) else { todo!() };
            let parts = captures.get(1).unwrap().as_str().split("|").collect::<Vec<&str>>();
            let control = parts[0].trim();
            match control {
                "IPA" => {
                    if parts.len() > 1 && parts[1] != dict.source_language {
                        continue;
                    }

                    if current_language != "english" {
                        continue;
                    }

                    for i in 2..parts.len() {
                        let pronunciation = parts[i].trim();
                        if !pronunciation.starts_with("/") {
                            continue;
                        }
                        dict.add_pronunciation(headword, "wiki", pronunciation);
                    }
                },

                "trans-top" => {
                    if !current_meaning.is_empty() {
                        dict.add_meaning(headword, current_word_class.clone(), current_meaning.clone());
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
                },

                "trans-bottom" => {
                    if !current_meaning.is_empty() {
                        dict.add_meaning(headword, current_word_class.clone(), current_meaning.clone());
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
                    let translation = parts[2].trim();
                    current_meaning.add_translation(translation);
                }
            }
        }
    }

    if !current_meaning.is_empty() {
        dict.add_meaning(headword, current_word_class.clone(), current_meaning.clone());
    }
}
