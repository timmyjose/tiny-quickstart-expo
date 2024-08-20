use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Processor {
    Dwolla,
    Galileo,
    ModernTreasury,
    Ocrolus,
    Vesta,
    Drivewealth,
    Vopay,
    Achq,
    Check,
    Checkbook,
    Circle,
    SilaMoney,
    Rize,
    SvbApi,
    Unit,
    Wyre,
    Lithic,
    Alpaca,
    Astra,
    Moov,
    TreasuryPrime,
    Marqeta,
    Checkout,
    Solid,
    Highnote,
    Gemini,
    ApexClearing,
    Gusto,
    Adyen,
    Atomic,
    I2c,
    Wepay,
    Riskified,
    Utb,
    AdpRoll,
    FortressTrust,
    Bond,
    Bakkt,
    Teal,
    ZeroHash,
    TabaPay,
    Knot,
    Sardine,
    Alloy,
    Finix,
    Nuvei,
    Layer,
    Boom,
    Paynote,
    Stake,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProcessorTokenCreateRequest {
    pub client_id: String,
    pub secret: String,
    pub access_token: String,
    pub account_id: String,
    pub processor: Processor,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProcessorTokenCreateResponse {
    pub processor_token: String,
    pub request_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProcessorStripeBankAccountTokenCreateRequest {
    pub client_id: String,
    pub secret: String,
    pub access_token: String,
    pub account_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProcessorStripeBankAccountTokenCreateResponse {
    pub stripe_bank_account_account_token: String,
    pub request_id: String,
}
