use crate::{config::CONFIG, errors::DotfilesError};

pub fn parse_argument(arg: &str) -> Result<String, DotfilesError> {
    // Get the current theme
    let themes = CONFIG.get_theme_hashmap();
    let Some(current_theme) = themes.get(&CONFIG.theme) else {
        return Err(DotfilesError::ThemeNotFound {
            name: CONFIG.theme.clone(),
            themes: CONFIG
                .themes
                .iter()
                .map(|theme| theme["name"].clone())
                .collect(),
        });
    };

    let Some(value) = current_theme.get(arg) else {
        return Err(DotfilesError::ArgNotFound {
            arg: arg.to_string(),
            theme_hashmap: current_theme.clone(),
        });
    };

    Ok(value.to_string())
}
