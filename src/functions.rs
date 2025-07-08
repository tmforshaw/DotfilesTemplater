use lazy_static::lazy_static;
use regex::Regex;

use crate::config::FUNCTION_CHAR;
use crate::errors::DotfilesError;
use crate::file::MatchedText;

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
        let regex = Regex::new("(('|\")[^'\"]+('|\"))|(\\w[\\w\\d\\-_]*)")?;
        Ok(regex)
    };
}

pub(crate) fn parse_config_functions(
    before: MatchedText,
    after: MatchedText,
) -> Result<(), DotfilesError> {
    for (_, [name, args]) in (*FUNCTION_REGEX)
        .clone()?
        .captures_iter(&after.text)
        .map(|c| c.extract())
    {
        let args = args
            .trim_start_matches('(')
            .trim_end_matches(')')
            .split(',')
            .collect::<Vec<&str>>();

        println!("Name: {name}\tArgs: {args:?}");

        parse_function(name, args, before.clone())?;
    }

    Ok(())
}

fn parse_function(name: &str, args: Vec<&str>, _text: MatchedText) -> Result<(), DotfilesError> {
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

                // Arguments have the correct form (as defined by the regex)
            } else {
                return Err(DotfilesError::ArgumentError {
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
