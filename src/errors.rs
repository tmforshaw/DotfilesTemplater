use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum DotfilesError {
    #[error("Regex failed to be created")]
    RegexFail(#[from] regex::Error),

    #[error("Regex '{regex_str}' did not match anything in haystack: {hay}")]
    RegexMatchError { regex_str: String, hay: String },

    #[error("Function '{name}' needs {needed} args, found {found}: {args:?}")]
    ArgumentError {
        name: String,
        needed: usize,
        found: usize,
        args: Vec<String>,
    },
}
