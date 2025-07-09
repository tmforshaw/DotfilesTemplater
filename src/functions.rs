use regex::Regex;

use crate::arguments::parse_argument;
use crate::errors::DotfilesError;
use crate::file::{MatchedText, write_to_file};
use crate::regex::{
    FUNCTION_REGEX, HEX_COLOUR_REGEX, get_nth_match, get_single_match, matches_keyword_or_string,
    matches_pattern,
};

pub fn parse_and_run_function(
    file_path: &str,
    function_code_text: &MatchedText,
    actual_text: &MatchedText,
) -> Result<(), DotfilesError> {
    // Match the first function pattern // TODO Attempt to allow multiple functions on the same line
    let functions = FUNCTION_REGEX
        .clone()?
        .captures_iter(&function_code_text.text)
        .map(|function_captures| {
            // Extract the groups
            let (_, [name, args]) = function_captures.extract();

            // Remove the brackets around the arguments, then split them based on commas
            let args = args
                .trim_start_matches('(')
                .trim_end_matches(')')
                .split(',')
                .map(str::trim) // Make sure the remove excess whitespace on the arguments
                .collect::<Vec<&str>>();

            // Print the function and its arguments (This will help to track what is happening)
            println!("\t{name}({})", args.join(", "));

            (name.to_string(), args)
        })
        .collect::<Vec<_>>();

    // println!("\t{functions:?}");

    // Run each function on the specified file
    for (i, (name, args)) in functions.iter().enumerate() {
        run_function(name, args, file_path.to_string(), actual_text, i)?;
    }

    Ok(())
}

pub fn run_function(
    name: &str,
    args: &[&str],
    file_path: String,
    text: &MatchedText,
    index_to_match: usize,
) -> Result<(), DotfilesError> {
    match name {
        // Requires: pattern, replace-string
        "replace" => {
            if args.len() == 2 {
                matches_pattern(args[0])?; // First argument is a pattern
                matches_keyword_or_string(args[1])?; // Second argument is a keyword or string

                // Run the function
                replace_fn(file_path, args, text, index_to_match)?;
            } else {
                // Incorrect number of arguments
                return Err(DotfilesError::FuncArgumentError {
                    name: name.to_string(),
                    needed: 2,
                    args: args.iter().map(ToString::to_string).collect(),
                });
            }
        }
        // R
        "replace-col" => {
            if args.len() == 1 {
                matches_keyword_or_string(args[0])?; // First argument is a keyword or string

                // Run the function
                replace_fn(
                    file_path,
                    &[HEX_COLOUR_REGEX.clone()?.as_str(), args[0]],
                    text,
                    index_to_match,
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
                replace_fn(file_path, args, text, index_to_match)?;
            } else {
                // Incorrect number of arguments
                return Err(DotfilesError::FuncArgumentError {
                    name: name.to_string(),
                    needed: 3,
                    args: args.iter().map(ToString::to_string).collect(),
                });
            }
        }
        // Replace function which also puts a pattern onto the text which is going to replace, and applies that same pattern to the text_to_replace (so they're the same length), Also the initial pattern to match is the colour pattern
        "replace-pattern-col" => {
            if args.len() == 2 {
                matches_keyword_or_string(args[0])?; // First argument is a keyword or string
                matches_pattern(args[1])?; // Second argument is a pattern

                // Run the function
                replace_fn(
                    file_path,
                    &[HEX_COLOUR_REGEX.clone()?.as_str(), args[0], args[1]],
                    text,
                    index_to_match,
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

fn replace_fn(
    file_path: String,
    args: &[&str],
    text: &MatchedText,
    index_to_match: usize,
) -> Result<(), DotfilesError> {
    // Parse the 2nd argument, to convert keywords into strings
    let mut keyword_as_string = parse_argument(args[1].trim_matches('\"'))?;

    // Remove the surrounding apostrophes from the pattern, then turn it into a Regex
    let replace_pattern_regex = Regex::new(args[0].trim_matches('\''))?;

    // Match the text with this pattern (Choosing the nth match)
    let mut text_to_replace = get_nth_match(&replace_pattern_regex, text.clone(), index_to_match)?;

    // Check if there is a pattern to apply to text_to_replace and the keyword_as_string
    if let Some(&pattern_str_2) = args.get(2) {
        // Check that this third argument is a pattern
        matches_pattern(pattern_str_2)?;

        // Remove the surrounding apostrophes from the pattern, and turn it into a Regex
        let keyword_pattern_regex = Regex::new(pattern_str_2.trim_matches('\''))?;

        // Shrink the text_to_replace to fit the new pattern
        text_to_replace = get_single_match(&keyword_pattern_regex, text_to_replace.clone())?;

        // Perform the pattern matching on the keyword as well, giving a dummy range so the function signature is correct
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

    // Only replace if the text has changed
    if text_to_replace.text != keyword_as_string {
        println!(
            "\t\t'{}'\n\t\t{}  -->  {}",
            text.text.trim(),
            text_to_replace.text,
            keyword_as_string
        );

        // Replace the text in the file
        write_to_file(
            file_path,
            MatchedText {
                range: text_to_replace.range,
                text: keyword_as_string,
            },
        )?;
    }
    println!();

    Ok(())
}
