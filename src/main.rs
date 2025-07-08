use crate::errors::DotfilesError;

mod config;
mod errors;
pub mod file;
pub mod functions;

fn main() -> Result<(), DotfilesError> {
    match file::modify_files() {
        Err(e) => {
            eprintln!("{e}");
            Err(e)
        }
        Ok(_) => Ok(()),
    }
}
