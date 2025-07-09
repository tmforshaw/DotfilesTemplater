use regex::Regex;
use std::sync::LazyLock;

use crate::arguments::parse_argument;
use crate::config::FUNCTION_CHAR;
use crate::errors::DotfilesError;
use crate::file::{MatchedText, open_file, write_to_file};

pub static FUNCTION_REGEX: LazyLock<Result<Regex, DotfilesError>> = LazyLock::new(|| {
    Ok(Regex::new(
        format!("{FUNCTION_CHAR}(?<name>\\w[\\w\\-]*)(?<args>(?:\\\\.|[^)])+\\))").as_str(),
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
    actual_text: &MatchedText,
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
    text: &MatchedText,
) -> Result<(), DotfilesError> {
    #[allow(clippy::single_match)]
    match name {
        // Requires: pattern, replace-string
        "replace" => {
            if args.len() == 2 {
                matches_pattern(args[0])?; // First argument is a pattern
                matches_keyword_or_string(args[1])?; // Second argument is a keyword or string

                // Run the function
                replace_fn(file_path, args, text)?;
            } else {
                // Incorrect number of arguments
                return Err(DotfilesError::FuncArgumentError {
                    name: name.to_string(),
                    needed: 2,
                    args: args.iter().map(ToString::to_string).collect(),
                });
            }
        }
        "replace-col" => {
            if args.len() == 1 {
                // First argument is a keyword or string
                matches_keyword_or_string(args[0])?;

                // Run the function
                replace_fn(
                    file_path,
                    &[(HEX_COLOUR_REGEX.clone())?.as_str(), args[0]],
                    text,
                )?;
            } else {
                // Incorrect number of arguments
                return Err(DotfilesError::FuncArgumentError {
                    name: name.to_string(),
                    needed: 1,
                    args: args.iter().map(ToString::to_string).collect(),
                });
            }
        }
        // Replace function which also puts a pattern onto the text which is going to replace, and applies that same pattern to the text_to_replace (so they're the same length)
        "replace-pattern" => {
            if args.len() == 3 {
                matches_pattern(args[0])?; // First argument is a pattern
                matches_keyword_or_string(args[1])?; // Second argument is a keyword or string
                matches_pattern(args[2])?; // Third argument is a pattern

                // Run the function
                replace_fn(file_path, args, text)?;
            } else {
                // Incorrect number of arguments
                return Err(DotfilesError::FuncArgumentError {
                    name: name.to_string(),
                    needed: 3,
                    args: args.iter().map(ToString::to_string).collect(),
                });
            }
        }
        f => {
            eprintln!("Function '{f}' does not exist");
        }
    }

    Ok(())
}

// -------------------------------------------------------------------------------------------------------------------------------
// ---------------------------------- Code to perform each function call on the specified file -----------------------------------
// -------------------------------------------------------------------------------------------------------------------------------

fn replace_fn(file_path: String, args: &[&str], text: &MatchedText) -> Result<(), DotfilesError> {
    // Parse the 2nd argument, to convert keywords into strings
    let mut keyword_as_string = parse_argument(args[1].trim_matches('\"'))?;

    // Remove the surrounding apostrophes from the pattern, then turn it into a Regex
    let replace_pattern_regex = Regex::new(args[0].trim_matches('\''))?;

    // Match the text with this pattern
    let mut text_to_replace = get_single_match(&replace_pattern_regex, text.clone())?;

    // Check if there is a pattern to apply to text_to_replace and the keyword_as_string
    if let Some(&pattern_str_2) = args.get(2) {
        // Check that this third argument is a pattern
        matches_pattern(pattern_str_2)?;

        // Remove the surrounding apostrophes from the pattern, and turn it into a Regex
        let keyword_pattern_regex = Regex::new(pattern_str_2.trim_matches('\''))?;

        // Shrink the text_to_replace to fit the new pattern
        text_to_replace = get_single_match(&keyword_pattern_regex, text_to_replace.clone())?;

        let keyword_matched_text = MatchedText {
            range: 0..1, // Not used
            text: keyword_as_string,
        };
        keyword_as_string = get_single_match(&keyword_pattern_regex, keyword_matched_text)?.text;
    }

    // TODO This is so that the file length and locations don't change (Should fix this issue at some point)
    if text_to_replace.range.len() != keyword_as_string.len() {
        return Err(DotfilesError::ReplaceTextDifferentLength {
            text_to_replace: text_to_replace.text,
            replace_text: keyword_as_string,
        });
    }

    if text_to_replace.text != keyword_as_string {
        println!(
            "\t\t'{}'\n\t\t{}  -->  {}",
            text.text.trim(),
            text_to_replace.text,
            keyword_as_string
        );

        let replacement_text = MatchedText {
            range: text_to_replace.range,
            text: keyword_as_string,
        };

        // Replace the text in the file
        write_to_file(file_path, replacement_text)?;
    }
    println!();

    Ok(())
}

fn matches_pattern(arg: &str) -> Result<(), DotfilesError> {
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

fn matches_keyword_or_string(arg: &str) -> Result<(), DotfilesError> {
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

fn get_single_match(regex: &Regex, text: MatchedText) -> Result<MatchedText, DotfilesError> {
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

#[allow(dead_code)]
fn test_fn_print_chars(
    file_path: String,
    range: std::ops::Range<usize>,
) -> Result<(), DotfilesError> {
    let f = open_file(file_path)?;

    println!("{:?}", &f.chars().collect::<Vec<_>>()[range]);

    Ok(())
}
