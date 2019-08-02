use hex;
use reqwest;
use std::fmt;
use serde_json;
use std::error::Error;

#[derive(Debug)]
pub enum AppError {
    Custom(String),
    IOError(std::io::Error),
    HexError(hex::FromHexError),
    ReqwestError(reqwest::Error),
    SerdeJsonError(serde_json::Error),
    NoneError(std::option::NoneError),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            AppError::Custom(ref msg) =>
                format!("\n{}\n", msg),
            AppError::HexError(ref e) =>
                format!("\n✘ Hex Error!\n✘ {}\n", e),
            AppError::IOError(ref e) =>
                format!("\n✘ I/O Error!\n✘ {}\n", e),
            AppError::ReqwestError(ref e) =>
                format!("\n✘ HTTP Reqwest Error!\n✘ {}\n", e),
            AppError::SerdeJsonError(ref e) =>
                format!("\n✘ Serde-Json Error!\n✘ {}\n", e),
            AppError::NoneError(ref e) =>
                format!("\n✘ Nothing to unwrap!\n✘ {:?}\n", e),
        };
        f.write_fmt(format_args!("{}", msg))
    }
}

impl Error for AppError {
    fn description(&self) -> &str {
        "\n✘ Program Error!\n"
    }
}

impl From<hex::FromHexError> for AppError {
    fn from(e: hex::FromHexError) -> AppError {
        AppError::HexError(e)
    }
}

impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> AppError {
        AppError::IOError(e)
    }
}

impl From<std::option::NoneError> for AppError {
    fn from(e: std::option::NoneError) -> AppError {
        AppError::NoneError(e)
    }
}

impl From<reqwest::Error> for AppError {
    fn from(e: reqwest::Error) -> AppError {
        AppError::ReqwestError(e)
    }
}
impl From<serde_json::Error> for AppError {
    fn from(e: serde_json::Error) -> AppError {
        AppError::SerdeJsonError(e)
    }
}
