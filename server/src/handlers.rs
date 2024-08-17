use std::sync::Arc;

use actix_web::{body::BoxBody, http::header::ContentType, post, web, HttpResponse, Responder};
use log::{debug, info};
use plaid::{
    model::{AccountsGetResponse, LinkTokenCreateRequestUser},
    request::LinkTokenCreateRequired,
};
use serde::{Deserialize, Serialize};

use crate::{db, AppState};

type HttpResult<T> = std::result::Result<T, crate::error::Error>;

#[derive(Deserialize)]
struct CreateLinkTokenInput {
    client_user_id: String,
    address: String,
}

#[derive(Serialize)]
struct CreateLinkTokenOutput {
    link_token: String,
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

    let res = match payload.address.as_str() {
        "localhost" => {
            let payload = LinkTokenCreateRequired {
                client_name: "Plaid Tiny QuickStart - expo RN",
                language: "en",
                country_codes: &["US"],
                user: LinkTokenCreateRequestUser {
                    client_user_id: id.clone(),
                    ..Default::default()
                },
            };

            state
                .plaid_client
                .link_token_create(payload)
                .products(vec!["auth".to_owned()])
                .redirect_uri(&state.plaid_redirect_uri)
                .await
                .unwrap()
        }
        _ => {
            let payload = LinkTokenCreateRequired {
                client_name: "Plaid Tiny QuickStart - expo RN",
                language: "en",
                country_codes: &["US"],
                user: LinkTokenCreateRequestUser {
                    client_user_id: id.clone(),
                    ..Default::default()
                },
            };

            state
                .plaid_client
                .link_token_create(payload)
                .products(vec!["auth".to_owned()])
                .android_package_name(&state.plaid_android_package_name)
                .await
                .unwrap()
        }
    };

    debug!("link_token = {}", res.link_token);

    // insert an entry for this client in the db
    // todo : insert only if row not present
    db::insert_client(&id, &state.database_url)
        .await
        .map_err(crate::error::Error::GenericError)?;

    Ok(CreateLinkTokenOutput {
        link_token: res.link_token,
    })
}

#[derive(Debug, Deserialize)]
struct ExchangePublicTokenInput {
    client_user_id: String,
    public_token: String,
}

#[derive(Debug, Serialize)]
struct ExchangePublicTokenOutput;

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
    let res = state
        .plaid_client
        .item_public_token_exchange(&public_token)
        .await
        .unwrap();

    debug!("access token = {}", res.access_token);

    // update the db row for this device id with the access token
    db::update_client(&id, &res.access_token, &state.database_url)
        .await
        .map_err(crate::error::Error::GenericError)?;

    Ok(ExchangePublicTokenOutput)
}

#[derive(Debug, Deserialize)]
struct GetBalanceInput {
    client_user_id: String,
}

#[derive(Debug, Serialize)]
struct GetBalanceOutput {
    balance: AccountsGetResponse,
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
    let res = state
        .plaid_client
        .accounts_balance_get(&token)
        .await
        .unwrap();

    Ok(GetBalanceOutput { balance: res })
}
