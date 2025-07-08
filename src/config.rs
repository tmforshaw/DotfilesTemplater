use serde::Deserialize;
use std::{collections::HashMap, sync::LazyLock};

use crate::{errors::DotfilesError, file::open_file};

// TODO Figure out the config path from XDG_CONFIG_HOME

static CONFIG_PATH: &str = "/home/tmforshaw/.config/dotfile-templater/config.toml";
pub const FUNCTION_CHAR: char = '@';

#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub(crate) theme: String,
    pub(crate) files: Vec<FileConfig>,

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
    pub(crate) file: String,
    pub(crate) comment_char: char,
}

pub static CONFIG: LazyLock<Config> = LazyLock::new(parse_config);

fn parse_config() -> Config {
    // Read the TOML config into a Config struct (Exit the program if the config cannot be parsed)
    match open_file(CONFIG_PATH) {
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
