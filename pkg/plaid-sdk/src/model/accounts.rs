use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::{common::Product, error::PlaidErrorResponse};

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccountBalanceGetRequestOptions {
    pub account_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccountsBalanceGetRequest {
    pub client_id: String,
    pub secret: String,
    pub access_token: String,
    pub options: Option<AccountBalanceGetRequestOptions>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AccountType {
    Depository,
    Credit,
    Loan,
    Investment,
    Other,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccountVerificationStatus {
    AutomaticallyVerified,
    PendingAutomaticVerification,
    PendingManualVerification,
    ManuallyVerified,
    VerificationExpired,
    VerificationFailed,
    DatabaseMatched,
    DatabaseInsightsPass,
    DatabaseInsightsPassWithCaution,
    DatabaseInsightsFail,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AccountNetworkStatus {
    pub has_numbers_match: bool,
    pub is_numbers_match_verified: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AccountPreviousReturns {
    pub has_previous_administrative_return: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AccountNumberFormat {
    Valid,
    Invalid,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AccountVerificationInsights {
    pub network_status: AccountNetworkStatus,
    pub previous_returns: AccountPreviousReturns,
    pub account_number_format: AccountNumberFormat,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AccountHolderCategory {
    Business,
    Personal,
    Unrecognized,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AccountBalance {
    pub available: Option<f64>,
    pub current: Option<f64>,
    pub limit: Option<f64>,
    pub iso_currency_code: Option<String>,
    pub unofficial_currency_code: Option<String>,
    pub last_updated_datetime: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Account {
    pub account_id: String,
    pub balances: AccountBalance,
    pub mask: Option<String>,
    pub name: String,
    pub official_name: Option<String>,
    #[serde(rename = "type")]
    pub type_: AccountType,
    pub subtype: Option<String>,
    pub verification_status: Option<AccountVerificationStatus>,
    pub verification_insights: Option<AccountVerificationInsights>,
    pub persistent_account_id: String,
    pub holder_category: Option<AccountHolderCategory>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccountUpdateType {
    Background,
    UserPresentRequired,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccountItem {
    pub item_id: String,
    pub institution_id: Option<String>,
    pub webhook: Option<String>,
    pub error: Option<PlaidErrorResponse>,
    pub available_products: Vec<Product>,
    pub billed_products: Option<Vec<Product>>,
    pub consented_products: Option<Vec<Product>>,
    pub consent_expiration_time: Option<DateTime<Utc>>,
    pub updated_type: Option<AccountUpdateType>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AccountsGetResponse {
    pub accounts: Vec<Account>,
    pub item: AccountItem,
    pub request_id: String,
}
