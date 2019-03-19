use super::create_io_error;
use super::serde_json;
use super::*;
use csv;
use linked_hash_map::LinkedHashMap;
use std::fmt;
use std::io::Error;
use yaml_rust::Yaml;

#[derive(Debug, Clone, PartialEq)]
pub enum Number {
    Int(i64),
    Float(f64),
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Number::Int(v) => write!(f, "{}", v),
            Number::Float(v) => write!(f, "{}", v),
        }
    }
}

#[derive(Debug, Clone)]
pub enum BasicTypes {
    String(String),
    Number(Number),
    Null,
}

impl ToString for BasicTypes {
    fn to_string(&self) -> String {
        match self {
            BasicTypes::String(s) => s.to_owned(),
            BasicTypes::Number(num) => format!("{}", num),
            BasicTypes::Null => "null".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Row {
    pub values: Vec<BasicTypes>,
}

#[derive(Debug)]
pub struct Tabular {
    pub headers: Row,
    pub data: Vec<Row>,
}

pub fn str_to_basictypes(v: String) -> BasicTypes {
    match v.parse::<f64>() {
        Ok(f_val) => {
            if v.contains(".") {
                return BasicTypes::Number(Number::Float(f_val));
            } else {
                let num = v.parse::<i64>().unwrap();
                return BasicTypes::Number(Number::Int(num));
            };
        }
        Err(_) => BasicTypes::String(v),
    }
}

impl Row {
    pub fn new(values: Vec<String>) -> Row {
        let mut row = Row { values: vec![] };
        for v in values {
            row.values.push(str_to_basictypes(v));
        }
        row
    }

    pub fn from_iter<'a, T: Iterator<Item = &'a str>>(iter: T) -> Row {
        Row::new(iter.map(|x| String::from(x)).collect())
    }

    pub fn as_vec(&self) -> &Vec<BasicTypes> {
        &self.values
    }

    pub fn to_csv_vec(&self) -> Vec<String> {
        self.values.iter().map(|v| v.to_string()).collect()
    }

    pub fn to_serde_map(&self, headers: &Row) -> serde_json::Map<String, serde_json::Value> {
        let mut map = serde_json::Map::new();
        for (i, v) in self.values.iter().enumerate() {
            let serde_v = match v.clone() {
                BasicTypes::String(s) => serde_json::Value::String(s),
                BasicTypes::Number(n) => match n {
                    Number::Int(n) => json!(n),
                    Number::Float(n) => json!(n),
                },
                BasicTypes::Null => serde_json::Value::Null,
            };
            map.insert(headers.values[i].clone().to_string(), serde_v);
        }
        map
    }

    pub fn to_yaml_hash(&self, headers: &Row) -> Yaml {
        let mut rv = LinkedHashMap::new();

        for (i, v) in self.values.iter().enumerate() {
            let yaml_val = match v.clone() {
                BasicTypes::String(s) => Yaml::String(s),
                BasicTypes::Number(n) => match n {
                    Number::Int(n) => Yaml::Integer(n),
                    Number::Float(n) => Yaml::Real(n.to_string()),
                },
                BasicTypes::Null => Yaml::Null,
            };
            rv.insert(
                Yaml::String(headers.values[i].clone().to_string()),
                yaml_val,
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
            match wtr.write_record(self.headers.to_csv_vec()) {
                Err(e) => {
                    return Err(create_io_error(&format!("{:?}", e)));
                }
                _ => {}
            }
        }

        for row in self.data.iter() {
            match wtr.write_record(row.to_csv_vec()) {
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
