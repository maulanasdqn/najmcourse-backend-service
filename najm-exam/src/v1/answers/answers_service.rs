use super::{
	AnswersCreateAkademikRequestDto, AnswersCreatePsikologiRequestDto,
	AnswersRepository,
};
use crate::{AppState, ResponseSuccessDto, common_response, success_response};
use axum::{http::StatusCode, response::Response};

pub struct AnswersService;

impl AnswersService {
	pub async fn get_answer_by_id(state: &AppState, id: String) -> Response {
		let repo = AnswersRepository::new(state);
		match repo.query_by_id(&id).await {
			Ok(answer) => success_response(ResponseSuccessDto { data: answer }),
			Err(e) => common_response(StatusCode::NOT_FOUND, &e.to_string()),
		}
	}

	pub async fn get_answer_by_test_and_user_id(
		state: &AppState,
		test_id: String,
		user_id: String,
	) -> Response {
		let repo = AnswersRepository::new(state);
		match repo.query_by_test_and_user_id(&test_id, &user_id).await {
			Ok(answer) => success_response(ResponseSuccessDto { data: answer }),
			Err(e) => common_response(StatusCode::NOT_FOUND, &e.to_string()),
		}
	}

	pub async fn get_answer_by_test_sub_test_and_user_id(
		state: &AppState,
		test_id: String,
		sub_test_id: String,
		user_id: String,
	) -> Response {
		let repo = AnswersRepository::new(state);
		match repo
			.query_by_test_sub_test_and_user_id(&test_id, &sub_test_id, &user_id)
			.await
		{
			Ok(answer) => success_response(ResponseSuccessDto { data: answer }),
			Err(e) => common_response(StatusCode::NOT_FOUND, &e.to_string()),
		}
	}

	pub async fn create_answer_akademik(
		state: &AppState,
		payload: AnswersCreateAkademikRequestDto,
	) -> Response {
		let repo = AnswersRepository::new(state);
		match repo.query_create_akademik(payload).await {
			Ok(data) => success_response(ResponseSuccessDto { data }),
			Err(e) => {
				let msg = e.to_string();
				let status = match msg.as_str() {
					"Test not found" | "Question not found" | "Option not found" => {
						StatusCode::BAD_REQUEST
					}
					_ => StatusCode::INTERNAL_SERVER_ERROR,
				};
				return common_response(status, &msg);
			}
		}
	}

	pub async fn create_answer_psikologi(
		state: &AppState,
		payload: AnswersCreatePsikologiRequestDto,
	) -> Response {
		let repo = AnswersRepository::new(state);
		match repo.query_create_psikologi(payload).await {
			Ok(data) => success_response(ResponseSuccessDto { data }),
			Err(e) => {
				let msg = e.to_string();
				let status = match msg.as_str() {
					"Test not found" | "Question not found" | "Option not found" => {
						StatusCode::BAD_REQUEST
					}
					_ => StatusCode::INTERNAL_SERVER_ERROR,
				};
				return common_response(status, &msg);
			}
		}
	}

	pub async fn delete_answer(state: &AppState, id: String) -> Response {
		let repo = AnswersRepository::new(state);
		match repo.query_by_id(&id).await {
			Ok(_) => {}
			Err(err) if err.to_string().contains("not found") => {
				return common_response(StatusCode::NOT_FOUND, "Answer not found");
			}
			Err(e) => {
				return common_response(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string());
			}
		}
		match repo.query_delete(id).await {
			Ok(msg) => common_response(StatusCode::OK, &msg),
			Err(e) => common_response(StatusCode::BAD_REQUEST, &e.to_string()),
		}
	}
}
