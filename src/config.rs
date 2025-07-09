use serde::Deserialize;
use std::{collections::HashMap, sync::LazyLock};

use crate::{errors::DotfilesError, file::open_file};

const CONFIG_FILE_SUB_PATH: &str = "dotfile-templater/config.toml";

pub static XDG_CONFIG_PATH: LazyLock<String> = LazyLock::new(|| {
    std::env::vars().collect::<HashMap<String, String>>()["XDG_CONFIG_HOME"].clone()
});

static CONFIG_FILE_PATH: LazyLock<String> =
    LazyLock::new(|| format!("{}/{CONFIG_FILE_SUB_PATH}", *XDG_CONFIG_PATH));

pub const FUNCTION_CHAR: char = '@';

const fn get_default_marker_repetition_num() -> usize {
    3
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub theme: String,
    #[serde(default = "get_default_marker_repetition_num")]
    pub marker_repetition_num: usize,
    pub files: Vec<FileConfig>,

    #[serde(default)]
    pub themes: Vec<HashMap<String, String>>,
}

impl Config {
    pub fn get_theme_hashmap(&self) -> HashMap<String, HashMap<String, String>> {
        self.themes
            .iter()
            .map(|theme| (theme["name"].clone(), theme.clone()))
            .collect()
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct FileConfig {
    pub file: String,
    pub marker_char: String,
}

pub static CONFIG: LazyLock<Config> = LazyLock::new(parse_config);

fn parse_config() -> Config {
    // Read the TOML config into a Config struct (Exit the program if the config cannot be parsed)
    match open_file(CONFIG_FILE_PATH.as_str()) {
        Ok(config) => match toml::from_str(config.as_str()) {
            Ok(config) => return config,
            Err(e) => {
                eprintln!("{}", Into::<DotfilesError>::into(e));
            }
        },
        Err(e) => {
            eprintln!("{e}");
        }
    }

    std::process::exit(0x1000)
}
