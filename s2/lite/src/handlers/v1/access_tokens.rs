use axum::extract::{FromRequest, Path, Query, State};
use http::StatusCode;
use s2_api::{data::Json, v1 as v1t};
use s2_common::types::access::AccessTokenId;

use crate::{backend::Backend, handlers::v1::error::ServiceError};

pub fn router() -> axum::Router<Backend> {
    use axum::routing::{delete, get, post};
    axum::Router::new()
        .route(super::paths::access_tokens::LIST, get(list_access_tokens))
        .route(super::paths::access_tokens::ISSUE, post(issue_access_token))
        .route(
            super::paths::access_tokens::REVOKE,
            delete(revoke_access_token),
        )
}

#[derive(FromRequest)]
#[from_request(rejection(ServiceError))]
pub struct ListArgs {
    #[from_request(via(Query))]
    _request: v1t::access::ListAccessTokensRequest,
}

/// List access tokens.
pub async fn list_access_tokens(
    State(_backend): State<Backend>,
    ListArgs { .. }: ListArgs,
) -> Result<Json<v1t::access::ListAccessTokensResponse>, ServiceError> {
    Err(ServiceError::NotImplemented)
}

#[derive(FromRequest)]
#[from_request(rejection(ServiceError))]
pub struct IssueArgs {
    #[from_request(via(Json))]
    _request: v1t::access::AccessTokenInfo,
}

/// Issue a new access token.
pub async fn issue_access_token(
    State(_backend): State<Backend>,
    IssueArgs { .. }: IssueArgs,
) -> Result<(StatusCode, Json<v1t::access::IssueAccessTokenResponse>), ServiceError> {
    Err(ServiceError::NotImplemented)
}

#[derive(FromRequest)]
#[from_request(rejection(ServiceError))]
pub struct RevokeArgs {
    #[from_request(via(Path))]
    _id: AccessTokenId,
}

/// Revoke an access token.
pub async fn revoke_access_token(
    State(_backend): State<Backend>,
    RevokeArgs { .. }: RevokeArgs,
) -> Result<StatusCode, ServiceError> {
    Err(ServiceError::NotImplemented)
}
