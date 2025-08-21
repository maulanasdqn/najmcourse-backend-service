use super::{StudentDashboardResponseDto, StudentStatsService};
use crate::{AppState, PermissionsEnum, ResponseSuccessDto, permissions_guard};
use axum::{Extension, extract::Path, response::IntoResponse};

#[utoipa::path(
	get,
	security(("Bearer" = [])),
	path = "/v1/student/stats/dashboard/{user_id}",
	params(("user_id" = String, Path, description = "User ID")),
	responses(
		(status = 200, description = "Get student dashboard statistics", body = ResponseSuccessDto<StudentDashboardResponseDto>)
	),
	tag = "Student Stats"
)]
pub async fn get_student_dashboard(
	headers: axum::http::HeaderMap,
	Extension(state): Extension<AppState>,
	Path(user_id): Path<String>,
) -> impl IntoResponse {
	match permissions_guard(
		&headers,
		state.clone(),
		vec![PermissionsEnum::ReadDetailSessions],
	)
	.await
	{
		Ok(_) => StudentStatsService::get_student_dashboard(&state, user_id).await,
		Err(response) => response,
	}
}
