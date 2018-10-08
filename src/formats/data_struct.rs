#[derive(Debug, Clone)]
pub struct Row {
    values: Vec<String>,
}

#[derive(Debug)]
pub struct Tabular {
    headers: Row,
    data: Vec<Row>,
}

impl Row {
    pub fn new(values: Vec<String>) -> Row {
        Row { values: values }
    }

    pub fn from_iter<'a, T: Iterator<Item = &'a str>>(iter: T) -> Row {
        Row::new(iter.map(|x| x.to_owned()).collect())
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
}
