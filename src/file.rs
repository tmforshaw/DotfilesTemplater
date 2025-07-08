use std::fmt::Debug;
use std::fs::File;
use std::io::prelude::*;
use std::ops::Range;

use regex::Regex;

use crate::config::{CONFIG, FileConfig};
use crate::errors::DotfilesError;
use crate::functions::parse_and_run_function;

#[derive(Debug, Clone)]
pub(crate) struct MatchedText {
    pub(crate) range: Range<usize>,
    pub(crate) text: String,
}

impl<'a> From<regex::Match<'a>> for MatchedText {
    fn from(value: regex::Match) -> Self {
        Self {
            range: value.range(),
            text: value.as_str().to_string(),
        }
    }
}

pub(crate) fn open_file<S: AsRef<str>>(path: S) -> String {
    let mut config_file = File::open(path.as_ref()).unwrap();
    let mut contents = String::new();

    config_file.read_to_string(&mut contents).unwrap();
    contents
}

pub(crate) fn write_to_file<S: AsRef<str>>(
    path: S,
    replace_text: MatchedText,
) -> Result<(), DotfilesError> {
    let mut config_file = File::open(path.as_ref()).unwrap();
    let mut contents = String::new();

    config_file.read_to_string(&mut contents).unwrap();

    println!(
        "{:?}",
        &contents.chars().collect::<Vec<_>>()[replace_text.range.clone()]
    );

    // let mut chars = contents.chars().collect::<Vec<_>>();
    // chars[replace_text.range] = &replace_text.text.chars().collect::<Vec<_>>();

    contents.replace_range(replace_text.range, replace_text.text.as_str());
    println!("{contents}");

    Ok(())
}

pub(crate) fn modify_files() -> Result<(), DotfilesError> {
    for file_config in Into::<Vec<FileConfig>>::into((*CONFIG).clone()?.files.clone()).iter() {
        let file = open_file(file_config.file.clone());

        // Find the parts which need to be replaced
        let marker_regex_string = file_config.comment_char.to_string().repeat(3);
        let marker_regex =
            Regex::new(format!("(?m)(?<before>^.*){marker_regex_string}(?<after>.*)$").as_str())?;

        for captures in marker_regex.captures_iter(&file) {
            let Some(before) = captures.get(1) else {
                return Err(DotfilesError::CaptureFail {
                    captures: format!("{captures:?}"),
                    index: 1,
                });
            };

            let Some(after) = captures.get(2) else {
                return Err(DotfilesError::CaptureFail {
                    captures: format!("{captures:?}"),
                    index: 2,
                });
            };

            println!("{before:?}\t\t{after:?}");

            parse_and_run_function(file_config.file.to_string(), after.into(), before.into())?;
        }
    }

    Ok(())
}
