use quick_xml::DeError as XMLError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MensaError {
    #[error("failed to request: {0}")]
    Failure(String),
    #[error("failed to derive xml")]
    ParseFailure(XMLError),
}

impl From<XMLError> for MensaError {
    fn from(error: XMLError) -> Self {
        Self::ParseFailure(error)
    }
}

impl From<&str> for MensaError {
    fn from(error: &str) -> Self {
        Self::Failure(error.to_string())
    }
}

impl From<String> for MensaError {
    fn from(error: String) -> Self {
        Self::Failure(error)
    }
}
