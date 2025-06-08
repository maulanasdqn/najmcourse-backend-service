use super::{
	SessionsCreateRequestDto, SessionsDetailResponseDto, SessionsResponseDto,
	SessionsService, SessionsUpdateRequestDto,
};
use crate::{
	permissions_guard, AppState, MessageResponseDto, MetaRequestDto, PermissionsEnum,
	ResponseListSuccessDto, ResponseSuccessDto,
};
use axum::{
	extract::{Path, Query},
	response::IntoResponse,
	Extension, Json,
};

#[utoipa::path(
	get,
	security(("Bearer" = [])),
	path = "/v1/sessions",
	params(
		("page" = Option<i64>, Query, description = "Page number"),
		("per_page" = Option<i64>, Query, description = "Items per page"),
		("search" = Option<String>, Query, description = "Search keyword"),
		("sort_by" = Option<String>, Query, description = "Sort by field"),
		("order" = Option<String>, Query, description = "Order ASC or DESC"),
		("filter" = Option<String>, Query, description = "Filter value"),
		("filter_by" = Option<String>, Query, description = "Field to filter by"),
	),
	responses(
		(status = 200, description = "Get session list", body = ResponseListSuccessDto<Vec<SessionsResponseDto>>)
	),
	tag = "Sessions"
)]
pub async fn get_session_list(
	headers: axum::http::HeaderMap,
	Extension(state): Extension<AppState>,
	Query(meta): Query<MetaRequestDto>,
) -> impl IntoResponse {
	match permissions_guard(
		&headers,
		state.clone(),
		vec![PermissionsEnum::ReadListSessions],
	)
	.await
	{
		Ok(_) => SessionsService::get_session_list(&state, meta).await,
		Err(response) => response,
	}
}

#[utoipa::path(
	get,
	security(("Bearer" = [])),
	path = "/v1/sessions/detail/{id}",
	params(("id" = String, Path, description = "Session ID")),
	responses(
		(status = 200, description = "Get session detail", body = ResponseSuccessDto<SessionsDetailResponseDto>)
	),
	tag = "Sessions"
)]
pub async fn get_session_by_id(
	headers: axum::http::HeaderMap,
	Extension(state): Extension<AppState>,
	Path(id): Path<String>,
) -> impl IntoResponse {
	match permissions_guard(
		&headers,
		state.clone(),
		vec![PermissionsEnum::ReadDetailSessions],
	)
	.await
	{
		Ok(_) => SessionsService::get_session_by_id(&state, id).await,
		Err(response) => response,
	}
}

#[utoipa::path(
	post,
	security(("Bearer" = [])),
	path = "/v1/sessions/create",
	request_body = SessionsCreateRequestDto,
	responses(
		(status = 201, description = "Create new session", body = MessageResponseDto)
	),
	tag = "Sessions"
)]
pub async fn post_create_session(
	headers: axum::http::HeaderMap,
	Extension(state): Extension<AppState>,
	Json(payload): Json<SessionsCreateRequestDto>,
) -> impl IntoResponse {
	match permissions_guard(
		&headers,
		state.clone(),
		vec![PermissionsEnum::CreateSessions],
	)
	.await
	{
		Ok(_) => SessionsService::create_session(&state, payload).await,
		Err(response) => response,
	}
}

#[utoipa::path(
	put,
	security(("Bearer" = [])),
	path = "/v1/sessions/update/{id}",
	request_body = SessionsUpdateRequestDto,
	responses(
		(status = 200, description = "Update session", body = MessageResponseDto)
	),
	tag = "Sessions"
)]
pub async fn put_update_session(
	headers: axum::http::HeaderMap,
	Extension(state): Extension<AppState>,
	Path(id): Path<String>,
	Json(payload): Json<SessionsUpdateRequestDto>,
) -> impl IntoResponse {
	match permissions_guard(
		&headers,
		state.clone(),
		vec![PermissionsEnum::UpdateSessions],
	)
	.await
	{
		Ok(_) => SessionsService::update_session(&state, id, payload).await,
		Err(response) => response,
	}
}

#[utoipa::path(
	delete,
	security(("Bearer" = [])),
	path = "/v1/sessions/delete/{id}",
	responses(
		(status = 200, description = "Delete session", body = MessageResponseDto)
	),
	tag = "Sessions"
)]
pub async fn delete_session(
	headers: axum::http::HeaderMap,
	Extension(state): Extension<AppState>,
	Path(id): Path<String>,
) -> impl IntoResponse {
	match permissions_guard(
		&headers,
		state.clone(),
		vec![PermissionsEnum::DeleteSessions],
	)
	.await
	{
		Ok(_) => SessionsService::delete_session(&state, id).await,
		Err(response) => response,
	}
}
