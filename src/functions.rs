use lazy_static::lazy_static;
use regex::Regex;

use crate::config::FUNCTION_CHAR;
use crate::errors::DotfilesError;

lazy_static! {
    pub static ref FUNCTION_REGEX: Result<Regex, DotfilesError> = {
        let regex = Regex::new(
            format!("{FUNCTION_CHAR}(?<name>\\w[\\w\\d]*)(?<args>\\([^\\)]*\\))").as_str(),
        )?;
        Ok(regex)
    };
    pub static ref PATTERN_REGEX: Result<Regex, DotfilesError> = {
        let regex = Regex::new(r"'[^']+'")?;
        Ok(regex)
    };
    pub static ref STRING_OR_KEYWORD_REGEX: Result<Regex, DotfilesError> = {
        let regex = Regex::new("(('|\")[^'\"]+('|\"))|(\\w[\\w\\d\\-_]*)")?;
        Ok(regex)
    };
}

pub fn parse_config_functions(before: &str, after: &str) -> Result<(), DotfilesError> {
    for (_, [name, args]) in (*FUNCTION_REGEX)
        .clone()?
        .captures_iter(after)
        .map(|c| c.extract())
    {
        let args = args
            .trim_start_matches('(')
            .trim_end_matches(')')
            .split(',')
            .collect::<Vec<&str>>();

        println!("Name: {name}\tArgs: {args:?}");

        parse_function(name, args, before)?;
    }

    Ok(())
}

fn parse_function(name: &str, args: Vec<&str>, text: &str) -> Result<(), DotfilesError> {
    let pattern_regex = (*PATTERN_REGEX).clone()?;
    let string_or_keyword_regex = (*STRING_OR_KEYWORD_REGEX).clone()?;

    match name {
        // Requires pattern, replace-string
        "replace" => {
            if args.len() == 2 {
                if !pattern_regex.is_match(args[0]) {
                    eprintln!("Argument 0 is not a pattern");
                    return Err(DotfilesError::RegexMatchError {
                        regex_str: pattern_regex.to_string(),
                        hay: args[0].to_string(),
                    });
                }

                if !string_or_keyword_regex.is_match(args[1]) {
                    eprintln!("Argument 1 is not a string/keyword");
                    return Err(DotfilesError::RegexMatchError {
                        regex_str: string_or_keyword_regex.to_string(),
                        hay: args[1].to_string(),
                    });
                }
            } else {
                eprintln!("'replace' function requires 2 arguments");
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
