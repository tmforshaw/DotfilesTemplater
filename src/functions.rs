use regex::Regex;

use crate::arguments::parse_argument;
use crate::errors::DotfilesError;
use crate::file::{MatchedText, write_to_file};
use crate::regex::{
    FUNCTION_REGEX, HEX_COLOUR_REGEX, get_single_match, matches_keyword_or_string, matches_pattern,
};

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
                    &[HEX_COLOUR_REGEX.clone()?.as_str(), args[0]],
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
        // Replace function which also puts a pattern onto the text which is going to replace, and applies that same pattern to the text_to_replace (so they're the same length)
        "replace-pattern-col" => {
            if args.len() == 2 {
                matches_keyword_or_string(args[0])?; // First argument is a keyword or string
                matches_pattern(args[1])?; // Second argument is a pattern

                // Run the function
                replace_fn(
                    file_path,
                    &[HEX_COLOUR_REGEX.clone()?.as_str(), args[0], args[1]],
                    text,
                )?;
            } else {
                // Incorrect number of arguments
                return Err(DotfilesError::FuncArgumentError {
                    name: name.to_string(),
                    needed: 2,
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
