use axum::{
	Router,
	routing::{delete, get, post, put},
};

pub mod sessions_controller;
pub mod sessions_dto;
pub mod sessions_enum;
pub mod sessions_repository;
pub mod sessions_schema;
pub mod sessions_service;

pub use sessions_controller::*;
pub use sessions_dto::*;
pub use sessions_enum::*;
pub use sessions_repository::*;
pub use sessions_schema::*;
pub use sessions_service::*;

pub fn sessions_router() -> Router {
	Router::new()
		.route("/", get(get_session_list))
		.route("/create", post(post_create_session))
		.route("/detail/{id}", get(get_session_by_id))
		.route("/update/{id}", put(put_update_session))
		.route("/delete/{id}", delete(delete_session))
		.route("/student-stats/{user_id}", get(get_student_stats))
}
