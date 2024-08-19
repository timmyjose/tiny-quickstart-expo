use error::PlaidError;
use serde::{Deserialize, Serialize};

pub mod api;
pub mod error;
pub mod model;
pub mod traits;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PlaidEnv {
    Sandbox,
    Production,
}

impl PlaidEnv {
    pub(crate) fn get_base_url(&self) -> &'static str {
        match self {
            PlaidEnv::Sandbox => "https://sandbox.plaid.com",
            PlaidEnv::Production => "https://production.plaid.com",
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Either<L, R> {
    Left(L),
    Right(R),
}

pub type PlaidResult<T> = std::result::Result<T, PlaidError>;

pub struct PlaidClient {
    pub plaid_client_id: String,
    pub plaid_secret: String,
    pub plaid_env: PlaidEnv,
    pub plaid_redirect_uri: String,
    pub plaid_android_package_name: String,
    reqwest_client: reqwest::Client,
}

impl PlaidClient {
    pub fn new(
        plaid_client_id: &str,
        plaid_secret: &str,
        plaid_env: &str,
        plaid_redirect_uri: &str,
        plaid_android_package_name: &str,
    ) -> Self {
        Self {
            plaid_client_id: plaid_client_id.into(),
            plaid_secret: plaid_secret.into(),
            plaid_env: plaid_env.into(),
            plaid_redirect_uri: plaid_redirect_uri.into(),
            plaid_android_package_name: plaid_android_package_name.into(),
            reqwest_client: reqwest::Client::new(),
        }
    }
}

impl From<&str> for PlaidEnv {
    fn from(env: &str) -> Self {
        match env {
            "sandbox" => PlaidEnv::Sandbox,
            "production" => PlaidEnv::Production,
            _ => PlaidEnv::Production,
        }
    }
}
