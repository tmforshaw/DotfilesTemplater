#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

// TODO Need to allow colour changing without # for HyprLand

mod arguments;
mod config;
mod errors;
mod file;
mod functions;

fn main() {
    // Modify the files accordinig to the template text, then print any errors that occur
    if let Err(e) = file::modify_files() {
        eprintln!("{e}");
    }
}
