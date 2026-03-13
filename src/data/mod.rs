use csv::{IntoInnerError, Writer};
use thiserror::Error;

pub mod market;
pub mod net_worth;
pub mod ownership;
pub mod portfolio;
pub mod term;
pub mod yf;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Csv read error: {0}")]
    CsvReadError(#[from] csv::Error),
}

#[derive(Error, Debug)]
pub enum FormatError {
    #[error("Csv write error: {0}")]
    CsvWriteError(#[from] csv::Error),

    #[error("Csv into inner error: {0}")]
    CsvIntoInnerError(#[from] IntoInnerError<Writer<Vec<u8>>>),

    #[error("String from utf8 error: {0}")]
    StringFromUtf8Error(#[from] std::string::FromUtf8Error),
}
