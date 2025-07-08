use lazy_static::lazy_static;
use serde::Deserialize;

use crate::file::open_file;

static CONFIG_PATH: &str = "/home/tmforshaw/.config/dotfile-templater/config.toml";
pub const FUNCTION_CHAR: char = '@';

#[derive(Deserialize, Debug)]
pub struct Config {
    pub _background_colour: String,
    pub files: Vec<FileConfig>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct FileConfig {
    pub file: String,
    pub comment_char: char,
}

lazy_static! {
    #[derive(Debug)]
    pub static ref CONFIG: Config = parse_config();
}

fn parse_config() -> Config {
    let config = toml::from_str(open_file(CONFIG_PATH).as_str()).unwrap();

    println!("{config:#?}");

    config
}
