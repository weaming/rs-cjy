use super::data_struct::{Row, Tabular};
use super::error::IOError;
use csv::{ReaderBuilder, Trim};
use std::error::Error;
use super::read_file;

pub fn read_csv(path: &str) -> Result<Tabular, Box<Error>> {
    let text = read_file(path)?;
    parse_csv(&text)
}

pub fn parse_csv(text: &str) -> Result<Tabular, Box<Error>> {
    let mut rdr = ReaderBuilder::new().trim(Trim::All).from_reader(text.as_bytes());
    let header = Row::from_iter(rdr.headers()?.iter());
    let mut data = Tabular::new(header);
    let mut has_error = false;
    // ignore error row
    data.add_data_from_iter(
        rdr.records()
            .take_while(|row| match row {
                Ok(_) => true,
                Err(_) => {
                    has_error = true;
                    false
                }
            })
            .map(|row| Row::from_iter(row.unwrap().iter())),
    );
    if has_error {
        Err(Box::new(IOError::new("error reading csv")))
    } else {
        Ok(data)
    }
}
