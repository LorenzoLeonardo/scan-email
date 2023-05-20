use std::{
    error,
    fmt::{self, Debug},
};

#[derive(Debug)]
pub struct Error(pub String);

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub type ScanEmailResult<T> = Result<T, Error>;
