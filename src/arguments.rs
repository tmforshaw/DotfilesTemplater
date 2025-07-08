use crate::config::CONFIG;

pub fn parse_argument(arg: &str) -> String {
    // Replace certain keywords with particular values from the dotfiles_templater config
    match arg {
        "background_colour" => CONFIG.background_colour.clone(),
        _ => arg.to_string(),
    }
}
