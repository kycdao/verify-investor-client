use json_api_client::types::*;
use serde::{Deserialize, Serialize};
use strum::Display;

pub type UserId = u64;
pub type VerificationRequestId = u64;

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: UserId,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthorizedParty {
    pub name: Option<String>,
    pub identifier: String,
    pub authorized_legal_names: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LegalProfile {
    pub name: String,
    pub legal_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserDetails {
    #[serde(flatten)]
    pub base: User,
    pub verified_on: Option<Date>,
    pub identifier: String,
    pub authorized_parties: Vec<AuthorizedParty>,
    pub legal_profile: LegalProfile,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Investor {
    pub id: UserId,
    #[deprecated]
    pub verification_status: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum VerificationRequestStatus {
    /// The verification is ready and waiting for the investor to accept it.
    WaitingForInvestorAcceptance,
    /// The investor has accepted the verification request but has not yet completed it.
    AcceptedByInvestor,
    /// Investor has completed the request, and it is now in the reviewers' queue.
    WaitingForReview,
    /// The verification request has been assigned a reviewer and is under review.
    InReview,
    /// The investor is verified as accredited.
    Accredited,
    /// After review, it appears the investor is not accredited.
    NotAccredited,
    /// The reviewer has requested additional information from the investor.
    WaitingForInformationFromInvestor,
    /// The verification request has expired. The investor accepted but did not complete.
    AcceptedExpire,
    /// The verification request has expired. The investor never accepted.
    DeclinedExpire,
    /// The investor has declined the verification request.
    DeclinedByInvestor,
    /// The investor has accepted then canceled the verification request.
    SelfNotAccredited,
}

#[derive(Display, Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ApiType {
    Regular,
    Embedded,
    Lite,
    All,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VerificationRequest {
    pub id: VerificationRequestId,
    pub waiting_for_info: Option<bool>, // not present in check_verification_request response
    pub portal_name: Option<String>,
    pub verified_expires_at: Option<Date>,
    pub deal_name: Option<String>,
    pub api_type: Option<ApiType>,  // not present in check_user_verification_request response
    pub identifier: Option<String>, // not present in check_user_verification_request response
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VerificationRequestDetails {
    #[serde(flatten)]
    pub base: VerificationRequest,
    pub status: Option<VerificationRequestStatus>, // not present in create_verification_request response
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: DateTime,
    #[serde(with = "time::serde::rfc3339::option")]
    pub completed_at: Option<DateTime>,
    pub redirect_url: Option<String>,
    pub webhook_url: Option<String>,
    pub investor_url: Option<String>,
    pub investor: Investor,
    pub legal_name: Option<String>,
    //#[deprecated]
    // verification_request_step
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct VerificationRequestInput {
    pub portal_name: Option<String>,
    pub deal_name: Option<String>,
    pub legal_name: Option<String>,
    pub redirect_url: Option<String>,
    pub webhook_url: Option<String>,
    pub issuer_email: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InvestorCompletionResponse {
    pub id: VerificationRequestId,
    pub investor: Investor,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FireWebhookResponse {
    pub meta: String,
    pub webhook_body: Option<String>,
    // missing fields compared to API doc (action, verification_request_id, investor_id etc.)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReviewResponse {
    pub id: VerificationRequestId,
    pub status: VerificationRequestStatus,
    pub legal_name: String,
    pub investor: Investor,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ReviewStatus {
    Accredited,
    NotAccredited,
    NeedMoreInformation,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum WebhookAction {
    CreateVerificationRequest,
    VerificationResult,
    // what else?
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WebhookData {
    pub action: WebhookAction,
    pub eapi_identifier: Option<String>,
    pub embedded_api: bool,
    pub identifier: Option<String>,
    pub investor_id: u64,
    pub legal_name: String,
    pub status: VerificationRequestStatus,
    pub verification_request_id: u64,
}
