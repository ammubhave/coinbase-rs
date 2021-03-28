use std::fmt;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    /// Error returned by Coinbase
    Coinbase(CoinbaseError),
    /// Error when sending request
    Request(reqwest::Error),
    /// Error when serializing/deserializing error
    Json(serde_json::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Error - {}",
            match self {
                Self::Coinbase(err) => err.to_string(),
                Self::Request(err) => err.to_string(),
                Self::Json(err) => err.to_string(),
            },
        )
    }
}

impl From<CoinbaseError> for Error {
    fn from(err: CoinbaseError) -> Self {
        Self::Coinbase(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Self::Json(err)
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Self::Request(err)
    }
}

#[derive(Debug, Clone)]
pub struct CoinbaseError {
    /// The HTTP status code associated with the error.
    pub status_code: reqwest::StatusCode,
}

impl fmt::Display for CoinbaseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Coinbase Error - code: {}", self.status_code)
    }
}
