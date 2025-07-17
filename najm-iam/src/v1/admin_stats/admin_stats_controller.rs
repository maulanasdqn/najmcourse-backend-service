use super::{AdminStatsService, AdminDashboardStatsResponseDto};
use crate::{permissions_guard, AppState, PermissionsEnum, ResponseSuccessDto};
use axum::{response::IntoResponse, Extension};

#[utoipa::path(
	get,
	security(("Bearer" = [])),
	path = "/v1/admin/stats/dashboard",
	responses(
		(status = 200, description = "Get admin dashboard statistics", body = ResponseSuccessDto<AdminDashboardStatsResponseDto>)
	),
	tag = "Admin Stats"
)]
pub async fn get_admin_dashboard_stats(
	headers: axum::http::HeaderMap,
	Extension(state): Extension<AppState>,
) -> impl IntoResponse {
	match permissions_guard(
		&headers,
		state.clone(),
		vec![PermissionsEnum::ReadDashboard],
	)
	.await
	{
		Ok(_) => AdminStatsService::get_admin_dashboard_stats(&state).await,
		Err(response) => response,
	}
}