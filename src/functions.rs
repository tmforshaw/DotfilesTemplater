use regex::Regex;
use std::sync::LazyLock;

use crate::arguments::parse_argument;
use crate::config::FUNCTION_CHAR;
use crate::errors::DotfilesError;
use crate::file::{MatchedText, write_to_file};

pub static FUNCTION_REGEX: LazyLock<Result<Regex, DotfilesError>> = LazyLock::new(|| {
    Ok(Regex::new(
        format!("{FUNCTION_CHAR}(?<name>\\w[\\w\\d\\-_]*)(?<args>\\([^\\)]*\\))").as_str(),
    )?)
});
pub static PATTERN_REGEX: LazyLock<Result<Regex, DotfilesError>> =
    LazyLock::new(|| Ok(Regex::new(r"'[^']+'")?));
pub static STRING_OR_KEYWORD_REGEX: LazyLock<Result<Regex, DotfilesError>> =
    LazyLock::new(|| Ok(Regex::new("('[^']+')|(\\w[\\w\\d\\-_]*)")?));
pub static HEX_COLOUR_REGEX: LazyLock<Result<Regex, DotfilesError>> =
    LazyLock::new(|| Ok(Regex::new("#[\\w\\d]{6}")?));

pub fn parse_and_run_function(
    file_path: String,
    function_code_text: MatchedText,
    actual_text: MatchedText,
) -> Result<(), DotfilesError> {
    // Match the first function pattern // TODO Attempt to allow multiple functions on the same line
    let Some(function_match) = FUNCTION_REGEX.clone()?.captures(&function_code_text.text) else {
        return Err(DotfilesError::RegexMatchError {
            regex_str: FUNCTION_REGEX.clone()?.to_string(),
            hay: function_code_text.text,
        });
    };

    // Extract the groups
    let (_, [name, args]) = function_match.extract();

    // Remove the brackets around the arguments, then split them based on commas
    let args = args
        .trim_start_matches('(')
        .trim_end_matches(')')
        .split(',')
        .map(str::trim) // Make sure the remove excess whitespace on the arguments
        .collect::<Vec<&str>>();

    // Print the function and its arguments (This will help to track what is happening)
    println!("\t{name}({})", args.join(", "));

    // Run the function on the specified file
    run_function(name, &args, file_path, actual_text)?;

    Ok(())
}

pub fn run_function(
    name: &str,
    args: &[&str],
    file_path: String,
    text: MatchedText,
) -> Result<(), DotfilesError> {
    let pattern_regex = (*PATTERN_REGEX).clone()?;
    let string_or_keyword_regex = (*STRING_OR_KEYWORD_REGEX).clone()?;

    #[allow(clippy::single_match)]
    match name {
        // Requires: pattern, replace-string
        "replace" => {
            if args.len() == 2 {
                // First argument is a pattern
                if !pattern_regex.is_match(args[0]) {
                    return Err(DotfilesError::RegexMatchError {
                        regex_str: pattern_regex.to_string(),
                        hay: args[0].to_string(),
                    });
                }

                // Second argument is a keyword or string
                if !string_or_keyword_regex.is_match(args[1]) {
                    return Err(DotfilesError::RegexMatchError {
                        regex_str: string_or_keyword_regex.to_string(),
                        hay: args[1].to_string(),
                    });
                }

                // Run the function
                replace_fn(file_path, args, text)?;
            } else {
                // Incorrect number of arguments
                return Err(DotfilesError::FuncArgumentError {
                    name: name.to_string(),
                    needed: 2,
                    found: args.len(),
                    args: args.iter().map(ToString::to_string).collect(),
                });
            }
        }
        "replace-colour" => {
            if args.len() == 1 {
                // First argument is a keyword or string
                if !string_or_keyword_regex.is_match(args[0]) {
                    return Err(DotfilesError::RegexMatchError {
                        regex_str: string_or_keyword_regex.to_string(),
                        hay: args[0].to_string(),
                    });
                }

                // Run the function
                replace_fn(
                    file_path,
                    &[(HEX_COLOUR_REGEX.clone())?.as_str(), args[0]],
                    text,
                )?;
            }
        }
        _ => {}
    }

    Ok(())
}

// -------------------------------------------------------------------------------------------------------------------------------
// ---------------------------------- Code to perform each function call on the specified file -----------------------------------
// -------------------------------------------------------------------------------------------------------------------------------

fn replace_fn(file_path: String, args: &[&str], text: MatchedText) -> Result<(), DotfilesError> {
    // Remove the surrounding apostrophes from the pattern
    let replace_pattern_regex = Regex::new(args[0].trim_matches('\''))?;

    // Capture the text which matches the pattern, using the regex
    let Some(captures) = replace_pattern_regex.captures(&text.text) else {
        return Err(DotfilesError::RegexMatchError {
            regex_str: replace_pattern_regex.to_string(),
            hay: text.text,
        });
    };

    // Exit function if the captured text cannot be extracted into a Match variable
    let Some(text_match) = captures.get(0) else {
        return Err(DotfilesError::CaptureFail {
            captures: format!("{captures:?}"),
            index: 0,
        });
    };
    let text_match: MatchedText = text_match.into(); // Convert the Match variable into a MatchedText variable

    // Adjust the range to be with the respect to the whole file, and replace any keywords with their value
    let replace_text = MatchedText {
        range: (text.range.start + text_match.range.start)
            ..(text.range.start + text_match.range.end),
        text: parse_argument(args[1].trim_matches('\"'))?,
    };

    // TODO This is so that the file length and locations don't change (Should fix this issue at some point)
    if text_match.range.len() != replace_text.text.len() {
        return Err(DotfilesError::ReplaceTextDifferentLength {
            text_to_replace: text_match.text,
            replace_text: replace_text.text,
        });
    }

    println!(
        "\t\t{}:  {}  -->  {}",
        text.text.trim(),
        text_match.text,
        replace_text.text
    );

    // Replace the text in the file
    write_to_file(file_path, replace_text)?;

    Ok(())
}
