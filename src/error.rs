use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("API error: '{0}'")]
    ApiError(String), // TODO KYC-136 make this an enum

    #[error(transparent)]
    ClientError(#[from] json_api_client::error::Error),

    #[error(transparent)]
    JsonError(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
