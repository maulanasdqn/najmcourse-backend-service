use super::{
	SessionsCreateRequestDto, SessionsRepository, SessionsSchema,
	SessionsUpdateRequestDto,
};
use crate::{
	AppState, MetaRequestDto, ResponseListSuccessDto, ResponseSuccessDto,
	common_response, success_list_response, success_response, validate_request,
};
use axum::{http::StatusCode, response::Response};

pub struct SessionsService;

impl SessionsService {
	pub async fn get_session_list(state: &AppState, meta: MetaRequestDto) -> Response {
		let repo = SessionsRepository::new(state);
		match repo.query_session_list(meta).await {
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

	pub async fn get_session_by_id(state: &AppState, id: String) -> Response {
		let repo = SessionsRepository::new(state);
		match repo.query_session_by_id(id).await {
			Ok(data) => success_response(ResponseSuccessDto { data }),
			Err(e) => {
				let status = if e.to_string().contains("not found") {
					StatusCode::NOT_FOUND
				} else {
					StatusCode::INTERNAL_SERVER_ERROR
				};
				common_response(status, &e.to_string())
			}
		}
	}

	pub async fn create_session(
		state: &AppState,
		payload: SessionsCreateRequestDto,
	) -> Response {
		if let Err((status, message)) = validate_request(&payload) {
			return common_response(status, &message);
		}

		// Validate that tests are not empty
		if payload.tests.is_empty() {
			return common_response(StatusCode::BAD_REQUEST, "Tests must not be empty");
		}

		let repo = SessionsRepository::new(state);
		match repo
			.query_create_session(SessionsSchema::create(payload))
			.await
		{
			Ok(msg) => common_response(StatusCode::CREATED, &msg),
			Err(e) => common_response(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()),
		}
	}

	pub async fn update_session(
		state: &AppState,
		id: String,
		payload: SessionsUpdateRequestDto,
	) -> Response {
		if let Err((status, message)) = validate_request(&payload) {
			return common_response(status, &message);
		}

		// Validate that tests are not empty
		if payload.tests.is_empty() {
			return common_response(StatusCode::BAD_REQUEST, "Tests must not be empty");
		}

		let repo = SessionsRepository::new(state);
		let updated_session = SessionsSchema::update(payload, id);
		match repo.query_update_session(updated_session).await {
			Ok(msg) => common_response(StatusCode::OK, &msg),
			Err(e) => common_response(StatusCode::BAD_REQUEST, &e.to_string()),
		}
	}

	pub async fn delete_session(state: &AppState, id: String) -> Response {
		let repo = SessionsRepository::new(state);
		match repo.query_raw_session_by_id(&id).await {
			Ok(_) => {}
			Err(err) if err.to_string().contains("not found") => {
				return common_response(StatusCode::NOT_FOUND, "Session not found");
			}
			Err(e) => {
				return common_response(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string());
			}
		}
		match repo.query_delete_session(id).await {
			Ok(msg) => common_response(StatusCode::OK, &msg),
			Err(e) => common_response(StatusCode::BAD_REQUEST, &e.to_string()),
		}
	}
}
