use crate::{
    error::PlaidError,
    model::{
        accounts::{AccountsBalanceGetRequest, AccountsGetResponse},
        error::PlaidErrorResponse,
        items::{ItemPublicTokenExchangeRequest, ItemPublicTokenExchangeResponse},
        link::{LinkTokenCreateRequest, LinkTokenCreateResponse},
        processor::{
            ProcessorStripeBankAccountTokenCreateRequest,
            ProcessorStripeBankAccountTokenCreateResponse, ProcessorTokenCreateRequest,
            ProcessorTokenCreateResponse,
        },
    },
    Either, PlaidResult,
};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[async_trait]
pub trait PlaidApi<Req: Serialize + Send + 'static + Sync, Resp: Debug + for<'a> Deserialize<'a>> {
    async fn call(
        &self,
        client: &reqwest::Client,
        req: Req,
        url: &str,
    ) -> PlaidResult<Either<Resp, PlaidErrorResponse>> {
        let res = client
            .post(url)
            .json(&req)
            .send()
            .await
            .map_err(PlaidError::Reqwest)?;

        let status = res.status();
        println!("status = {status:#?}");
        if status.is_success() {
            let succ_res = res.json::<Resp>().await.map_err(PlaidError::Reqwest)?;
            println!("succ_res = {succ_res:#?}");
            Ok(Either::Left(succ_res))
        } else {
            let failure_res = res
                .json::<PlaidErrorResponse>()
                .await
                .map_err(PlaidError::Reqwest)?;
            println!("failure = {failure_res:#?}");
            Ok(Either::Right(failure_res))
        }
    }
}

#[async_trait]
pub trait LinkApi: PlaidApi<LinkTokenCreateRequest, LinkTokenCreateResponse> {
    async fn link_token_create(
        &self,
        req: LinkTokenCreateRequest,
    ) -> PlaidResult<Either<LinkTokenCreateResponse, PlaidErrorResponse>>;
}

#[async_trait]
pub trait ItemsApi:
    PlaidApi<ItemPublicTokenExchangeRequest, ItemPublicTokenExchangeResponse>
{
    async fn item_public_token_exchange(
        &self,
        req: ItemPublicTokenExchangeRequest,
    ) -> PlaidResult<Either<ItemPublicTokenExchangeResponse, PlaidErrorResponse>>;
}

#[async_trait]
pub trait AccountsApi: PlaidApi<AccountsBalanceGetRequest, AccountsGetResponse> {
    async fn accounts_balance_get(
        &self,
        req: AccountsBalanceGetRequest,
    ) -> PlaidResult<Either<AccountsGetResponse, PlaidErrorResponse>>;
}

#[async_trait]
pub trait ProcessorApi:
    PlaidApi<ProcessorTokenCreateRequest, ProcessorTokenCreateResponse>
{
    async fn processor_token_create(
        req: ProcessorTokenCreateRequest,
    ) -> PlaidResult<Either<ProcessorTokenCreateResponse, PlaidErrorResponse>>;
}

#[async_trait]
pub trait ProcessorStripeApi:
    PlaidApi<
    ProcessorStripeBankAccountTokenCreateRequest,
    ProcessorStripeBankAccountTokenCreateResponse,
>
{
    async fn processor_stripe_bank_account_token_create(
        req: ProcessorStripeBankAccountTokenCreateRequest,
    ) -> PlaidResult<Either<ProcessorStripeBankAccountTokenCreateResponse, PlaidErrorResponse>>;
}

#[async_trait]
pub trait TransferApi {}
