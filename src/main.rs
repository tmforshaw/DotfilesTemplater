#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

use crate::errors::DotfilesError;

pub(crate) mod arguments;
mod config;
mod errors;
pub(crate) mod file;
pub(crate) mod functions;

fn main() -> Result<(), DotfilesError> {
    // Modify the files accordinig to the template text, then print any errors that occur
    match file::modify_files() {
        Err(e) => {
            eprintln!("{e}");
            Err(e)
        }
        _ => Ok(()),
    }
}
