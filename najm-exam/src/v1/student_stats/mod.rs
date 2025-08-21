use axum::{Router, routing::get};

pub mod student_stats_controller;
pub mod student_stats_dto;
pub mod student_stats_repository;
pub mod student_stats_service;

pub use student_stats_controller::*;
pub use student_stats_dto::*;
pub use student_stats_repository::*;
pub use student_stats_service::*;

pub fn student_stats_router() -> Router {
	Router::new().route("/dashboard/{user_id}", get(get_student_dashboard))
}
