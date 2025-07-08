use std::fmt;

use lazy_static::lazy_static;
use serde::Deserialize;

use crate::file::open_file;
// use toml::Table;

static CONFIG_PATH: &str = "/home/tmforshaw/.config/dotfile-templater/config.toml";
pub const FUNCTION_CHAR: char = '@';

#[derive(Deserialize, Debug)]
pub struct Config {
    pub background_colour: String,
    pub files: FileConfigs,
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Config {{
    background_colour: {:?},
    files: {:?}
}}",
            self.background_colour, self.files
        )
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct FileConfig {
    pub file: String,
    pub comment_char: char,
}

impl fmt::Display for FileConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{file: {:?}, comment_char: {:?}}}",
            self.file, self.comment_char
        )
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct FileConfigs {
    pub configs: Vec<FileConfig>,
}

impl fmt::Display for FileConfigs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut message = String::new();
        message += "[";

        for file_config in self.configs.iter() {
            message += format!("{file_config}").as_str();
        }
        message += "]";

        write!(f, "{message}")
    }
}

impl From<FileConfigs> for Vec<FileConfig> {
    fn from(value: FileConfigs) -> Self {
        value.configs
    }
}

lazy_static! {
    #[derive(Debug)]
    pub static ref CONFIG: Config = parse_config();
}

fn parse_config() -> Config {
    let config = toml::from_str(open_file(CONFIG_PATH).as_str()).unwrap();

    println!("{config}");

    config
}
