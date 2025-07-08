use crate::config::CONFIG;

pub(crate) fn parse_argument(arg: &str) -> String {
    // Replace certain keywords with particular values from the dotfiles_templater config
    match arg {
        "background_colour" => (*CONFIG).clone().unwrap().background_colour,
        _ => arg.to_string(),
    }
}
