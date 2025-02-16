use std::error::Error;

use crate::dictionary::Dictionary;

pub fn read_pronunciation(dict: &mut Dictionary, name: &str, path: &str) -> Result<(), Box<dyn Error>> {
    let lines = super::read_tab_file(path)?;

    for line in lines {
        let headword = line[0].trim();
        for pronunciation in line[1].split(',') {
            dict.add_pronunciation(headword, name, pronunciation.trim());
        }
    }

    Ok(())
}