use axum::{
	extract::{Path, Query},
	response::IntoResponse,
	Extension, Json,
};

use super::{
	TestsCreateRequestDto, TestsItemDto, TestsResponseListDto, TestsService,
	TestsUpdateRequestDto,
};
use crate::{
	permissions_guard, AppState, MessageResponseDto, MetaRequestDto, PermissionsEnum,
	ResponseListSuccessDto, ResponseSuccessDto,
};

#[utoipa::path(
	get,
	security(("Bearer" = [])),
	path = "/v1/tests",
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
		(status = 200, description = "Get test list", body = ResponseListSuccessDto<Vec<TestsResponseListDto>>)
	),
	tag = "Tests"
)]
pub async fn get_test_list(
	headers: axum::http::HeaderMap,
	Extension(state): Extension<AppState>,
	Query(meta): Query<MetaRequestDto>,
) -> impl IntoResponse {
	match permissions_guard(
		&headers,
		state.clone(),
		vec![PermissionsEnum::ReadListTests],
	)
	.await
	{
		Ok(_) => TestsService::get_test_list(&state, meta).await,
		Err(response) => response,
	}
}

#[utoipa::path(
	get,
	security(("Bearer" = [])),
	path = "/v1/tests/detail/{id}",
	params(("id" = String, Path, description = "Test ID")),
	responses(
		(status = 200, description = "Get test by ID", body = ResponseSuccessDto<TestsItemDto>)
	),
	tag = "Tests"
)]
pub async fn get_test_by_id(
	headers: axum::http::HeaderMap,
	Extension(state): Extension<AppState>,
	Path(id): Path<String>,
) -> impl IntoResponse {
	match permissions_guard(
		&headers,
		state.clone(),
		vec![PermissionsEnum::ReadDetailTests],
	)
	.await
	{
		Ok(_) => TestsService::get_test_by_id(&state, id).await,
		Err(response) => response,
	}
}

#[utoipa::path(
	post,
	security(("Bearer" = [])),
	path = "/v1/tests/create",
	request_body = TestsCreateRequestDto,
	responses(
		(status = 201, description = "Create new test", body = MessageResponseDto)
	),
	tag = "Tests"
)]
pub async fn post_create_test(
	headers: axum::http::HeaderMap,
	Extension(state): Extension<AppState>,
	Json(payload): Json<TestsCreateRequestDto>,
) -> impl IntoResponse {
	match permissions_guard(
		&headers,
		state.clone(),
		vec![PermissionsEnum::CreateTests],
	)
	.await
	{
		Ok(_) => TestsService::create_test(&state, payload).await,
		Err(response) => response,
	}
}

#[utoipa::path(
	put,
	security(("Bearer" = [])),
	path = "/v1/tests/update/{id}",
	request_body = TestsUpdateRequestDto,
	responses(
		(status = 200, description = "Update test", body = MessageResponseDto)
	),
	tag = "Tests"
)]
pub async fn put_update_test(
	headers: axum::http::HeaderMap,
	Extension(state): Extension<AppState>,
	Path(id): Path<String>,
	Json(payload): Json<TestsUpdateRequestDto>,
) -> impl IntoResponse {
	match permissions_guard(
		&headers,
		state.clone(),
		vec![PermissionsEnum::UpdateTests],
	)
	.await
	{
		Ok(_) => TestsService::update_test(&state, id, payload).await,
		Err(response) => response,
	}
}

#[utoipa::path(
	delete,
	security(("Bearer" = [])),
	path = "/v1/tests/delete/{id}",
	responses(
		(status = 200, description = "Delete test", body = MessageResponseDto)
	),
	tag = "Tests"
)]
pub async fn delete_test(
	headers: axum::http::HeaderMap,
	Extension(state): Extension<AppState>,
	Path(id): Path<String>,
) -> impl IntoResponse {
	match permissions_guard(
		&headers,
		state.clone(),
		vec![PermissionsEnum::DeleteTests],
	)
	.await
	{
		Ok(_) => TestsService::delete_test(&state, id).await,
		Err(response) => response,
	}
}
