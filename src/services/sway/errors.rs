use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConnectionError {
    #[error("Cannot connect to sway: (`{0}`)")]
    Sway(failure::Error)
}

impl From<failure::Error> for ConnectionError {
    fn from(e: failure::Error) -> Self {
        Self::Sway(e)
    }
}

#[derive(Debug, Error)]
pub enum RunError {
    #[error("Error on command execution: (`{0}`)")]
    Failed(String),
    #[error("Cannot run command: (`{0}`)")]
    Sway(failure::Error),
    #[error("Unknown error")]
    UnknownErr
}

impl From<failure::Error> for RunError {
    fn from(e: failure::Error) -> Self {
        Self::Sway(e)
    }
}
