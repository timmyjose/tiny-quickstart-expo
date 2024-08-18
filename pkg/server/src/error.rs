use actix_web::{http::StatusCode, ResponseError};
use plaid_sdk::error::PlaidError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    PlaidError(#[from] plaid_sdk::error::PlaidError),

    #[error(transparent)]
    GenericError(#[from] eyre::ErrReport),
}

impl ResponseError for Error {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            Error::PlaidError(err) => match err {
                PlaidError::InvalidDateOfBirth(_) => StatusCode::BAD_REQUEST,
                PlaidError::InvalidPhoneNumber(_) => StatusCode::BAD_REQUEST,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            },
            Error::GenericError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
