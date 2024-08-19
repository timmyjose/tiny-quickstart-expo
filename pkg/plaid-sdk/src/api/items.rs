use crate::{
    model::{
        error::PlaidErrorResponse,
        items::{ItemPublicTokenExchangeRequest, ItemPublicTokenExchangeResponse},
    },
    traits::{ItemsApi, PlaidApi},
    Either, PlaidClient, PlaidResult,
};
use async_trait::async_trait;

impl PlaidApi<ItemPublicTokenExchangeRequest, ItemPublicTokenExchangeResponse> for PlaidClient {}

#[async_trait]
impl ItemsApi for PlaidClient {
    async fn item_public_token_exchange(
        &self,
        req: ItemPublicTokenExchangeRequest,
    ) -> PlaidResult<Either<ItemPublicTokenExchangeResponse, PlaidErrorResponse>> {
        let item_public_token_exchange_url = format!(
            "{}/item/public_token/exchange",
            self.plaid_env.get_base_url()
        );

        self.call(&self.reqwest_client, req, &item_public_token_exchange_url)
            .await
    }
}
