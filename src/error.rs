use std::path::PathBuf;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Http(#[from] reqwest::Error),

    #[error("XML error at position {position}: {source}")]
    Xml {
        position: u64,
        #[source]
        source: quick_xml::errors::Error,
    },

    #[error("invalid text: {0}")]
    TextDecode(#[from] quick_xml::encoding::EncodingError),

    #[error("invalid progress bar template: {0}")]
    ProgressTemplate(String),

    #[error("invalid regular expression pattern {pattern:?}: {source}")]
    InvalidPattern {
        pattern: String,
        #[source]
        source: regex::Error,
    },

    #[error("missing value for {flag}")]
    MissingArgument { flag: &'static str },

    #[error("illegal argument: {0}")]
    IllegalArgument(String),

    #[error("File does not exist: {path}")]
    FileNotFound { path: PathBuf },

    #[error("cannot stat file {path}")]
    FileCheckFailed { path: PathBuf },

    #[error("Pronunciation must have 2 parts: '<name>:<filename>', got: {0}")]
    PronunciationFormat(String),

    #[error("unsupported language combination: {source_lang}-{target_lang}")]
    UnsupportedLanguagePair { source_lang: String, target_lang: String },

    #[error("no wiki prefix specified")]
    MissingWikiPrefix,

    #[error("No search (-s) or output path (-o) is specified.")]
    MissingQueryOrOutput,

    #[error("No reverse title (-rt) is specified.")]
    MissingReverseTitle,

    #[error("{path} is a file, a directory expected.")]
    OutputIsFile { path: String },

    #[error("{path} is an existing directory, use -f to force.")]
    OutputExists { path: String },

    #[error("internal invariant violated: {0}")]
    Invariant(String),
}

pub type Result<T> = std::result::Result<T, Error>;
