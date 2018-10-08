extern crate csv;

use std::error::Error;
use super::data_struct::{Row, Tabular};

pub fn read_csv(path: String) -> Result<Tabular, Box<Error>> {
    let mut rdr = csv::Reader::from_path(path)?;
    let mut data = Tabular::new(Row::from_iter(rdr.headers()?.iter()));
    data.add_data_from_iter(rdr.records().map(|row| Row::from_iter(row.unwrap().iter())));
    Ok(data)
}
