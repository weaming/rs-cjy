use std::fs::File;
use std::io::{Read, Error};

pub mod data_struct;
pub mod error;
pub mod io_csv;
pub mod io_json;
pub mod io_yaml;

pub fn read_file(path: &str) -> Result<String, Error> {
    let mut f = File::open(path)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    Ok(contents)
}
