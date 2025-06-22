use std::fs;

#[derive(Debug)]
pub struct CliConfig {
    pub input_file_path: Option<String>,
    pub pronunciation_files: Vec<(String, String)>,
    pub wiki_file_path: Option<String>,
    pub output_path: Option<String>,
    pub query: Option<String>,
    pub wiki_prefix: Option<String>,
    pub source_language: String,
    pub target_language: String,
    pub title: String,
    pub author: String,
    pub force: bool,
    pub print_help: bool,
    pub reverse_output_path: Option<String>,
    pub reverse_title: Option<String>,
}

impl CliConfig {
    pub fn new() -> Self {
        CliConfig {
            input_file_path: None,
            pronunciation_files: Vec::new(),
            wiki_file_path: None,
            output_path: None,
            query: None,
            wiki_prefix: None,
            force: false,
            source_language: "en".to_string(),
            target_language: "cs".to_string(),
            title: "".to_string(),
            author: "".to_string(),
            print_help: false,
            reverse_output_path: None,
            reverse_title: None,
        }
    }

    // parse args into CliConfig
    pub fn build(mut args: impl Iterator<Item=String>) -> Result<CliConfig, &'static str> {
        args.next();

        let mut config = Self::new();
        let mut has_params = false;

        while let Some(arg) = args.next() {
            has_params = true;

            match arg.as_str() {
                "-i" => config.input_file_path = Some(Self::get_file_name(args.next())?),
                "-o" => config.output_path = Some(Self::get_param_value(args.next())?),
                "-ro" => config.reverse_output_path = Some(Self::get_param_value(args.next())?),
                "-w" => config.wiki_file_path = Some(Self::get_param_value(args.next())?),
                "-s" => config.query = Some(Self::get_param_value(args.next())?),
                "-wp" => config.wiki_prefix = Some(Self::get_param_value(args.next())?),
                "-p" => config.pronunciation_files.push(Self::get_pronunciation(args.next())?),
                "-f" => config.force = true,
                "-sl" => config.source_language = Self::get_param_value(args.next())?,
                "-tl" => config.target_language = Self::get_param_value(args.next())?,
                "-t" => config.title = Self::get_param_value(args.next())?,
                "-rt" => config.reverse_title = Some(Self::get_param_value(args.next())?),
                "-a" => config.author = Self::get_param_value(args.next())?,
                "-h" => config.print_help = true,
                _ => return Err("Illegal argument"),
            }
        }

        if !has_params {
            config.print_help = true;
        }

        Ok(config)
    }

    // convert Option to Result
    fn get_param_value(param: Option<String>) -> Result<String, &'static str> {
        param.ok_or("Missing parameter")
    }

    // get the param value and check if the file exists
    fn get_file_name(param: Option<String>) -> Result<String, &'static str> {
        let file_name = CliConfig::get_param_value(param)?;
        match fs::exists(&file_name) {
            Ok(res) => if res { Ok(file_name) } else { Err("File does not exist") },
            Err(_) => Err("Cannot find file"),
        }
    }

    fn get_pronunciation(param: Option<String>) -> Result<(String, String), &'static str> {
        let name_and_file_name = CliConfig::get_param_value(param)?;
        let name_and_file_name = name_and_file_name.split(":").collect::<Vec<&str>>();
        if name_and_file_name.len() != 2 {
            return Err("Pronunciation must have 2 parts: '<name>:<filename>'");
        }

        let name = name_and_file_name.first().unwrap().trim().to_string();
        let file_name = CliConfig::get_file_name(Some(name_and_file_name.get(1).unwrap().to_string()))?;

        Ok((name, file_name))
    }

    pub fn print_help() {
        println!(
r#"
All input files must be encoded in UTF-8.

Dictionary parameters:

    -i      Path to tab delimited input file where first column is in source language
            and second column is in target language.

    -w      Path to wiktionary file in xml.bz2 format. Requires -wp parameter.

    -wp     Wiki prefix e.g. Czech or German or ...

    -p      Pronunciation file with a name. e.g. US:data/en_US.txt where US is name
            and data/en_US.txt tab delimited file where first column is a word in
            source language and in second column is pronunciation. You can use this
            parametr multiple times.

    -o      Output directory

    -ro      Output directory for reversed dictionary

    -s      Search given word in builded dictionary.

    -f      Force output if the output directory exists and overwrite the files.

    -sl     Source language. Default en.

    -tl     Target language. Default cs.

    -t      Dictionary title.

    -rt      Dictionary title for reversed dictionary.

    -a      Author.

    -h      This help.

Examples:

    It is recommended to build the app in release mode. Processing
    wiktionary data may be very slow.

    To generate English-Czech dictionary run:

    cargo run --release -- -w data/enwiktionary.xml.bz2 -wp Czech -o data/kindle-en-cs -t "English-Czech dictionary" -a pejuko
"#
        )
    }
}

impl Default for CliConfig {
    fn default() -> Self {
        Self::new()
    }
}
