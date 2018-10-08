extern crate csv;

use super::data_struct::{Row, Tabular};
use std::error::Error;

pub fn read_csv(path: String) -> Result<Tabular, Box<Error>> {
    let mut rdr = csv::Reader::from_path(path)?;
    let mut data = Tabular::new(Row::from_iter(rdr.headers()?.iter()));
    // ignore error row
    data.add_data_from_iter(
        rdr.records()
            .take_while(|row| match row {
                Ok(_) => true,
                Err(_) => false,
            }).map(|row| Row::from_iter(row.unwrap().iter())),
    );
    Ok(data)
}
