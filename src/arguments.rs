use crate::config::CONFIG;

pub(crate) fn parse_argument(arg: &str) -> String {
    match arg {
        "background_colour" => (*CONFIG).clone().unwrap().background_colour,
        _ => arg.to_string(),
    }
}
