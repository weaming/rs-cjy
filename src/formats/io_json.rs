use serde_json;

use super::create_io_error;
use super::data_struct::{Row, Tabular};
use super::read_file;
use serde_json::Value;
use std::collections::HashSet;
use std::fs::File;
use std::io::Write;
use std::io::Error;

pub fn read_json(path: &str) -> Result<Tabular, Error> {
    let text = read_file(path)?;
    parse_json(&text)
}

pub fn parse_json(text: &str) -> Result<Tabular, Error> {
    let value: Value = serde_json::from_str(text)?;

    let data = match value {
        Value::Array(v) => {
            let mut headers: HashSet<String> = HashSet::new();
            // validate struct
            for (i, row) in v.iter().enumerate() {
                match row {
                    Value::Object(row) => {
                        if i == 0 {
                            for entry in row {
                                headers.insert(entry.0.to_owned());
                            }
                        } else {
                            for entry in row {
                                if !headers.contains(entry.0) {
                                    return Err(create_io_error(
                                        "the json is not a fully valid tabular struct",
                                    ));
                                }
                            }
                        }
                    }
                    _ => return Err(create_io_error("the tabular row is not a object")),
                }
            }

            // do the real parse
            let mut headers_row: Row = Row::new(vec![]);
            for k in headers {
                headers_row.values.push(k.to_string());
            }

            let mut rv = Tabular::new(headers_row.clone());
            for row in v {
                let mut r = Row::new(vec![]);
                for k in headers_row.as_vec().iter() {
                    // TODO: process differenct value types
                    r.values.push(match row.get(k).unwrap() {
                        Value::String(s) => s.to_string(),
                        Value::Number(s) => format!("{}", s),
                        Value::Bool(s) => format!("{}", s),
                        _ => "".to_string(),
                    });
                }
                rv.add_row(r);
            }

            rv
        }
        _ => return Err(create_io_error("the json is not array")),
    };

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
