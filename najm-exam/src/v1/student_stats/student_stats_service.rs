use super::StudentStatsRepository;
use crate::{AppState, ResponseSuccessDto, common_response, success_response};
use axum::{http::StatusCode, response::Response};

pub struct StudentStatsService;

impl StudentStatsService {
	pub async fn get_student_dashboard(state: &AppState, user_id: String) -> Response {
		let repo = StudentStatsRepository::new(state);
		match repo.query_student_dashboard(&user_id).await {
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
}