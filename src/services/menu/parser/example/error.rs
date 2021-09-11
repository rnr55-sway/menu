use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("This cannot be")]
    _UnknownErr
}

impl super::super::Error for Error {}
