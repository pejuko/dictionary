use crate::dictionary::Dictionary;
use crate::error::Result;

pub fn read_pronunciation(dict: &mut Dictionary, name: &str, path: &str) -> Result<()> {
    let lines = super::read_tab_file(path)?;

    for line in lines {
        let Some(pronunciation_field) = line.get(1) else {
            continue;
        };
        let headword = line[0].trim();
        for pronunciation in pronunciation_field.split(',') {
            dict.add_pronunciation(headword, name, pronunciation.trim());
        }
    }

    Ok(())
}
