use std::{collections::HashMap, fmt};

use chrono::{DateTime, Utc};
use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::{error::PlaidError, PlaidResult};

use super::common::Product;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Default)]
pub enum Language {
    #[serde(rename = "da")]
    Danish,
    #[serde(rename = "nl")]
    Dutch,
    #[default]
    #[serde(rename = "en")]
    English,
    #[serde(rename = "et")]
    Estonian,
    #[serde(rename = "fr")]
    French,
    #[serde(rename = "de")]
    German,
    #[serde(rename = "it")]
    Italian,
    #[serde(rename = "lv")]
    Latvian,
    #[serde(rename = "lt")]
    Lithuanian,
    #[serde(rename = "no")]
    Norwegian,
    #[serde(rename = "pl")]
    Polish,
    #[serde(rename = "pt")]
    Portuguese,
    #[serde(rename = "ro")]
    Romanian,
    #[serde(rename = "es")]
    Spanish,
    #[serde(rename = "sv")]
    Swedish,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Default)]
pub enum CountryCode {
    #[serde(rename = "BE")]
    Be,
    #[serde(rename = "CA")]
    Ca,
    #[serde(rename = "DE")]
    De,
    #[serde(rename = "DK")]
    Dk,
    #[serde(rename = "EE")]
    Ee,
    #[serde(rename = "ES")]
    Es,
    #[serde(rename = "FR")]
    Fr,
    #[serde(rename = "IE")]
    Ie,
    #[serde(rename = "IT")]
    It,
    #[serde(rename = "LT")]
    Lt,
    #[serde(rename = "LV")]
    Lv,
    #[serde(rename = "NL")]
    Nl,
    #[serde(rename = "NO")]
    No,
    #[serde(rename = "PL")]
    Pl,
    #[serde(rename = "PT")]
    Pt,
    #[serde(rename = "SE")]
    Se,
    #[serde(rename = "GB")]
    Gb,
    #[default]
    #[serde(rename = "US")]
    Us,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct UserName {
    pub given_name: String,
    pub family_name: String,
}

/// Struct representing a basic E.164 phone number
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct E164PhoneNumber(String);

impl E164PhoneNumber {
    pub fn new(phone_number: &str) -> PlaidResult<E164PhoneNumber> {
        let re = Regex::new(r"^\+[1-9]\d{1,14}$").expect("incorrect E164 phone number regex");
        if re.is_match(&phone_number) {
            Ok(E164PhoneNumber(phone_number.to_string()))
        } else {
            Err(PlaidError::InvalidPhoneNumber(phone_number.to_string()))
        }
    }
}

impl fmt::Display for E164PhoneNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Serialize for E164PhoneNumber {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Default)]
pub struct User {
    pub client_user_id: String,
    pub legal_name: Option<String>,
    pub name: Option<UserName>,
    pub phone_number: Option<E164PhoneNumber>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dob(String);

impl Dob {
    pub fn new(date_of_birth: &str) -> PlaidResult<Dob> {
        let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").expect("Invalid date of birth regex");
        if re.is_match(&date_of_birth) {
            Ok(Dob(date_of_birth.to_string()))
        } else {
            Err(PlaidError::InvalidDateOfBirth(date_of_birth.to_string()))
        }
    }
}

impl fmt::Display for Dob {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Serialize for Dob {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Address {
    pub city: Option<String>,
    // TODO:  ISO 3166-2 subdivision code
    pub region: Option<String>,
    // TODO
    pub postal_code: Option<String>,
    pub country: CountryCode,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Default)]
pub enum IdType {
    #[serde(rename = "ar_dni")]
    ArDni,
    #[serde(rename = "au_drivers_license")]
    AuDriversLicense,
    #[serde(rename = "au_passport")]
    AuPassport,
    #[serde(rename = "br_cpf")]
    BrCpf,
    #[serde(rename = "ca_sin")]
    CaSin,
    #[serde(rename = "cl_run")]
    ClRun,
    #[serde(rename = "cn_resident_card")]
    CnResidentCard,
    #[serde(rename = "co_nit")]
    CoNit,
    #[serde(rename = "dk_cpr")]
    DkCpr,
    #[serde(rename = "eg_national_id")]
    EgNationalId,
    #[serde(rename = "es_dni")]
    EsDni,
    #[serde(rename = "es_nie")]
    EsNie,
    #[serde(rename = "hk_hkid")]
    HkHkid,
    #[serde(rename = "in_pan")]
    InPan,
    #[serde(rename = "it_cf")]
    ItCf,
    #[serde(rename = "jo_civil_id")]
    JoCivilId,
    #[serde(rename = "jp_my_number")]
    JpMyNumber,
    #[serde(rename = "ke_hudum_namba")]
    KeHudumaNamba,
    #[serde(rename = "kw_civil_id")]
    KwCivilId,
    #[serde(rename = "mx_curp")]
    MxCurp,
    #[serde(rename = "mx_rfc")]
    MxRfc,
    #[serde(rename = "my_nric")]
    MyNric,
    #[serde(rename = "ng_nin")]
    NgNin,
    #[serde(rename = "nz_drivers_license")]
    NzDriversLicense,
    #[serde(rename = "om_civil_id")]
    OmCivilId,
    #[serde(rename = "ph_psn")]
    PhPsn,
    #[serde(rename = "pl_pesel")]
    PlPesel,
    #[serde(rename = "ro_cnp")]
    RoCnp,
    #[serde(rename = "sa_national_id")]
    SaNationalId,
    #[serde(rename = "se_pin")]
    SePin,
    #[serde(rename = "sg_nric")]
    SgNric,
    #[serde(rename = "tr_tc_kimlik")]
    TrTcKimlik,
    #[default]
    #[serde(rename = "us_ssn")]
    UsSsn,
    #[serde(rename = "us_ssn_last_4")]
    UsSsnLast4,
    #[serde(rename = "za_smart_id")]
    ZaSmartId,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Default)]
pub struct IdNumber {
    pub value: String,
    #[serde(rename = "type")]
    pub type_: IdType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AccountFilters {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Default)]
pub struct LinkTokenCreateRequest {
    pub client_id: String,
    pub secret: String,
    pub client_name: String,
    pub language: Language,
    pub country_codes: Vec<CountryCode>,
    pub user: User,
    // TODO - these cause issues if present (are not required)
    // pub email_address: Option<String>,
    // pub date_of_birth: Option<Dob>,
    // // pub address: Option<Address>,
    // pub id_number: Option<IdNumber>,
    pub products: Vec<Product>,
    pub required_if_supported_products: Option<Vec<Product>>, // TODO
    pub optional_products: Option<Vec<Product>>,              // TODO
    pub additional_consented_products: Option<Vec<Product>>,
    pub webhook: Option<String>,
    pub access_token: Option<String>,
    pub link_customization_name: Option<String>,
    pub redirect_uri: Option<String>,
    pub android_package_name: Option<String>,
    pub institution_data: Option<HashMap<String, String>>, // TODO
    pub account_filters: Option<AccountFilters>,           // TODO
                                                           // TODO - remaining fields
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct LinkTokenCreateResponse {
    pub expiration: DateTime<Utc>,
    pub link_token: String,
    pub request_id: String,
    pub hosted_link_url: Option<String>,
}
