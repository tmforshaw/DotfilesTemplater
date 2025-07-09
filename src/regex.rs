use regex::Regex;

use crate::config::FUNCTION_CHAR;
use crate::errors::DotfilesError;
use crate::file::{MatchedText, open_file};
use std::sync::LazyLock;

pub static FUNCTION_REGEX: LazyLock<Result<Regex, DotfilesError>> = LazyLock::new(|| {
    Ok(Regex::new(
        format!("{FUNCTION_CHAR}(?<name>[a-zA-Z][A-Za-z\\d_\\-]*)(?<args>(?:\\\\.|[^)])+\\))")
            .as_str(),
    )?)
});
pub static PATTERN_REGEX: LazyLock<Result<Regex, DotfilesError>> =
    LazyLock::new(|| Ok(Regex::new(r"'[^']+'")?));
pub static STRING_OR_KEYWORD_REGEX: LazyLock<Result<Regex, DotfilesError>> =
    LazyLock::new(|| Ok(Regex::new("('[^']+')|([a-zA-Z][\\w\\-]*)")?));
pub static HEX_COLOUR_REGEX: LazyLock<Result<Regex, DotfilesError>> =
    LazyLock::new(|| Ok(Regex::new("#[A-Za-z\\d]{6}")?));

pub fn matches_pattern(arg: &str) -> Result<(), DotfilesError> {
    let pattern_regex = (*PATTERN_REGEX).clone()?;

    if pattern_regex.is_match(arg) {
        Ok(())
    } else {
        Err(DotfilesError::RegexMatchError {
            regex_str: pattern_regex.to_string(),
            hay: arg.to_string(),
        })
    }
}

pub fn matches_keyword_or_string(arg: &str) -> Result<(), DotfilesError> {
    let string_or_keyword_regex = (*STRING_OR_KEYWORD_REGEX).clone()?;

    if string_or_keyword_regex.is_match(arg) {
        Ok(())
    } else {
        Err(DotfilesError::RegexMatchError {
            regex_str: string_or_keyword_regex.to_string(),
            hay: arg.to_string(),
        })
    }
}

pub fn get_single_match(regex: &Regex, text: MatchedText) -> Result<MatchedText, DotfilesError> {
    // Get the capture for this regex and text
    let Some(captures) = regex.captures(&text.text) else {
        return Err(DotfilesError::RegexMatchError {
            regex_str: regex.to_string(),
            hay: text.text,
        });
    };

    // Exit function if the captured text cannot be extracted into a Match variable
    let Some(regex_match) = captures.get(0) else {
        return Err(DotfilesError::CaptureFail {
            captures: format!("{captures:?}"),
            index: 0,
        });
    };

    // Adjust the range to be with the respect to the whole file (given that the input MatchedText is also with respect to the whole file)
    let matched_text = MatchedText {
        range: (text.range.start + regex_match.start())..(text.range.start + regex_match.end()),
        text: regex_match.as_str().to_string(),
    };

    Ok(matched_text)
}

pub fn get_nth_match(
    regex: &Regex,
    text: MatchedText,
    n: usize,
) -> Result<MatchedText, DotfilesError> {
    // Get the capture for this regex and text
    let Some(captures) = regex.captures_iter(&text.text).nth(n) else {
        return if n == 0 {
            Err(DotfilesError::RegexMatchError {
                regex_str: regex.to_string(),
                hay: text.text,
            })
        } else {
            Err(DotfilesError::RegexNthMatchError {
                regex_str: regex.to_string(),
                hay: text.text,
                capture_index: n,
            })
        };
    };

    // Exit function if the captured text cannot be extracted into a Match variable
    let Some(regex_match) = captures.get(0) else {
        return Err(DotfilesError::CaptureFail {
            captures: format!("{captures:?}"),
            index: 0,
        });
    };

    // Adjust the range to be with the respect to the whole file (given that the input MatchedText is also with respect to the whole file)
    let matched_text = MatchedText {
        range: (text.range.start + regex_match.start())..(text.range.start + regex_match.end()),
        text: regex_match.as_str().to_string(),
    };

    Ok(matched_text)
}
#[allow(dead_code)]
fn test_fn_print_chars(
    file_path: String,
    range: std::ops::Range<usize>,
) -> Result<(), DotfilesError> {
    let f = open_file(file_path)?;

    println!("{:?}", &f.chars().collect::<Vec<_>>()[range]);

    Ok(())
}
