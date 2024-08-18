use async_trait::async_trait;

use crate::{
    model::{
        error::PlaidErrorResponse,
        link::{LinkTokenCreateRequest, LinkTokenCreateResponse},
    },
    traits::{LinkApi, PlaidApi},
    Either, PlaidClient, PlaidResult,
};

impl PlaidApi<LinkTokenCreateRequest, LinkTokenCreateResponse> for PlaidClient {}

#[async_trait]
impl LinkApi for PlaidClient {
    async fn link_token_create(
        &self,
        req: LinkTokenCreateRequest,
    ) -> PlaidResult<Either<LinkTokenCreateResponse, PlaidErrorResponse>> {
        let link_token_create_url = format!("{}/link/token/create", self.plaid_env.get_base_url());
        self.call(&self.reqwest_client, req, &link_token_create_url)
            .await
    }
}
