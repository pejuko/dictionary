pub mod gnu_fdl;
pub mod pronunciation;
pub mod wiki;

use std::error::Error;
use std::fs;

type LineType = Vec<String>;

pub fn read_tab_file(path: &str) -> Result<Vec<LineType>, Box<dyn Error>> {
    let contents = fs::read_to_string(path)?;
    let lines = contents
        .lines()
        .filter(|line| !line.starts_with("#"))
        .map(|line| {
            line.split('\t').map(|element| element.to_string()).collect::<LineType>()
        })
        .filter(|line| line.len() > 1)
        .collect::<Vec<LineType>>();

    Ok(lines)
}