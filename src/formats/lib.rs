use std::fs::File;
use std::io::{Error, ErrorKind, Read};

pub mod data_struct;
pub mod error;
pub mod io_csv;
pub mod io_json;
pub mod io_yaml;

pub use self::data_struct::{Row, Tabular};

pub fn read_file(path: &str) -> Result<String, Error> {
    let mut f = File::open(path)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    Ok(contents)
}

fn create_io_error(msg: &str) -> Error {
    // errors can be created from strings
    Error::new(ErrorKind::Other, msg)
}
