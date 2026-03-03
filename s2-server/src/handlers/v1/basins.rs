use axum::extract::{FromRequest, Path, Query, State};
use http::StatusCode;
use s2_api::{data::Json, v1 as v1t};
use s2_common::{
    http::extract::HeaderOpt,
    types::{
        basin::{BasinName, ListBasinsRequest},
        config::BasinConfig,
        resources::{CreateMode, Page, RequestToken},
    },
};

use crate::{auth::Principal, backend::Backend, handlers::v1::error::ServiceError};

pub fn router() -> axum::Router<Backend> {
    use axum::routing::{delete, get, post};
    axum::Router::new()
        .route(super::paths::basins::LIST, get(list_basins))
        .route(super::paths::basins::CREATE, post(create_basin))
        .route(super::paths::basins::DELETE, delete(delete_basin))
}

#[derive(FromRequest)]
#[from_request(rejection(ServiceError))]
pub struct ListArgs {
    #[from_request(via(Query))]
    request: v1t::basin::ListBasinsRequest,
}

/// List basins.
pub async fn list_basins(
    _auth: Principal,
    State(backend): State<Backend>,
    ListArgs { request }: ListArgs,
) -> Result<Json<v1t::basin::ListBasinsResponse>, ServiceError> {
    let request: ListBasinsRequest = request.try_into()?;
    let Page { values, has_more } = backend.list_basins(request).await?;
    Ok(Json(v1t::basin::ListBasinsResponse {
        basins: values.into_iter().map(Into::into).collect(),
        has_more,
    }))
}

#[derive(FromRequest)]
#[from_request(rejection(ServiceError))]
pub struct CreateArgs {
    request_token: HeaderOpt<RequestToken>,
    #[from_request(via(Json))]
    request: v1t::basin::CreateBasinRequest,
}

/// Create a basin.
pub async fn create_basin(
    _auth: Principal,
    State(backend): State<Backend>,
    CreateArgs {
        request_token: HeaderOpt(request_token),
        request,
    }: CreateArgs,
) -> Result<(StatusCode, Json<v1t::basin::BasinInfo>), ServiceError> {
    let config: BasinConfig = request
        .config
        .map(TryInto::try_into)
        .transpose()?
        .unwrap_or_default();
    let info = backend
        .create_basin(request.basin, config, CreateMode::CreateOnly(request_token))
        .await?;
    Ok((StatusCode::CREATED, Json(info.into_inner().into())))
}

#[derive(FromRequest)]
#[from_request(rejection(ServiceError))]
pub struct DeleteArgs {
    #[from_request(via(Path))]
    basin: BasinName,
}

/// Delete a basin.
pub async fn delete_basin(
    _auth: Principal,
    State(backend): State<Backend>,
    DeleteArgs { basin }: DeleteArgs,
) -> Result<StatusCode, ServiceError> {
    backend.delete_basin(basin).await?;
    Ok(StatusCode::ACCEPTED)
}
