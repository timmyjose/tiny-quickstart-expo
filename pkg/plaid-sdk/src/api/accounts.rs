use async_trait::async_trait;

use crate::{
    model::{
        accounts::{AccountsBalanceGetRequest, AccountsGetResponse},
        error::PlaidErrorResponse,
    },
    traits::{AccountsApi, PlaidApi},
    Either, PlaidClient, PlaidResult,
};

impl PlaidApi<AccountsBalanceGetRequest, AccountsGetResponse> for PlaidClient {}

#[async_trait]
impl AccountsApi for PlaidClient {
    async fn accounts_balance_get(
        &self,
        req: AccountsBalanceGetRequest,
    ) -> PlaidResult<Either<AccountsGetResponse, PlaidErrorResponse>> {
        let accounts_balance_get_url =
            format!("{}/accounts/balance/get", self.plaid_env.get_base_url());
        self.call(&self.reqwest_client, req, &accounts_balance_get_url)
            .await
    }
}
