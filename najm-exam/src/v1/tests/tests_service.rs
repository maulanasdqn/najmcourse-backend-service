use super::{TestsCreateRequestDto, TestsRepository, TestsUpdateRequestDto};
use crate::{
	AppState, MetaRequestDto, ResponseListSuccessDto, ResponseSuccessDto,
	common_response, success_list_response, success_response, validate_request,
};
use axum::{http::StatusCode, response::Response};
use validator::Validate;

pub struct TestsService;

impl TestsService {
	pub async fn get_test_list(state: &AppState, meta: MetaRequestDto) -> Response {
		let repo = TestsRepository::new(state);
		match repo.query_test_list(meta).await {
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

	pub async fn get_test_by_id(state: &AppState, id: String) -> Response {
		let repo = TestsRepository::new(state);
		match repo.query_test_by_id(&id).await {
			Ok(test) => success_response(ResponseSuccessDto { data: test }),
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

	pub async fn create_test(
		state: &AppState,
		payload: TestsCreateRequestDto,
	) -> Response {
		if let Err((status, message)) = validate_request(&payload) {
			return common_response(status, &message);
		}
		match payload.category.to_lowercase().as_str() {
			"psikologi" => {
				if let Some(sub_tests) = &payload.sub_tests {
					for sub_test in sub_tests {
						if let Err(e) = sub_test.validate() {
							return common_response(StatusCode::BAD_REQUEST, &e.to_string());
						}
						for question in &sub_test.questions {
							if let Err(e) = question.validate() {
								return common_response(StatusCode::BAD_REQUEST, &e.to_string());
							}
							for option in &question.options {
								if let Err(e) = option.validate() {
									return common_response(StatusCode::BAD_REQUEST, &e.to_string());
								}
							}
						}
					}
				}
			}
			"akademik" => {
				if payload.questions.is_none()
					|| payload.questions.as_ref().is_none_or(|q| q.is_empty())
				{
					return common_response(
						StatusCode::BAD_REQUEST,
						"Questions are required for Akademik category",
					);
				}
				if let Some(questions) = &payload.questions {
					for question in questions {
						if let Err(e) = question.validate() {
							return common_response(StatusCode::BAD_REQUEST, &e.to_string());
						}
						for option in &question.options {
							if let Err(e) = option.validate() {
								return common_response(StatusCode::BAD_REQUEST, &e.to_string());
							}
						}
					}
				}
				if let Some(sub_tests) = &payload.sub_tests {
					for sub_test in sub_tests {
						if let Err(e) = sub_test.validate() {
							return common_response(StatusCode::BAD_REQUEST, &e.to_string());
						}
						for question in &sub_test.questions {
							if let Err(e) = question.validate() {
								return common_response(StatusCode::BAD_REQUEST, &e.to_string());
							}
							for option in &question.options {
								if let Err(e) = option.validate() {
									return common_response(StatusCode::BAD_REQUEST, &e.to_string());
								}
							}
						}
					}
				}
			}
			_ => {
				if payload.sub_tests.is_some() {
					return common_response(
						StatusCode::BAD_REQUEST,
						"Sub-tests are only allowed for Psikologi and Akademik categories",
					);
				}
			}
		}
		let repo = TestsRepository::new(state);
		match repo.query_create_test_with_relations(payload).await {
			Ok(msg) => common_response(StatusCode::CREATED, &msg),
			Err(e) => common_response(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()),
		}
	}

	pub async fn update_test(
		state: &AppState,
		id: String,
		payload: TestsUpdateRequestDto,
	) -> Response {
		if let Err((status, message)) = validate_request(&payload) {
			return common_response(status, &message);
		}
		match payload.category.to_lowercase().as_str() {
			"psikologi" => {
				if let Some(sub_tests) = &payload.sub_tests {
					for sub_test in sub_tests {
						if let Err(e) = sub_test.validate() {
							return common_response(StatusCode::BAD_REQUEST, &e.to_string());
						}
						for question in &sub_test.questions {
							if let Err(e) = question.validate() {
								return common_response(StatusCode::BAD_REQUEST, &e.to_string());
							}
							for option in &question.options {
								if let Err(e) = option.validate() {
									return common_response(StatusCode::BAD_REQUEST, &e.to_string());
								}
							}
						}
					}
				}
			}
			"akademik" => {
				if payload.questions.is_none()
					|| payload.questions.as_ref().is_none_or(|q| q.is_empty())
				{
					return common_response(
						StatusCode::BAD_REQUEST,
						"Questions are required for Akademik category",
					);
				}
				if let Some(questions) = &payload.questions {
					for question in questions {
						if let Err(e) = question.validate() {
							return common_response(StatusCode::BAD_REQUEST, &e.to_string());
						}
						for option in &question.options {
							if let Err(e) = option.validate() {
								return common_response(StatusCode::BAD_REQUEST, &e.to_string());
							}
						}
					}
				}
				if let Some(sub_tests) = &payload.sub_tests {
					for sub_test in sub_tests {
						if let Err(e) = sub_test.validate() {
							return common_response(StatusCode::BAD_REQUEST, &e.to_string());
						}
						for question in &sub_test.questions {
							if let Err(e) = question.validate() {
								return common_response(StatusCode::BAD_REQUEST, &e.to_string());
							}
							for option in &question.options {
								if let Err(e) = option.validate() {
									return common_response(StatusCode::BAD_REQUEST, &e.to_string());
								}
							}
						}
					}
				}
			}
			_ => {
				if payload.sub_tests.is_some() {
					return common_response(
						StatusCode::BAD_REQUEST,
						"Sub-tests are only allowed for Psikologi and Akademik categories",
					);
				}
			}
		}
		let repo = TestsRepository::new(state);
		match repo.query_update_test_with_relations(id, payload).await {
			Ok(msg) => common_response(StatusCode::OK, &msg),
			Err(e) => common_response(StatusCode::BAD_REQUEST, &e.to_string()),
		}
	}

	pub async fn delete_test(state: &AppState, id: String) -> Response {
		let repo = TestsRepository::new(state);
		if repo.query_test_by_id(&id).await.is_err() {
			return common_response(StatusCode::BAD_REQUEST, "Test not found");
		}
		match repo.query_delete_test(id).await {
			Ok(msg) => common_response(StatusCode::OK, &msg),
			Err(e) => common_response(StatusCode::BAD_REQUEST, &e.to_string()),
		}
	}
}
