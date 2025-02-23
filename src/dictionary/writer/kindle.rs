use std::collections::HashSet;
use std::{collections::HashMap, error::Error};
use std::fs;
use std::io::Write;

use crate::dictionary::{Dictionary, Meaning, Term};

use super::escape_xml;

pub fn to_kindle(dict: &Dictionary, output_path: &str, force: bool) -> Result<(), Box<dyn Error>> {
    let output = fs::metadata(output_path);
    match output {
        Ok(metadata) => {
            if metadata.is_file() {
                Err(format!("{} is a file, a directory expected.", output_path))?;
            } else if metadata.is_dir() && !force {
                Err(format!("{} is an existing directory, use -f to force.", output_path))?;
            }
        },

        Err(_) => {
            fs::create_dir_all(output_path)?;
        }
    }

    let files = create_kindle_content_files(dict, output_path)?;
    create_kindle_opf_file(dict, output_path, &files)?;

    Ok(())
}

fn create_kindle_content_files(dict: &Dictionary, output_path: &str) -> Result<Vec<(String, String)>, Box<dyn Error>> {
    let keys = dict.terms.keys().collect::<Vec<_>>();
    let mut pos: usize = 0;
    let batch_size: usize = 30_000;
    let mut files = Vec::<(String, String)>::new();
    let mut i = 1;
    while pos < keys.len() {
        let mut max = pos + batch_size;
        if max > keys.len() {
            max = keys.len();
        }
        let id = format!("content{:04}", i);
        let path = format!("{}/{}.xhtml", output_path, id);
        create_kindle_content_file(dict, &keys[pos..max], path.as_str())?;
        files.push((id.clone(), path.clone()));
        pos += batch_size;
        i += 1;
    }
    Ok(files)
}

fn create_kindle_content_file(dict: &Dictionary, keys: &[&String], content_file_path: &str) -> Result<(), Box<dyn Error>> {
    let mut f = fs::File::create(content_file_path)?;

    start_kindle_content_file(&mut f)?;

    for &key in keys.iter() {
        let term = dict.terms.get(key).unwrap();

        if term.is_empty() {
            continue;
        }

        let mut out_str = r#"
        <idx:entry name="main" scriptable="yes" spell="yes">
"#.to_string();

        format_headword(&mut out_str, term);
        format_pronunciations(&mut out_str, term);
        format_classes(&mut out_str, term);

        out_str.push_str("\n</idx:entry>\n");

        f.write_all(out_str.as_bytes())?;
    }

    end_kindle_content_file(&mut f)?;

    Ok(())
}

fn format_headword(out_str: &mut String, term: &Term) {
    out_str.push_str(format!("<b><idx:orth>{}", super::escape_xml(&term.headword)).as_str());
    if !term.inflections.is_empty() {
        out_str.push_str("<idx:infl>");
        for inflection in term.inflections.iter() {
            out_str.push_str(format!("<idx:iform value=\"{}\" />", super::escape_xml(inflection)).as_str());
        }
        out_str.push_str("</idx:infl>");
    }
    out_str.push_str("</idx:orth></b><br />");
}

fn format_pronunciations(out_str: &mut String, term: &Term) {
    let mut pron_keys = term.pronunciations.keys().collect::<Vec<_>>();
    pron_keys.sort();
    for name in pron_keys {
        if term.pronunciations.len() > 1 && name == "wiki" {
            // skip wiki pronunciation if we have record from other sources
            continue;
        }
        let pronunciations = term.pronunciations.get(name).unwrap();
        if !name.is_empty() && name != "wiki" {
            out_str.push_str(format!("<i>{}</i>: ", name).as_str());
        }
        out_str.push_str(escape_xml(pronunciations.join(", ").as_str()).as_str());
        out_str.push_str("<br />\n");
    }
}

fn format_classes(out_str: &mut String, term: &Term) {
    let mut classes = term.classes.keys().collect::<Vec<_>>();
    classes.sort();
    for word_class in classes {
        let meanings = term.classes.get(word_class).unwrap();
        if meanings.is_empty() {
            continue;
        }

        out_str.push_str(word_class.as_str());
        format_meanings(out_str, meanings);
    }
}

