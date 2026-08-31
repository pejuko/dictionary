mod cli_config;
mod dictionary;
mod download;
mod error;

use std::env;

use cli_config::CliConfig;
use dictionary::Dictionary;
use error::Error;

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(1);
    }
}

fn run() -> error::Result<()> {
    let config = CliConfig::build(env::args())
        .unwrap_or_else(|err| {
            eprintln!("{err}");
            std::process::exit(1);
        });

    if config.print_help {
        CliConfig::print_help();
        std::process::exit(0);
    }

    if config.default_mode {
        let path = download::ensure_wiktionary()?;
        println!("Wiktionary available at: {}", path.display());
        return Ok(());
    }

    println!("{:#?}", &config);

    if config.query.is_none() && config.output_path.is_none() {
        return Err(Error::MissingQueryOrOutput);
    }

    let dict = Dictionary::build(&config)?;

    println!("Records: {}", dict.len());
    println!("Non-empty records: {}", dict.non_empty_len());
    println!("Translated records: {}", dict.translations_len());

    if let Some(query) = &config.query {
        println!("{:#?}", dict.lookup(query));
    }

    if let Some(output_path) = &config.output_path {
        dict.to_kindle(output_path, config.force)?;
    }

    if let Some(reverse_output_path) = &config.reverse_output_path {
        if let Some(reverse_title) = &config.reverse_title {
            let reversed_dict = dict.reverse(reverse_title);
            println!("Records in reversed dictionary: {}", reversed_dict.len());
            reversed_dict.to_kindle(reverse_output_path, config.force)?;
        } else {
            return Err(Error::MissingReverseTitle);
        }
    }

    Ok(())
}
