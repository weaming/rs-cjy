extern crate serde_json;

use super::create_io_error;
use super::*;
use csv;
use linked_hash_map::LinkedHashMap;
use std::io::Error;
use yaml_rust::Yaml;

// TODO: parse accorrding to the field value type
pub enum JSONTypes {
    String,
    Int,
    Float,
    Null,
}

#[derive(Debug, Clone)]
pub struct Row {
    pub values: Vec<String>,
}

#[derive(Debug)]
pub struct Tabular {
    pub headers: Row,
    pub data: Vec<Row>,
}

impl Row {
    pub fn new(values: Vec<String>) -> Row {
        Row { values: values }
    }

    pub fn from_iter<'a, T: Iterator<Item = &'a str>>(iter: T) -> Row {
        Row::new(iter.map(|x| String::from(x)).collect())
    }

    pub fn as_vec(&self) -> &Vec<String> {
        &self.values
    }

    pub fn to_serde_map(&self, headers: &Row) -> serde_json::Map<String, serde_json::Value> {
        let mut map = serde_json::Map::new();
        for (i, v) in self.values.iter().enumerate() {
            map.insert(
                headers.values[i].clone(),
                serde_json::Value::String(v.clone()),
            );
        }
        map
    }

    pub fn to_yaml_hash(&self, headers: &Row) -> Yaml {
        let mut rv = LinkedHashMap::new();

        for (i, v) in self.values.iter().enumerate() {
            rv.insert(
                Yaml::String(headers.values[i].clone()),
                Yaml::String(v.clone()),
            );
        }
        Yaml::Hash(rv)
    }
}

impl Tabular {
    pub fn new(headers: Row) -> Tabular {
        Tabular {
            headers: headers,
            data: vec![],
        }
    }

    pub fn add_row(&mut self, row: Row) {
        self.data.push(row);
    }

    pub fn add_data_from_iter<T>(&mut self, iter: T)
    where
        T: Iterator<Item = Row>,
    {
        for x in iter {
            self.data.push(x)
        }
    }

    pub fn has_headers(&self) -> bool {
        self.headers.values.len() > 0
    }

    pub fn has_data(&self) -> bool {
        self.data.len() > 0
    }

    pub fn write_csv(&self, path: &str) -> Result<(), Error> {
        let mut wtr = csv::Writer::from_path(path)?;
        if self.has_headers() {
            match wtr.write_record(self.headers.as_vec()) {
                Err(e) => {
                    return Err(create_io_error(&format!("{:?}", e)));
                }
                _ => {}
            }
        }

        for row in self.data.iter() {
            match wtr.write_record(row.as_vec()) {
                Err(e) => {
                    return Err(create_io_error(&format!("{:?}", e)));
                }
                _ => {}
            }
        }
        Ok(())
    }

    pub fn get_output_headers_data(&self) -> Result<(&Row, &[Row]), Error> {
        let headers;
        let data;
        if self.has_headers() {
            headers = &self.headers;
            data = self.data.as_slice();
        } else {
            if self.has_data() {
                headers = &self.data[0];
                data = &self.data[1..];
            } else {
                return Err(create_io_error("the tablular does not have data"));
            }
        }
        Ok((&headers, data))
    }

    pub fn write_json(&self, path: &str) -> Result<(), Error> {
        let (headers, data) = self.get_output_headers_data()?;
        let data = data.iter().map(|row| row.to_serde_map(headers)).collect();
        io_json::write_json_object(path, &data, has_env("PRETTY"))?;
        Ok(())
    }

    pub fn write_yaml(&self, path: &str) -> Result<(), Error> {
        let (headers, data) = self.get_output_headers_data()?;
        let data = data.iter().map(|row| row.to_yaml_hash(headers)).collect();
        let doc = Yaml::Array(data);
        io_yaml::write_yaml_doc(path, &doc)?;
        Ok(())
    }
}
