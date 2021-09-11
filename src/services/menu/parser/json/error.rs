use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Cannot read stream: {0}")]
    StreamError(std::io::Error),
    #[error("Cannot read stream: {0}")]
    LockError(std::cell::BorrowMutError),
    #[error("Cannot parse: {0}")]
    ParseError(serde_json::Error)
}

impl super::super::Error for Error {}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Error {
        Error::ParseError(e)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Error {
        Error::StreamError(e)
    }
}

impl From<std::cell::BorrowMutError> for Error {
    fn from(e: std::cell::BorrowMutError) -> Error {
        Error::LockError(e)
    }
}
