use clap::{arg, command, Parser, ValueEnum};
use serde::Serialize;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Serialize)]
pub(crate) enum PlaidEnv {
    Sandbox,
    Development,
    Production,
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub(crate) struct Config {
    #[arg(env, short, long)]
    pub database_url: String,

    #[arg(env, short, long)]
    pub plaid_redirect_uri: String,

    #[arg(env, short, long)]
    pub plaid_android_package_name: String,
}
