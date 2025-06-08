use axum::{
	extract::{Path, Query},
	response::IntoResponse,
	Extension, Json,
};

use super::{
	QuestionsCreateRequestDto, QuestionsItemDto, QuestionsResponseListDto,
	QuestionsService, QuestionsUpdateRequestDto,
};
use crate::{
	permissions_guard, AppState, MessageResponseDto, MetaRequestDto, PermissionsEnum,
	ResponseListSuccessDto, ResponseSuccessDto,
};

#[utoipa::path(
	get,
	security(
		("Bearer" = [])
	),
	path = "/v1/questions",
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
		(status = 200, description = "Get question list", body = ResponseListSuccessDto<Vec<QuestionsResponseListDto>>)
	),
	tag = "Questions"
)]
pub async fn get_question_list(
	headers: axum::http::HeaderMap,
	Extension(state): Extension<AppState>,
	Query(meta): Query<MetaRequestDto>,
) -> impl IntoResponse {
	match permissions_guard(
		&headers,
		state.clone(),
		vec![PermissionsEnum::ReadListQuestions],
	)
	.await
	{
		Ok(_) => QuestionsService::get_question_list(&state, meta).await,
		Err(response) => response,
	}
}

#[utoipa::path(
	get,
	security(
		("Bearer" = [])
	),
	path = "/v1/questions/detail/{id}",
	params(("id" = String, Path, description = "Question ID")),
	responses(
		(status = 200, description = "Get question by ID", body = ResponseSuccessDto<QuestionsItemDto>)
	),
	tag = "Questions"
)]
pub async fn get_question_by_id(
	headers: axum::http::HeaderMap,
	Extension(state): Extension<AppState>,
	Path(id): Path<String>,
) -> impl IntoResponse {
	match permissions_guard(
		&headers,
		state.clone(),
		vec![PermissionsEnum::ReadDetailQuestions],
	)
	.await
	{
		Ok(_) => QuestionsService::get_question_by_id(&state, id).await,
		Err(response) => response,
	}
}

#[utoipa::path(
	post,
	security(
		("Bearer" = [])
	),
	path = "/v1/questions/create",
	request_body = QuestionsCreateRequestDto,
	responses(
		(status = 201, description = "Create new question", body = MessageResponseDto)
	),
	tag = "Questions"
)]
pub async fn post_create_question(
	headers: axum::http::HeaderMap,
	Extension(state): Extension<AppState>,
	Json(payload): Json<QuestionsCreateRequestDto>,
) -> impl IntoResponse {
	match permissions_guard(
		&headers,
		state.clone(),
		vec![PermissionsEnum::CreateQuestions],
	)
	.await
	{
		Ok(_) => QuestionsService::create_question(&state, payload).await,
		Err(response) => response,
	}
}

#[utoipa::path(
	put,
	security(
		("Bearer" = [])
	),
	path = "/v1/questions/update/{id}",
	request_body = QuestionsUpdateRequestDto,
	responses(
		(status = 200, description = "Update question", body = MessageResponseDto)
	),
	tag = "Questions"
)]
pub async fn put_update_question(
	headers: axum::http::HeaderMap,
	Extension(state): Extension<AppState>,
	Path(id): Path<String>,
	Json(payload): Json<QuestionsUpdateRequestDto>,
) -> impl IntoResponse {
	match permissions_guard(
		&headers,
		state.clone(),
		vec![PermissionsEnum::UpdateQuestions],
	)
	.await
	{
		Ok(_) => QuestionsService::update_question(&state, id, payload).await,
		Err(response) => response,
	}
}

#[utoipa::path(
	delete,
	security(
		("Bearer" = [])
	),
	path = "/v1/questions/delete/{id}",
	responses(
		(status = 200, description = "Delete question", body = MessageResponseDto)
	),
	tag = "Questions"
)]
pub async fn delete_question(
	headers: axum::http::HeaderMap,
	Extension(state): Extension<AppState>,
	Path(id): Path<String>,
) -> impl IntoResponse {
	match permissions_guard(
		&headers,
		state.clone(),
		vec![PermissionsEnum::DeleteQuestions],
	)
	.await
	{
		Ok(_) => QuestionsService::delete_question(&state, id).await,
		Err(response) => response,
	}
}
