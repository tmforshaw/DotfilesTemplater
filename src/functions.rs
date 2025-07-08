use lazy_static::lazy_static;
use regex::Regex;

use crate::arguments::parse_argument;
use crate::config::FUNCTION_CHAR;
use crate::errors::DotfilesError;
use crate::file::{MatchedText, open_file};

lazy_static! {
    pub(crate) static ref FUNCTION_REGEX: Result<Regex, DotfilesError> = {
        let regex = Regex::new(
            format!("{FUNCTION_CHAR}(?<name>\\w[\\w\\d]*)(?<args>\\([^\\)]*\\))").as_str(),
        )?;
        Ok(regex)
    };
    pub(crate) static ref PATTERN_REGEX: Result<Regex, DotfilesError> = {
        let regex = Regex::new(r"'[^']+'")?;
        Ok(regex)
    };
    pub(crate) static ref STRING_OR_KEYWORD_REGEX: Result<Regex, DotfilesError> = {
        let regex = Regex::new("('[^']+')|(\\w[\\w\\d\\-_]*)")?;
        Ok(regex)
    };
}

pub(crate) fn parse_and_run_function(
    file_path: String,
    function_code_text: MatchedText,
    actual_text: MatchedText,
) -> Result<(), DotfilesError> {
    let Some(function_match) = (*FUNCTION_REGEX)
        .clone()?
        .captures(&function_code_text.text)
    else {
        return Err(DotfilesError::RegexMatchError {
            regex_str: (*FUNCTION_REGEX).clone().unwrap().to_string(),
            hay: function_code_text.text,
        });
    };

    let (_, [name, args]) = function_match.extract();

    let args = args
        .trim_start_matches('(')
        .trim_end_matches(')')
        .split(',')
        .collect::<Vec<&str>>();

    println!("Name: {name}\tArgs: {args:?}");

    check_function_args(name, args.clone())?;

    run_function(name, args, file_path, actual_text)?;

    Ok(())
}

fn check_function_args(name: &str, args: Vec<&str>) -> Result<(), DotfilesError> {
    let pattern_regex = (*PATTERN_REGEX).clone()?;
    let string_or_keyword_regex = (*STRING_OR_KEYWORD_REGEX).clone()?;

    match name {
        // Requires pattern, replace-string
        "replace" => {
            if args.len() == 2 {
                if !pattern_regex.is_match(args[0]) {
                    return Err(DotfilesError::RegexMatchError {
                        regex_str: pattern_regex.to_string(),
                        hay: args[0].to_string(),
                    });
                }

                if !string_or_keyword_regex.is_match(args[1]) {
                    return Err(DotfilesError::RegexMatchError {
                        regex_str: string_or_keyword_regex.to_string(),
                        hay: args[1].to_string(),
                    });
                }
            } else {
                return Err(DotfilesError::FuncArgumentError {
                    name: name.to_string(),
                    needed: 2,
                    found: args.len(),
                    args: args.iter().map(ToString::to_string).collect(),
                });
            }
        }
        "other" => {}
        _ => {}
    }

    Ok(())
}

// This is called after parse_function has already checked if the args are correct
pub(crate) fn run_function(
    name: &str,
    args: Vec<&str>,
    file_path: String,
    text: MatchedText,
) -> Result<(), DotfilesError> {
    let file_text = open_file(file_path);

    // println!("{file_text}");
    // println!("{text:?}");

    // println!("{:?}", &file_text.chars().collect::<Vec<_>>()[text.range]);

    match name {
        "replace" => {
            let replace_regex = Regex::new(args[0].trim_matches('\''))?;

            let Some(captures) = replace_regex.captures(&text.text) else {
                return Err(DotfilesError::RegexMatchError {
                    regex_str: replace_regex.to_string(),
                    hay: text.text,
                });
            };

            let Some(text_match) = captures.get(0) else {
                return Err(DotfilesError::CaptureFail {
                    captures: format!("{captures:?}"),
                    index: 0,
                });
            };

            let text_match: MatchedText = text_match.into();

            let range_to_replace = (text.range.start + text_match.range.start)
                ..(text.range.start + text_match.range.end);

            println!("{text_match:?}");
            println!(
                "{:?}",
                &file_text.chars().collect::<Vec<_>>()[range_to_replace]
            );

            let replace_text = parse_argument(args[1]);
            println!("{replace_text}");

            // TODO This is so that the file length and locations don't change (Should fix this issue at some point)
            if text_match.range.len() != replace_text.len() {
                return Err(DotfilesError::ReplaceTextDifferentLength {
                    text_to_replace: text_match.text,
                    replace_text,
                });
            }
        }
        "other" => {}
        _ => {}
    }

    Ok(())
}
