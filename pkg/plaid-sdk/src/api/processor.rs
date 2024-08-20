use async_trait::async_trait;

use crate::{
    model::{
        error::PlaidErrorResponse,
        processor::{
            ProcessorStripeBankAccountTokenCreateRequest,
            ProcessorStripeBankAccountTokenCreateResponse, ProcessorTokenCreateRequest,
            ProcessorTokenCreateResponse,
        },
    },
    traits::{PlaidApi, ProcessorApi, ProcessorStripeApi},
    Either, PlaidClient, PlaidResult,
};

impl PlaidApi<ProcessorTokenCreateRequest, ProcessorTokenCreateResponse> for PlaidClient {}

#[async_trait]
impl ProcessorApi for PlaidClient {
    async fn processor_token_create(
        req: ProcessorTokenCreateRequest,
    ) -> PlaidResult<Either<ProcessorTokenCreateResponse, PlaidErrorResponse>> {
        todo!()
    }
}

impl
    PlaidApi<
        ProcessorStripeBankAccountTokenCreateRequest,
        ProcessorStripeBankAccountTokenCreateResponse,
    > for PlaidClient
{
}

#[async_trait]
impl ProcessorStripeApi for PlaidClient {
    async fn processor_stripe_bank_account_token_create(
        req: ProcessorStripeBankAccountTokenCreateRequest,
    ) -> PlaidResult<Either<ProcessorStripeBankAccountTokenCreateResponse, PlaidErrorResponse>>
    {
        todo!()
    }
}
