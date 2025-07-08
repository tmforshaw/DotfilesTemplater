use std::fmt::Debug;
use std::fs::File;
use std::io::prelude::*;
use std::ops::Range;
use std::path::Path;

use regex::Regex;

use crate::config::{CONFIG, FileConfig, XDG_CONFIG_PATH};
use crate::errors::DotfilesError;
use crate::functions::parse_and_run_function;

#[derive(Debug, Clone)]
pub struct MatchedText {
    pub range: Range<usize>,
    pub text: String,
}

impl From<regex::Match<'_>> for MatchedText {
    fn from(value: regex::Match) -> Self {
        Self {
            range: value.range(),
            text: value.as_str().to_string(),
        }
    }
}

pub fn open_file<S: AsRef<str>>(path: S) -> Result<String, DotfilesError> {
    // Open file and copy contents
    let mut config_file = File::open(path.as_ref())?;
    let mut contents = String::new();
    config_file.read_to_string(&mut contents)?;

    Ok(contents)
}

pub fn write_to_file<S: AsRef<str>>(
    path: S,
    replace_text: MatchedText,
) -> Result<(), DotfilesError> {
    // Copy the file before modifications
    let mut config_file = File::open(path.as_ref())?;
    let mut contents = String::new();
    config_file.read_to_string(&mut contents)?;

    // Replace the region with the new text
    contents.replace_range(replace_text.range, replace_text.text.as_str());

    // Write the changes to the file
    let mut config_file = File::create(path.as_ref())?;
    config_file.write_all(contents.as_bytes())?;

    Ok(())
}

pub fn modify_files() -> Result<(), DotfilesError> {
    for file_config in &Into::<Vec<FileConfig>>::into(CONFIG.files.clone()) {
        // Allow file_path to be absolute, or relative to .config
        let path_str = {
            let p = Path::new(file_config.file.as_str());
            if p.is_absolute() {
                p.display().to_string()
            } else {
                format!("{}/{}", *XDG_CONFIG_PATH, p.display())
            }
        };

        println!("{path_str}");

        let file = open_file(path_str.as_str())?;

        // Find the parts which need to be replaced
        let marker_regex_string = file_config
            .comment_char
            .to_string()
            .repeat(CONFIG.marker_repetition_num);
        let marker_regex = Regex::new(format!("(?m)(^.*){marker_regex_string}(.*)$").as_str())?;

        // Find the lines which have the marker on them, and split the line into actual code and template code
        for captures in marker_regex.captures_iter(&file) {
            let Some(actual_text) = captures.get(1) else {
                return Err(DotfilesError::CaptureFail {
                    captures: format!("{captures:?}"),
                    index: 1,
                });
            };

            let Some(template_text) = captures.get(2) else {
                return Err(DotfilesError::CaptureFail {
                    captures: format!("{captures:?}"),
                    index: 2,
                });
            };

            // Parse the template code, and modify the actual_text
            parse_and_run_function(path_str.clone(), template_text.into(), actual_text.into())?;
        }
    }

    Ok(())
}