fn format_meanings(out_str: &mut String, meanings: &HashMap<String, Meaning>) {
    let mut translations = HashSet::new();
    for meaning in meanings.values() {
        for translation in &meaning.translations {
            translations.insert(translation);
        }
    }
    if !translations.is_empty() {
        out_str.push_str("<ul>\n");
        format_translations(out_str, &translations);
        out_str.push_str("</ul>\n");
    }

    out_str.push_str("<ol>\n");
    for meaning in meanings.values() {
        if meaning.description.is_empty() {
            continue;
        }
        out_str.push_str(format!("<li>{}</li>\n", escape_xml(&meaning.description)).as_str());
    }
    out_str.push_str("</ol>\n");
}

fn format_translations(out_str: &mut String, translations: &HashSet<&String>) {
    let transl = translations.iter().map(|&string| string.clone()).collect::<Vec<String>>();

    out_str.push_str(format!("<li>{}</li>\n", escape_xml(transl.join(" | ").as_str())).as_str());
}

fn start_kindle_content_file(f: &mut fs::File) -> Result<(), Box<dyn Error>> {
    f.write_all(r#"<html xmlns:math="http://exslt.org/math" xmlns:svg="http://www.w3.org/2000/svg"
    xmlns:tl="https://kindlegen.s3.amazonaws.com/AmazonKindlePublishingGuidelines.pdf" xmlns:saxon="http://saxon.sf.net/"
    xmlns:xs="http://www.w3.org/2001/XMLSchema" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
    xmlns:cx="https://kindlegen.s3.amazonaws.com/AmazonKindlePublishingGuidelines.pdf"
    xmlns:dc="http://purl.org/dc/elements/1.1/"
    xmlns:mbp="https://kindlegen.s3.amazonaws.com/AmazonKindlePublishingGuidelines.pdf"
    xmlns:mmc="https://kindlegen.s3.amazonaws.com/AmazonKindlePublishingGuidelines.pdf"
    xmlns:idx="https://kindlegen.s3.amazonaws.com/AmazonKindlePublishingGuidelines.pdf">
<head>
    <meta http-equiv="Content-Type" content="text/html; charset=utf-8" />
</head>
<body>
    <mbp:frameset>
"#.as_bytes())?;

    Ok(())
}

fn end_kindle_content_file(f: &mut fs::File) -> Result<(), Box<dyn Error>> {
    f.write_all(r#"
    </mbp:frameset>
</body>
</html>
"#.as_bytes())?;

    Ok(())
}

fn create_kindle_opf_file(dict: &Dictionary, output_path: &str, files: &Vec<(String, String)>) -> Result<(), Box<dyn Error>> {
    let opf_file_path = format!("{}/content.opf", output_path);
    let mut f = fs::File::create(opf_file_path)?;

    f.write_all(format!(r#"<?xml version="1.0"?>
<package version="2.0" xmlns="http://www.idpf.org/2007/opf" unique-identifier="en-cs-dict">
    <metadata>
        <dc:title>{}</dc:title>
        <dc:creator opf:role="aut">{}</dc:creator>
        <dc:language>{}</dc:language>
        <meta name="cover" content="my-cover-image" />
        <x-metadata>
          <DictionaryInLanguage>{}</DictionaryInLanguage>
          <DictionaryOutLanguage>{}</DictionaryOutLanguage>
        </x-metadata>
    </metadata>
    <manifest>
        <item href="dict.png" id="my-cover-image" media-type="image/png" />
"#, dict.title, dict.author, dict.source_language, dict.source_language, dict.target_language).as_bytes())?;
    for file in files {
        let id = &file.0;
        f.write_all(format!("<item id=\"{id}\" href=\"{id}.xhtml\" media-type=\"application/xhtml+xml\" />\n").as_bytes())?;
    }

    f.write_all(r#"
    </manifest>
    <spine>
"#.as_bytes())?;

    for file in files {
        let id = &file.0;
        f.write_all(format!("<itemref idref=\"{id}\"/>\n").as_bytes())?;
    }

    f.write_all(r#"
    </spine>
</package>
"#.as_bytes())?;

    Ok(())
}
