use std::sync::Arc;

use crate::{db, AppState};
use actix_web::{body::BoxBody, http::header::ContentType, post, web, HttpResponse, Responder};
use log::{debug, info};
use plaid_sdk::{
    model::{
        accounts::{AccountsBalanceGetRequest, AccountsGetResponse},
        common::Product,
        error::PlaidErrorResponse,
        items::ItemPublicTokenExchangeRequest,
        link::{CountryCode, Language, LinkTokenCreateRequest, User},
    },
    traits::{AccountsApi, ItemsApi, LinkApi},
    Either,
};
use serde::{Deserialize, Serialize};

type HttpResult<T> = std::result::Result<T, crate::error::Error>;

#[derive(Debug, Deserialize)]
pub struct CreateLinkTokenInput {
    client_user_id: String,
    address: String,
}

#[derive(Debug, Serialize)]
pub struct CreateLinkTokenOutput {
    link_token: Option<String>,
    failure: Option<PlaidErrorResponse>,
}

impl Responder for CreateLinkTokenOutput {
    type Body = BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}

#[post("/api/create_link_token")]
pub async fn create_link_token<'a>(
    state: web::Data<Arc<AppState>>,
    payload: web::Json<CreateLinkTokenInput>,
) -> HttpResult<CreateLinkTokenOutput> {
    info!("Got a request to create a new link token");

    let id = payload.client_user_id.clone();
    debug!("client_user_id = {}", id);

    let plaid_client = &state.plaid_client;

    let payload = match payload.address.as_str() {
        "localhost" => LinkTokenCreateRequest {
            client_id: plaid_client.plaid_client_id.clone(),
            secret: plaid_client.plaid_secret.clone(),
            client_name: "Plaid Tiny QuickStart - expo RN".to_string(),
            language: Language::English,
            country_codes: vec![CountryCode::Us],
            user: User {
                client_user_id: id.clone(),
                ..Default::default()
            },
            products: vec![Product::Auth],
            redirect_uri: Some(plaid_client.plaid_redirect_uri.clone()),
            ..Default::default()
        },
        _ => LinkTokenCreateRequest {
            client_id: plaid_client.plaid_client_id.clone(),
            secret: plaid_client.plaid_secret.clone(),
            client_name: "Plaid Tiny QuickStart - expo RN".to_string(),
            language: Language::English,
            country_codes: vec![CountryCode::Us],
            user: User {
                client_user_id: id.clone(),
                ..Default::default()
            },
            products: vec![Product::Auth],
            android_package_name: Some(plaid_client.plaid_android_package_name.clone()),
            ..Default::default()
        },
    };

    match plaid_client.link_token_create(payload).await? {
        Either::Left(success) => {
            info!("link_token = {}", success.link_token);

            // insert an entry for this client in the db
            // todo : insert only if row not present - refactor flow
            db::insert_client(&id, &state.database_url)
                .await
                .map_err(crate::error::Error::GenericError)?;

            Ok(CreateLinkTokenOutput {
                link_token: Some(success.link_token),
                failure: None,
            })
        }
        Either::Right(failure) => {
            info!("plaid error response = {:#?}", failure);
            Ok(CreateLinkTokenOutput {
                link_token: None,
                failure: Some(failure),
            })
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ExchangePublicTokenInput {
    client_user_id: String,
    public_token: String,
}

#[derive(Debug, Serialize)]
pub struct ExchangePublicTokenOutput {
    failure: Option<PlaidErrorResponse>,
}

impl Responder for ExchangePublicTokenOutput {
    type Body = BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> HttpResponse<Self::Body> {
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(serde_json::to_string(&self).unwrap())
    }
}

#[post("/api/exchange_public_token")]
pub async fn exchange_public_token(
    state: web::Data<Arc<AppState>>,
    payload: web::Json<ExchangePublicTokenInput>,
) -> HttpResult<ExchangePublicTokenOutput> {
    info!("Exchanging public token for a Plaid access token");

    let id = payload.client_user_id.clone();
    let public_token = payload.public_token.clone();
    match state
        .plaid_client
        .item_public_token_exchange(ItemPublicTokenExchangeRequest {
            client_id: state.plaid_client.plaid_client_id.clone(),
            secret: state.plaid_client.plaid_secret.clone(),
            public_token: public_token.clone(),
        })
        .await?
    {
        Either::Left(success) => {
            debug!("access token = {}", success.access_token);

            // update the db row for this device id with the access token
            db::update_client(&id, &success.access_token, &state.database_url)
                .await
                .map_err(crate::error::Error::GenericError)?;

            Ok(ExchangePublicTokenOutput { failure: None })
        }
        Either::Right(failure) => {
            info!("plaid error response = {:#?}", failure);
            Ok(ExchangePublicTokenOutput {
                failure: Some(failure),
            })
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct GetBalanceInput {
    client_user_id: String,
}

#[derive(Debug, Serialize)]
pub struct GetBalanceOutput {
    balance: Option<AccountsGetResponse>,
    failure: Option<PlaidErrorResponse>,
}

impl Responder for GetBalanceOutput {
    type Body = BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> HttpResponse<Self::Body> {
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(serde_json::to_string(&self).unwrap())
    }
}

#[post("/api/balance")]
pub async fn balance(
    state: web::Data<Arc<AppState>>,
    payload: web::Json<GetBalanceInput>,
) -> HttpResult<GetBalanceOutput> {
    info!("Retrieving balance for account");

    let id = payload.client_user_id.clone();
    let client = db::get_client(&id, &state.database_url)
        .await
        .map_err(crate::error::Error::GenericError)?;

    let token = client.access_token.unwrap();
    match state
        .plaid_client
        .accounts_balance_get(AccountsBalanceGetRequest {
            client_id: state.plaid_client.plaid_client_id.clone(),
            secret: state.plaid_client.plaid_secret.clone(),
            access_token: token,
            ..Default::default()
        })
        .await?
    {
        Either::Left(success) => {
            info!("balance response = {success:#?}");
            Ok(GetBalanceOutput {
                balance: Some(success),
                failure: None,
            })
        }
        Either::Right(failure) => {
            info!("plaid error response = {:#?}", failure);
            Ok(GetBalanceOutput {
                balance: None,
                failure: Some(failure),
            })
        }
    }
}
