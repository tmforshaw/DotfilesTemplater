use std::collections::HashMap;

use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum DotfilesError {
    #[error("TOML could not be read to string: {0}")]
    TomlReadError(#[from] toml::de::Error),

    #[error("File could not be read: {0}")]
    FileReadError(String),

    #[error("Regex failed to be created: {0}")]
    RegexFail(#[from] regex::Error),

    #[error("Regex '{regex_str}' did not match anything in haystack: {hay}")]
    RegexMatchError { regex_str: String, hay: String },

    #[error("Regex '{regex_str}' did not produce {} captures: {hay}", capture_index + 1)]
    RegexNthMatchError {
        regex_str: String,
        hay: String,
        capture_index: usize,
    },

    #[error("Regex capture at index {index} could not be found: {captures}")]
    CaptureFail { captures: String, index: usize },

    #[error("Function '{name}' needs {needed} args, found {}: {args:?}", args.len())]
    FuncArgumentError {
        name: String,
        needed: usize,
        args: Vec<String>,
    },

    #[error("Tried to replace text with different length string (Length: {}  -->  {}): '{text_to_replace}'    -->    '{replace_text}'", text_to_replace.len(), replace_text.len())]
    ReplaceTextDifferentLength {
        text_to_replace: String,
        replace_text: String,
    },

    #[error("Theme '{name}' was not found in themes: {themes:?}")]
    ThemeNotFound { name: String, themes: Vec<String> },

    #[error("Argument '{arg}' not found in theme: {theme_hashmap:?}")]
    ArgNotFound {
        arg: String,
        theme_hashmap: HashMap<String, String>,
    },
}

impl From<std::io::Error> for DotfilesError {
    fn from(value: std::io::Error) -> Self {
        Self::FileReadError(value.to_string())
    }
}
