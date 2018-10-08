extern crate serde_json;

use super::data_struct::{Row, Tabular};
use serde_json::{Error as JSONError, Value};
use std::fs::File;
use std::io::Error;
use std::io::{Read, Write};

pub fn read_file(path: String) -> Result<String, Error> {
    let mut f = File::open(path)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn read_json(path: String) -> Result<Tabular, Error> {
    let text = read_file(path)?;
    let v: Value = serde_json::from_str(&text)?;
    println!("{:?}", v);
    let data = Tabular::new(Row::new(vec!["".to_owned()]));

    Ok(data)
}

pub fn write_json_object(path: &str, data: &Value, pretty: bool) -> Result<(), Error> {
    let text: String;
    if pretty {
        text = serde_json::to_string_pretty(data)?;
    } else {
        text = serde_json::to_string(data)?;
    }
    let mut file = File::create(path)?;
    file.write_all(text.as_bytes())?;
    Ok(())
}
