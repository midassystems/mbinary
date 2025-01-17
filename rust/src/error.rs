use chrono;
use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("Encoding error: {0}")]
    Encode(String),
    #[error("Decoding error: {0}")]
    Decode(String),
    #[error("Conversion error: {0}")]
    Conversion(String),
    #[error("Custom error: {0}")]
    CustomError(String),
    #[error("Parse Error : {0}")]
    ParseError(#[from] chrono::ParseError),
    #[error("Invalid Record Type : {0}")]
    InvalidRecordType(&'static str),
    #[error("Date error: {0}")]
    DateError(String),
}

impl Error {
    pub fn extract_message(&self) -> String {
        let error_string = self.to_string();
        if let Some(index) = error_string.find(':') {
            error_string[index + 1..].trim().to_string()
        } else {
            error_string
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
