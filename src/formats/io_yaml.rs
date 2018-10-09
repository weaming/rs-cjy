use super::create_io_error;
use super::read_file;
use super::{Row, Tabular};
use serde_json::Value;
use std::collections::HashSet;
use std::fs::File;
use std::io::Write;
use std::io::{Error, ErrorKind};
use yaml_rust::{Yaml, YamlEmitter, YamlLoader};

pub fn read_yaml(path: &str) -> Result<Tabular, Error> {
    let text = read_file(path)?;
    parse_yaml(&text)
}

pub fn parse_yaml(text: &str) -> Result<Tabular, Error> {
    let docs = YamlLoader::load_from_str(text).map_err(|_e| create_io_error("load yaml fail"))?;
    let doc = &docs[0];
    let data = match doc {
        Yaml::Array(doc) => {
            // validate struct
            let mut headers: HashSet<String> = HashSet::new();
            for (i, row) in doc.iter().enumerate() {
                match row {
                    Yaml::Hash(row) => {
                        if i == 0 {
                            for entry in row {
                                match entry.0 {
                                    Yaml::String(s) => {
                                        headers.insert(s.to_string());
                                    }
                                    _ => {}
                                }
                            }
                        } else {
                            for entry in row {
                                match entry.0 {
                                    Yaml::String(s) => {
                                        if !headers.contains(s) {
                                            return Err(create_io_error(
                                                "the yaml is not a fully valid tabular struct",
                                            ));
                                        }
                                    }
                                    _ => {}
                                };
                            }
                        }
                    }
                    _ => return Err(create_io_error("row type is not hash")),
                }
            }

            // do the real parse
            let mut headers_row: Row = Row::new(vec![]);
            for k in headers {
                headers_row.values.push(k.to_string());
            }

            let mut rv = Tabular::new(headers_row.clone());
            for row in doc.to_vec() {
                let mut r = Row::new(vec![]);
                let row_hash = row.into_hash().unwrap();
                for k in headers_row.as_vec().iter() {
                    // TODO: process differenct value types
                    let field = match row_hash.get(&Yaml::String(k.to_string())).unwrap() {
                        Yaml::String(s) => s.to_string(),
                        Yaml::Real(s) => format!("{}", s),
                        Yaml::Integer(s) => format!("{}", s),
                        Yaml::Boolean(s) => format!("{}", s),
                        _ => "".to_string(),
                    };
                    r.values.push(field);
                }
                rv.add_row(r);
            }

            rv
        }
        _ => return Err(create_io_error("not a valid array")),
    };

    Ok(data)
}
