use std::fs::File;
use std::io::prelude::*;

use regex::Regex;

use crate::config::{CONFIG, FileConfig};
use crate::errors::DotfilesError;
use crate::functions::parse_config_functions;

pub fn open_file<S: AsRef<str>>(path: S) -> String {
    let mut config_file = File::open(path.as_ref()).unwrap();
    let mut contents = String::new();

    config_file.read_to_string(&mut contents).unwrap();

    contents.trim().replace(" ", "").to_string()
}

pub fn modify_files() -> Result<(), DotfilesError> {
    for file_config in Into::<Vec<FileConfig>>::into(CONFIG.files.clone()).iter() {
        let file = open_file(file_config.file.clone());

        // Find the parts which need to be replaced
        let marker_regex_string = file_config.comment_char.to_string().repeat(3);
        let marker_regex =
            Regex::new(format!("(?m)(?<before>^.*){marker_regex_string}(?<after>.*)$").as_str())?;

        for (_, [before, after]) in marker_regex
            .captures_iter(file.as_str())
            .map(|c| c.extract())
        {
            println!("{before}\n{after}");

            parse_config_functions(before, after)?;
        }
    }

    Ok(())
}
