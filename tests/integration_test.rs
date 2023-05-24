use test_log::test;
use verify_investor_client::*;

const STAGING_TOKEN: &str = "REPLACE_ME";
const STAGING_URL: &str = "https://verifyinvestor-staging.herokuapp.com/api/v1/";

const USER_ID: UserId = 32099;

fn get_client() -> Client {
    Client::new(STAGING_URL, STAGING_TOKEN).unwrap()
}

#[test(tokio::test)]
#[ignore]
async fn list_users() {
    let resp = get_client().list_users().await;
    println!("Resp: {:#?}", resp);
    assert!(resp.is_ok());
}

#[test(tokio::test)]
#[ignore]
async fn get_user() {
    let resp = get_client().get_user(32099).await;
    println!("Resp: {:#?}", resp);
    assert!(resp.is_ok());
}

#[test(tokio::test)]
#[ignore]
async fn list_verification_requests() {
    let resp = get_client().list_verification_requests(ApiType::All).await;
    println!("Resp: {:#?}", resp);
    assert!(resp.is_ok());

    if let Ok(vrs) = resp {
        println!("Expires: {:?}", vrs[0].verified_expires_at);
    }
}

#[test(tokio::test)]
#[ignore]
async fn get_user_verification_requests() {
    let resp = get_client().get_user_verification_requests(32147).await;
    println!("Resp: {:#?}", resp);
    assert!(resp.is_ok());
}

#[test(tokio::test)]
#[ignore]
async fn create_verification_request() {
    let resp = get_client().create_verification_request(USER_ID, VerificationRequestInput::default()).await;
    println!("Resp: {:#?}", resp);
    assert!(resp.is_ok());
}

#[test(tokio::test)]
#[ignore]
async fn get_verification_request() {
    let resp = get_client().get_verification_request(34874).await;
    println!("Resp: {:#?}", resp);
    assert!(resp.is_ok());
}

#[test(tokio::test)]
#[ignore]
async fn check_user_verification_request() {
    let resp = get_client().check_user_verification_request(USER_ID, 34675).await;
    println!("Resp: {:#?}", resp);
    assert!(resp.is_ok());
}

#[cfg(feature = "staging")]
#[test(tokio::test)]
#[ignore]
async fn verification_flow() {
    let vr = get_client()
        .create_verification_request(USER_ID, VerificationRequestInput::default())
        .await
        .unwrap();
    let vr_id = vr.base.id;

    assert_eq!(vr.investor.id, USER_ID);
    assert_eq!(vr.status, None);

    let vr = get_client().check_user_verification_request(USER_ID, vr_id).await.unwrap();
    assert_eq!(vr.status, Some(VerificationRequestStatus::AcceptedByInvestor));

    let complete_resp = get_client().simulate_investor_completion(USER_ID, vr_id).await;
    println!("Complete resp: {:?}", complete_resp);
    assert!(complete_resp.is_ok());

    let vr = get_client().check_user_verification_request(USER_ID, vr_id).await.unwrap();
    assert_eq!(vr.status, Some(VerificationRequestStatus::WaitingForReview));

    let review_resp = get_client().simulate_user_review(USER_ID, vr_id, ReviewStatus::Accredited).await;
    println!("Review resp: {:?}", review_resp);
    assert!(review_resp.is_ok());

    let vr = get_client().check_user_verification_request(USER_ID, vr_id).await.unwrap();
    assert_eq!(vr.status, Some(VerificationRequestStatus::Accredited));
}

#[cfg(feature = "staging")]
#[test(tokio::test)]
#[ignore]
async fn simulate_investor_completion() {
    let resp = get_client().simulate_investor_completion(USER_ID, 34626).await;
    println!("Resp: {:?}", resp);
    assert!(resp.is_ok());
}

#[cfg(feature = "staging")]
#[test(tokio::test)]
#[ignore]
async fn simulate_user_review() {
    let resp = get_client().simulate_user_review(USER_ID, 34874, ReviewStatus::Accredited).await;
    println!("Resp: {:?}", resp);
    assert!(resp.is_ok());
}

#[cfg(feature = "staging")]
#[test(tokio::test)]
#[ignore]
async fn simulate_review() {
    let status = ReviewStatus::Accredited;
    //let status = ReviewStatus::NotAccredited;
    //let status = ReviewStatus::NeedMoreInformation;
    let resp = get_client().simulate_review(34877, status).await;
    println!("Resp: {:?}", resp);
    assert!(resp.is_ok());
}

#[test(tokio::test)]
#[ignore]
async fn fire_user_webhook() {
    let resp = get_client().fire_user_webhook(32394, 34874).await;
    println!("Resp: {:#?}", resp);
    assert!(resp.is_ok());
}
