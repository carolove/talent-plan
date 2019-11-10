use failure::Fail;
use std::io;

#[derive(Fail, Debug)]
pub enum  KvError {
    #[fail(display = "{}", _0)]
    Io(#[cause] io::Error),

    #[fail(display = "{}", _0)]
    Serde(#[cause] serde_json::Error),

    #[fail(display = "key not found")]
    KeyNotFound,

    #[fail(display = "unexpected command type")]
    UnexpectedCommandType,
}

impl From<io::Error> for KvError {
    fn from(err : io::Error) -> KvError {
        KvError::Io(err)
    }
}

impl From<serde_json::Error> for KvError {
    fn from (err : serde_json::Error) -> KvError {
        KvError::Serde(err)
    }
}

pub type Result<T> = std::result::Result<T, KvError>;