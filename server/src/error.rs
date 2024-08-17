use actix_web::ResponseError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    PlaidError(#[from] plaid::Error),

    #[error(transparent)]
    GenericError(#[from] eyre::ErrReport),
}

impl ResponseError for Error {}
