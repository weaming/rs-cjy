use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct IOError {
    details: String,
}

impl IOError {
    pub fn new(msg: &str) -> IOError {
        IOError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for IOError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for IOError {
    fn description(&self) -> &str {
        &self.details
    }
}
