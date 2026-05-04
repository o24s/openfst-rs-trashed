use std::error::Error;
use std::fmt;

/// An error that occurred during an OpenFst operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpenFstError {
    message: String,
}

impl OpenFstError {
    /// Creates a new `OpenFstError` with the given message.
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }

    /// Returns the error message.
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for OpenFstError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "OpenFst error: {}", self.message)
    }
}

impl Error for OpenFstError {}

impl From<cxx::Exception> for OpenFstError {
    fn from(err: cxx::Exception) -> Self {
        Self::new(err.what())
    }
}
