use std::fmt;

use clap::{arg, command, Parser, ValueEnum};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, ValueEnum)]
pub(crate) enum PlaidEnv {
    Sandbox,
    Production,
}

impl fmt::Display for PlaidEnv {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PlaidEnv::Sandbox => "sandbox",
                PlaidEnv::Production => "production",
            }
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Parser)]
#[command(version, about, long_about = None)]
pub(crate) struct Config {
    #[arg(env, short, long)]
    pub database_url: String,

    #[arg(env, long)]
    pub plaid_client_id: String,

    #[arg(env, long)]
    pub plaid_secret: String,

    #[arg(env, long, value_enum)]
    pub plaid_env: PlaidEnv,

    #[arg(env, long)]
    pub plaid_redirect_uri: String,

    #[arg(env, long)]
    pub plaid_android_package_name: String,
}
