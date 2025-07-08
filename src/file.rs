use regex::Regex;

use std::fs::File;
use std::io::prelude::*;

use crate::config::CONFIG;

pub fn open_file<S: AsRef<str>>(path: S) -> String {
    let mut config_file = File::open(path.as_ref()).unwrap();
    let mut contents = String::new();

    config_file.read_to_string(&mut contents).unwrap();

    contents
        .trim()
        // .replace("\n", "")
        .replace(" ", "")
        .to_string()
}

pub fn modify_files() {
    for file_config in CONFIG.files.iter() {
        let file = open_file(file_config.file.clone());
        // println!("{file}");

        // Find the parts which need to be replaced
        let marker_regex_string = file_config.comment_char.to_string().repeat(3);
        let marker_regex =
            Regex::new(format!("(?m)(?<before>^.*){marker_regex_string}(?<after>.*)$").as_str())
                .unwrap();

        // println!("{marker_regex:?}");

        for (_, [before, after]) in marker_regex
            .captures_iter(file.as_str())
            .map(|c| c.extract())
        {
            println!("{before}\n{after}");

            run_config_functions(before, after);
        }
    }
}

pub fn run_config_functions(before: &str, after: &str) {
    todo!()
}
