use super::{
	QuestionsCreateRequestDto, QuestionsRepository, QuestionsSchema,
	QuestionsUpdateRequestDto,
};
use crate::ResourceEnum;
use crate::{
	AppState, MetaRequestDto, ResponseListSuccessDto, ResponseSuccessDto,
	common_response, success_list_response, success_response, validate_request,
};
use axum::{http::StatusCode, response::Response};
use najm_iam::make_thing;
use surrealdb::Uuid;

pub struct QuestionsService;

impl QuestionsService {
	pub async fn get_question_list(
		state: &AppState,
		meta: MetaRequestDto,
	) -> Response {
		let repo = QuestionsRepository::new(state);
		match repo.query_question_list(meta).await {
			Ok(data) => {
				let response = ResponseListSuccessDto {
					data: data.data,
					meta: data.meta,
				};
				success_list_response(response)
			}
			Err(e) => common_response(StatusCode::BAD_REQUEST, &e.to_string()),
		}
	}

	pub async fn get_question_by_id(state: &AppState, id: String) -> Response {
		let repo = QuestionsRepository::new(state);
		match repo.query_question_by_id(&id.clone()).await {
			Ok(question) => success_response(ResponseSuccessDto { data: question }),
			Err(e) => common_response(StatusCode::NOT_FOUND, &e.to_string()),
		}
	}

	pub async fn create_question(
		state: &AppState,
		payload: QuestionsCreateRequestDto,
	) -> Response {
		if let Err((status, message)) = validate_request(&payload) {
			return common_response(status, &message);
		}
		let repo = QuestionsRepository::new(state);
		match repo
			.query_create_question(QuestionsSchema::create(
				payload,
				vec![make_thing(
					&ResourceEnum::Options.to_string(),
					&Uuid::new_v4().to_string(),
				)],
			))
			.await
		{
			Ok(msg) => common_response(StatusCode::CREATED, &msg),
			Err(e) => common_response(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()),
		}
	}

	pub async fn update_question(
		state: &AppState,
		id: String,
		payload: QuestionsUpdateRequestDto,
	) -> Response {
		if let Err((status, message)) = validate_request(&payload) {
			return common_response(status, &message);
		}
		let repo = QuestionsRepository::new(state);
		let updated_question = QuestionsSchema::update(payload, id);
		match repo.query_update_question(updated_question).await {
			Ok(msg) => common_response(StatusCode::OK, &msg),
			Err(e) => common_response(StatusCode::BAD_REQUEST, &e.to_string()),
		}
	}

	pub async fn delete_question(state: &AppState, id: String) -> Response {
		let repo = QuestionsRepository::new(state);
		if repo.query_question_by_id(&id).await.is_err() {
			return common_response(StatusCode::BAD_REQUEST, "Question not found");
		}
		match repo.query_delete_question(id).await {
			Ok(msg) => common_response(StatusCode::OK, &msg),
			Err(e) => common_response(StatusCode::BAD_REQUEST, &e.to_string()),
		}
	}
}
