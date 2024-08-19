use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PlaidErrorType {
    ApiError,
    AssetReportError,
    BankTransferError,
    IncomeVerificationError,
    InstitutionError,
    InvalidInput,
    InvalidRequest,
    InvalidResult,
    ItemError,
    MicrodepositsError,
    OauthError,
    PartnerError,
    PaymentError,
    RateLimitExceeded,
    RecaptchaError,
    SandboxError,
    TransactionError,
    TransactionsError,
    TransferError,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PlaidErrorCode {
    AdditionalConsentRequired,
    DirectIntegrationnotEnabled,
    IncorrectDepositVerification,
    InvalidAccessToken,
    InvalidAccountId,
    InvalidApiKeys,
    InvalidAuditCopyToken,
    InvalidClientId,
    InvalidField,
    InvalidInstitution,
    InvalidLinkCustomization,
    InvalidProcessorCount,
    InvalidProduct,
    InvalidProducts,
    InvalidPublicToken,
    InvalidStripeAccount,
    InvalidUserToken,
    InvalidWebhookVerificationKeyId,
    ProductUnavailable,
    TooManyVerificationAttempts,
    UnauthorizedEnvironment,
    UnauthorizedRouteAccess,
    UnknownFields,
    UserPermissionRevoked,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PlaidErrorCodeReason {
    OauthInvalidToken,
    OauthConsentExpired,
    OauthRevokedToken,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlaidErrorResponse {
    pub causes: Option<Vec<Value>>,
    pub display_message: Option<String>,
    pub documentation_url: String,
    pub error_code: PlaidErrorCode,
    pub error_code_reason: Option<PlaidErrorCodeReason>,
    pub error_message: String,
    pub error_type: PlaidErrorType,
    pub request_id: String,
    pub status: Option<i64>,
    pub suggested_action: Option<String>,
}
