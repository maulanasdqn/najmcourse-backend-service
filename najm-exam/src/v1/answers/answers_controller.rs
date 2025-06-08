use super::{AnswersCreateRequestDto, AnswersService};
use crate::{
	answers::TestsItemAnswersDto, permissions_guard, AppState, MessageResponseDto,
	PermissionsEnum, ResponseSuccessDto,
};
use axum::{extract::Path, response::IntoResponse, Extension, Json};

#[utoipa::path(
	get,
	security(
		("Bearer" = [])
	),
	path = "/v1/answers/detail/{id}",
	params(("id" = String, Path, description = "Answer ID")),
	responses(
		(status = 200, description = "Get answer by ID", body = ResponseSuccessDto<TestsItemAnswersDto>)
	),
	tag = "Answers"
)]
pub async fn get_answer_by_id(
	headers: axum::http::HeaderMap,
	Extension(state): Extension<AppState>,
	Path(id): Path<String>,
) -> impl IntoResponse {
	match permissions_guard(
		&headers,
		state.clone(),
		vec![PermissionsEnum::ReadDetailAnswers],
	)
	.await
	{
		Ok(_) => AnswersService::get_answer_by_id(&state, id).await,
		Err(response) => response,
	}
}

#[utoipa::path(
	get,
	security(
		("Bearer" = [])
	),
	path = "/v1/answers/detail/{testId}/{userId}",
	params(("testId" = String, Path, description = "Test ID"), ("userId" = String, Path, description = "User ID")),
	responses(
		(status = 200, description = "Get answer by Test ID and User ID", body = ResponseSuccessDto<TestsItemAnswersDto>)
	),
	tag = "Answers"
)]
pub async fn get_answer_by_test_and_user_id(
	headers: axum::http::HeaderMap,
	Extension(state): Extension<AppState>,
	Path((test_id, user_id)): Path<(String, String)>,
) -> impl IntoResponse {
	match permissions_guard(
		&headers,
		state.clone(),
		vec![PermissionsEnum::ReadDetailAnswers],
	)
	.await
	{
		Ok(_) => {
			AnswersService::get_answer_by_test_and_user_id(&state, test_id, user_id).await
		}
		Err(response) => response,
	}
}

#[utoipa::path(
	post,
	security(
		("Bearer" = [])
	),
	path = "/v1/answers/create",
	request_body = AnswersCreateRequestDto,
	responses(
		(status = 201, description = "Create new answer", body = ResponseSuccessDto<TestsItemAnswersDto>),
	),
	tag = "Answers"
)]
pub async fn post_create_answer(
	headers: axum::http::HeaderMap,
	Extension(state): Extension<AppState>,
	Json(payload): Json<AnswersCreateRequestDto>,
) -> impl IntoResponse {
	match permissions_guard(
		&headers,
		state.clone(),
		vec![PermissionsEnum::CreateAnswers],
	)
	.await
	{
		Ok(_) => AnswersService::create_answer(&state, payload).await,
		Err(response) => response,
	}
}

#[utoipa::path(
	delete,
	security(
		("Bearer" = [])
	),
	path = "/v1/answers/delete/{id}",
	responses(
		(status = 200, description = "Delete answer", body = MessageResponseDto)
	),
	tag = "Answers"
)]
pub async fn delete_answer(
	headers: axum::http::HeaderMap,
	Extension(state): Extension<AppState>,
	Path(id): Path<String>,
) -> impl IntoResponse {
	match permissions_guard(
		&headers,
		state.clone(),
		vec![PermissionsEnum::DeleteAnswers],
	)
	.await
	{
		Ok(_) => AnswersService::delete_answer(&state, id).await,
		Err(response) => response,
	}
}
