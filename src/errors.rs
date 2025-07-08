use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub(crate) enum DotfilesError {
    #[error("TOML could not be read to string: {0}")]
    TomlReadError(#[from] toml::de::Error),

    #[error("Regex failed to be created")]
    RegexFail(#[from] regex::Error),

    #[error("Regex '{regex_str}' did not match anything in haystack: {hay}")]
    RegexMatchError { regex_str: String, hay: String },

    #[error("Regex capture at index {index} could not be found: {captures}")]
    CaptureFail { captures: String, index: usize },

    #[error("Function '{name}' needs {needed} args, found {found}: {args:?}")]
    ArgumentError {
        name: String,
        needed: usize,
        found: usize,
        args: Vec<String>,
    },
}
