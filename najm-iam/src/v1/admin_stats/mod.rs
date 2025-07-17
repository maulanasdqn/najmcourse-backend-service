use axum::{
	Router,
	routing::get,
};

pub mod admin_stats_controller;
pub mod admin_stats_dto;
pub mod admin_stats_repository;
pub mod admin_stats_schema;
pub mod admin_stats_service;

pub use admin_stats_controller::*;
pub use admin_stats_dto::*;
pub use admin_stats_repository::*;
pub use admin_stats_schema::*;
pub use admin_stats_service::*;

pub fn admin_stats_router() -> Router {
	Router::new()
		.route("/dashboard", get(get_admin_dashboard_stats))
}