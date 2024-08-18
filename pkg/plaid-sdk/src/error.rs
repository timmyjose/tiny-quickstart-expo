#[derive(Debug, thiserror::Error)]
pub enum PlaidError {
    #[error("Invalid date of birth: {0}")]
    InvalidDateOfBirth(String),

    #[error("Invalid phone number: {0}")]
    InvalidPhoneNumber(String),

    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    #[error(transparent)]
    Serde(#[from] serde_json::Error),
}
