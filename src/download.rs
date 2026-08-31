use std::fs;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use indicatif::{ProgressBar, ProgressStyle};

use crate::error::{Error, Result};

const LATEST_URL: &str =
    "https://dumps.wikimedia.org/enwiktionary/latest/enwiktionary-latest-pages-articles.xml.bz2";
const TARGET_PATH: &str = "data/enwiktionary-latest-pages-articles.xml.bz2";
const USER_AGENT: &str = "dictionary-cli/0.1 (kindle dictionary builder)";

fn client() -> Result<reqwest::blocking::Client> {
    let client = reqwest::blocking::Client::builder()
        .user_agent(USER_AGENT)
        .build()?;
    Ok(client)
}

pub fn ensure_wiktionary() -> Result<PathBuf> {
    let target = PathBuf::from(TARGET_PATH);

    if let Ok(metadata) = fs::metadata(&target) {
        if metadata.is_file() && metadata.len() > 0 {
            let remote_size = get_remote_file_size(LATEST_URL)?;
            if let Some(size) = remote_size {
                println!("Local size: {}", metadata.len());
                println!("Remote size: {size}");
                if size == metadata.len() {
                    return Ok(target);
                }
            } else {
                return Ok(target);
            }
        }
    }

    download(LATEST_URL, &target)?;
    Ok(target)
}

fn get_remote_file_size(url: &str) -> Result<Option<u64>> {
    let client = client()?;
    let response = client.get(url).send()?.error_for_status()?;
    Ok(response.content_length())
}

fn download(url: &str, target: &Path) -> Result<()> {
    println!("Downloading: {url}");

    if let Some(parent) = target.parent() {
        fs::create_dir_all(parent)?;
    }

    let file_name = target
        .file_name()
        .map(|name| name.to_string_lossy().to_string())
        .unwrap_or_else(|| "download".to_string());
    let tmp_path = target.with_file_name(format!(".{file_name}.part"));

    let client = client()?;
    let response = client.get(url).send()?.error_for_status()?;
    let total = response.content_length();
    let bar = build_progress_bar(total)?;

    let mut file = fs::File::create(&tmp_path)?;
    let mut buffer = [0u8; 256 * 1024];
    let result = stream_to_file(response, &mut file, &mut buffer, &bar);

    if result.is_err() {
        let _ = fs::remove_file(&tmp_path);
    }
    bar.finish_and_clear();
    result?;

    fs::rename(&tmp_path, target)?;
    Ok(())
}

fn stream_to_file(
    mut response: reqwest::blocking::Response,
    file: &mut fs::File,
    buffer: &mut [u8],
    bar: &ProgressBar,
) -> Result<()> {
    loop {
        let len = response.read(buffer)?;
        if len == 0 {
            break;
        }
        file.write_all(&buffer[..len])?;
        bar.inc(len as u64);
    }
    file.flush()?;
    Ok(())
}

fn build_progress_bar(total: Option<u64>) -> Result<ProgressBar> {
    let template =
        "{spinner:.green} {bytes:>7} {ps} {percent:>3}% {bar:40.cyan/blue} {bytes}/{total_bytes}";
    let style = ProgressStyle::with_template(template)
        .map_err(|err| Error::ProgressTemplate(err.to_string()))?
        .progress_chars("#>-");

    let bar = match total {
        Some(len) => ProgressBar::new(len),
        None => ProgressBar::new_spinner(),
    };

    bar.enable_steady_tick(std::time::Duration::from_millis(100));
    bar.set_style(style);
    Ok(bar)
}
