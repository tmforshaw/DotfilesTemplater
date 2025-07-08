use lazy_static::lazy_static;
use serde::Deserialize;

use crate::{errors::DotfilesError, file::open_file};

static CONFIG_PATH: &str = "/home/tmforshaw/.config/dotfile-templater/config.toml";
pub(crate) const FUNCTION_CHAR: char = '@';

#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone)]
pub(crate) struct Config {
    pub(crate) background_colour: String,
    pub(crate) files: Vec<FileConfig>,
}

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct FileConfig {
    pub(crate) file: String,
    pub(crate) comment_char: char,
}

lazy_static! {
    #[derive(Debug)]
    pub(crate) static ref CONFIG: Result<Config, DotfilesError> = parse_config();
}

fn parse_config() -> Result<Config, DotfilesError> {
    let config = toml::from_str(open_file(CONFIG_PATH).as_str())?;

    println!("{config:#?}");

    Ok(config)
}
