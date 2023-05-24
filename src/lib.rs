mod error;
mod types;

#[cfg(feature = "staging")]
use serde_json::json;

pub use error::*;
use json_api_client::*;
pub use types::*;

pub struct Client {
    api: ApiClient,
}

impl Client {
    pub fn new(api_url: &str, api_token: &str) -> Result<Client> {
        let c = AuthorizationHeaderConfig {
            token: format!("Token {}", api_token),
        };

        let client = ApiClient::new(api_url, AuthConfig::AuthorizationHeader(c), None)?;

        Ok(Client { api: client })
    }

    async fn get<T>(&self, path: &str, query: Option<Queries<'_>>) -> Result<T>
    where
        T: JsonResponse,
    {
        self.api.get(path, query, None).await.map_err(Error::from)
    }

    async fn post<T>(&self, path: &str, data: Option<&serde_json::Value>) -> Result<T>
    where
        T: JsonResponse,
    {
        self.api.post(path, data, None).await.map_err(Error::from)
    }

    pub async fn list_users(&self) -> Result<Vec<User>> {
        self.get("users", None).await
    }

    pub async fn get_user(&self, user_id: UserId) -> Result<UserDetails> {
        let path = format!("users/{}", user_id);
        let query = [("legal_name", "legalcucc")];
        self.get(&path, Some(&query)).await
    }

    pub async fn list_verification_requests(&self, type_filter: ApiType) -> Result<Vec<VerificationRequest>> {
        let api_type = type_filter.to_string();
        let query = [("type", api_type.as_str())];
        self.get("verification_requests", Some(&query)).await
    }

    pub async fn get_user_verification_requests(&self, user_id: UserId) -> Result<Vec<VerificationRequest>> {
        let path = format!("users/{}/verification_requests", user_id);
        self.get(&path, None).await
    }

    pub async fn create_verification_request(&self, user_id: UserId, request: VerificationRequestInput) -> Result<VerificationRequestDetails> {
        let path = format!("users/{}/verification_requests", user_id);
        let data = serde_json::to_value(request)?;
        self.post(&path, Some(&data)).await
    }

    pub async fn get_verification_request(&self, request_id: VerificationRequestId) -> Result<VerificationRequestDetails> {
        let path = format!("verification_requests/{}", request_id);
        self.get(&path, None).await
    }

    pub async fn check_user_verification_request(&self, user_id: UserId, request_id: VerificationRequestId) -> Result<VerificationRequestDetails> {
        let path = format!("users/{}/verification_requests/{}", user_id, request_id);
        self.get(&path, None).await
    }

    // TODO KYC-320 stream PDF
    /*pub async fn download_user_certificate(&self, user_id: UserId, request_id: VerificationRequestId) -> Result<???> {
       let path = format!("users/{}/verification_requests/{}/certificate", user_id, request_id);
       self.get_stream(&path).await
    }

    pub async fn download_latest_user_certificate(&self, user_id: UserId) -> Result<???> {
       let path = format!("users/{}/verification_certificate", user_id);
       self.get_stream(&path).await
    }

    pub async fn download_certificate(&self, request_id: VerificationRequestId) -> Result<???> {
       let path = format!("/verification_requests/{}/certificate", request_id);
       self.get_stream(&path).await
    }
    */

    pub async fn fire_user_webhook(&self, user_id: UserId, request_id: VerificationRequestId) -> Result<FireWebhookResponse> {
        let path = format!("users/{}/verification_requests/{}/webhook", user_id, request_id);
        self.get(&path, None).await
    }

    pub async fn fire_webhook(&self, request_id: VerificationRequestId) -> Result<FireWebhookResponse> {
        let path = format!("verification_requests/{}/webhook", request_id);
        self.get(&path, None).await
    }

    #[cfg(feature = "staging")]
    pub async fn simulate_investor_completion(&self, user_id: UserId, request_id: VerificationRequestId) -> Result<InvestorCompletionResponse> {
        let path = format!("users/{}/verification_requests/{}/simulate_investor_completion", user_id, request_id);
        self.post(&path, None).await
    }

    #[cfg(feature = "staging")]
    pub async fn simulate_user_review(&self, user_id: UserId, request_id: VerificationRequestId, status: ReviewStatus) -> Result<ReviewResponse> {
        let path = format!("users/{}/verification_requests/{}/review", user_id, request_id);
        let data = json!({ "status": status });
        self.post(&path, Some(&data)).await
    }

    #[cfg(feature = "staging")]
    pub async fn simulate_review(&self, request_id: VerificationRequestId, status: ReviewStatus) -> Result<ReviewResponse> {
        let path = format!("verification_requests/{}/review", request_id);
        let data = json!({ "status": status });
        self.post(&path, Some(&data)).await
    }
}

#[cfg(test)]
mod tests {
    //use super::*;

    // put unittests here
}
