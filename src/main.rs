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

    if let Some(query) = &config.query {
        println!("{:#?}", dict.lookup(query));
    }

    if let Some(output_path) = &config.output_path {
        dict.to_kindle(output_path, config.force)?;
    }


    Ok(())
}
