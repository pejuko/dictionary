mod cli_config;
mod dictionary;

use std::{env, process};
use std::error::Error;

use cli_config::CliConfig;
use dictionary::Dictionary;

fn main() -> Result<(), Box<dyn Error>> {
    let config = CliConfig::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if config.print_help {
        CliConfig::print_help();
        process::exit(0);
    }

    println!("{:#?}", &config);

    if config.query.is_none() && config.output_path.is_none() {
        Err("No search (-s) or output path (-o) is specified.")?;
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
            Err("No reverse title (-rt) is specified.")?;
        }
    }

    Ok(())
}
