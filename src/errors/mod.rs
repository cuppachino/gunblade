mod credential_error;
use std::fmt::Display;

pub use credential_error::CredentialError;
use reqwest::Error;

#[derive(Debug)]
pub enum LcuError {
    /// An error occurred while extracting credentials from a LeagueClientUx process.
    CredentialError(CredentialError),
    /// An error occurred while connecting to the LCU.
    BadClient(Error),
}

impl Display for LcuError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LcuError::CredentialError(cred_err) => match cred_err {
                CredentialError::NotEnoughTokens => write!(f, "Not enough tokens"),
                CredentialError::NotFound => write!(f, "Not found"),
            },
            LcuError::BadClient(e) => write!(f, "Bad client: {:?}", e),
        }
    }
}
