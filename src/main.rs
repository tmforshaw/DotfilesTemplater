#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

// TODO Need to allow colour changing without # for HyprLand
// TODO Add Replace function which applies pattern to the keyword/value

mod arguments;
mod config;
mod errors;
mod file;
mod functions;
mod regex;

fn main() {
    // Modify the files accordinig to the template text, then print any errors that occur
    if let Err(e) = file::modify_files() {
        eprintln!("{e}");
    }
}
