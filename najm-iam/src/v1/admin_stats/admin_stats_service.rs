use super::AdminStatsRepository;
use crate::{AppState, ResponseSuccessDto, common_response, success_response};
use axum::{http::StatusCode, response::Response};

pub struct AdminStatsService;

impl AdminStatsService {
	pub async fn get_admin_dashboard_stats(state: &AppState) -> Response {
		let repo = AdminStatsRepository::new(state);
		match repo.query_admin_dashboard_stats().await {
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