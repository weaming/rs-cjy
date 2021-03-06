#[macro_use]
extern crate serde_json;

use std::env;
use std::fs::File;
use std::io::{Error, ErrorKind, Read, Write};

pub mod data_struct;
pub mod error;
pub mod io_csv;
pub mod io_json;
pub mod io_yaml;
pub use self::data_struct::*;

pub fn read_file(path: &str) -> Result<String, Error> {
    let mut f = File::open(path)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn write_file(path: &str, text: &str) -> Result<(), Error> {
    let mut file = File::create(path)?;
    file.write_all(text.as_bytes())?;
    Ok(())
}

/// errors can be created from strings
fn create_io_error(msg: &str) -> Error {
    Error::new(ErrorKind::Other, msg)
}

pub fn has_env(name: &str) -> bool {
    match env::var(name) {
        Ok(val) => {
            if val.len() > 0 {
                return true;
            } else {
                return false;
            }
        }
        Err(_) => return false,
    }
}

pub fn is_debug() -> bool {
    has_env("DEBUG")
}
