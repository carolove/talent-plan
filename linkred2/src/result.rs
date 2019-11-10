use failure::Fail;
use std::io;
use std::string::FromUtf8Error;

/// Error type for lk
#[derive(Fail, Debug)]
pub enum LKError {
    /// IO error
    #[fail(display = "IO error: {}", _0)]
    Io(#[cause] io::Error),
    /// Serialization or deserialization error
    #[fail(display = "serde_json error: {}", _0)]
    Serde(#[cause] serde_json::Error),
    /// Unexpected command type error.
    /// It indicated a corrupted log or a program bug.
    #[fail(display = "Unexpected command type")]
    UnexpectedCommandType,
    /// Key or value is invalid UTF-8 sequence
    #[fail(display = "UTF-8 error: {}", _0)]
    Utf8(#[cause] FromUtf8Error),
    /// Error with a string message
    #[fail(display = "{}", _0)]
    StringError(String),
}

impl From<io::Error> for LKError {
    fn from(err: io::Error) -> LKError {
        LKError::Io(err)
    }
}

impl From<serde_json::Error> for LKError {
    fn from(err: serde_json::Error) -> LKError {
        LKError::Serde(err)
    }
}

impl From<FromUtf8Error> for LKError {
    fn from(err: FromUtf8Error) -> LKError {
        LKError::Utf8(err)
    }
}

/// Result type for kvs
pub type Result<T> = std::result::Result<T, LKError>;