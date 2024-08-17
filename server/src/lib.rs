pub mod config;
pub(crate) mod db;
pub mod error;
pub(crate) mod handlers;
pub(crate) mod models;
pub(crate) mod schema;

use actix_server::Server;
use actix_web::{web, App, HttpServer};
use clap::Parser;
use config::Config;
use handlers::{balance, create_link_token, exchange_public_token};
use plaid::PlaidClient;
use std::sync::Arc;

struct AppState {
    database_url: String,
    plaid_client: PlaidClient,
    plaid_redirect_uri: String,
    plaid_android_package_name: String,
}

pub fn create_server() -> eyre::Result<Server> {
    let config = Config::parse();

    let plaid_client = PlaidClient::from_env();

    let app_data = Arc::new(AppState {
        database_url: config.database_url.clone(),
        plaid_client,
        plaid_redirect_uri: config.plaid_redirect_uri.clone(),
        plaid_android_package_name: config.plaid_android_package_name.clone(),
    });

    Ok(HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_data.clone()))
            .service(create_link_token)
            .service(exchange_public_token)
            .service(balance)
    })
    .bind(("127.0.0.1", 8080))?
    .run())
}
