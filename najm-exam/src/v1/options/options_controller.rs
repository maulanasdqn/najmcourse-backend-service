use super::{
	OptionsCreateRequestDto, OptionsItemDto, OptionsResponseListDto, OptionsService,
	OptionsUpdateRequestDto,
};
use axum::{
	Extension, Json,
	extract::{Path, Query},
	response::IntoResponse,
};
use najm_iam::{PermissionsEnum, permissions_guard};
use najm_lib::{
	AppState, MessageResponseDto, MetaRequestDto, ResponseListSuccessDto,
	ResponseSuccessDto,
};

#[utoipa::path(
	get,
	security(
		("Bearer" = [])
	),
	path = "/v1/options",
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
		(status = 200, description = "Get option list", body = ResponseListSuccessDto<Vec<OptionsResponseListDto>>)
	),
	tag = "Options"
)]
pub async fn get_option_list(
	headers: axum::http::HeaderMap,
	Extension(state): Extension<AppState>,
	Query(meta): Query<MetaRequestDto>,
) -> impl IntoResponse {
	match permissions_guard(
		&headers,
		state.clone(),
		vec![PermissionsEnum::ReadListOptions],
	)
	.await
	{
		Ok(_) => OptionsService::get_option_list(&state, meta).await,
		Err(response) => response,
	}
}

#[utoipa::path(
	get,
	security(
		("Bearer" = [])
	),
	path = "/v1/options/detail/{id}",
	params(("id" = String, Path, description = "Option ID")),
	responses(
		(status = 200, description = "Get option by ID", body = ResponseSuccessDto<OptionsItemDto>)
	),
	tag = "Options"
)]
pub async fn get_option_by_id(
	headers: axum::http::HeaderMap,
	Extension(state): Extension<AppState>,
	Path(id): Path<String>,
) -> impl IntoResponse {
	match permissions_guard(
		&headers,
		state.clone(),
		vec![PermissionsEnum::ReadDetailOptions],
	)
	.await
	{
		Ok(_) => OptionsService::get_option_by_id(&state, id).await,
		Err(response) => response,
	}
}

#[utoipa::path(
	post,
	security(
		("Bearer" = [])
	),
	path = "/v1/options/create",
	request_body = OptionsCreateRequestDto,
	responses(
		(status = 201, description = "Create new option", body = MessageResponseDto)
	),
	tag = "Options"
)]
pub async fn post_create_option(
	headers: axum::http::HeaderMap,
	Extension(state): Extension<AppState>,
	Json(payload): Json<OptionsCreateRequestDto>,
) -> impl IntoResponse {
	match permissions_guard(
		&headers,
		state.clone(),
		vec![PermissionsEnum::CreateOptions],
	)
	.await
	{
		Ok(_) => OptionsService::create_option(&state, payload).await,
		Err(response) => response,
	}
}

#[utoipa::path(
	put,
	security(
		("Bearer" = [])
	),
	path = "/v1/options/update/{id}",
	request_body = OptionsUpdateRequestDto,
	responses(
		(status = 200, description = "Update option", body = MessageResponseDto)
	),
	tag = "Options"
)]
pub async fn put_update_option(
	headers: axum::http::HeaderMap,
	Extension(state): Extension<AppState>,
	Path(id): Path<String>,
	Json(payload): Json<OptionsUpdateRequestDto>,
) -> impl IntoResponse {
	match permissions_guard(
		&headers,
		state.clone(),
		vec![PermissionsEnum::UpdateOptions],
	)
	.await
	{
		Ok(_) => OptionsService::update_option(&state, id, payload).await,
		Err(response) => response,
	}
}

#[utoipa::path(
	delete,
	security(
		("Bearer" = [])
	),
	path = "/v1/options/delete/{id}",
	responses(
		(status = 200, description = "Delete option", body = MessageResponseDto)
	),
	tag = "Options"
)]
pub async fn delete_option(
	headers: axum::http::HeaderMap,
	Extension(state): Extension<AppState>,
	Path(id): Path<String>,
) -> impl IntoResponse {
	match permissions_guard(
		&headers,
		state.clone(),
		vec![PermissionsEnum::DeleteOptions],
	)
	.await
	{
		Ok(_) => OptionsService::delete_option(&state, id).await,
		Err(response) => response,
	}
}
