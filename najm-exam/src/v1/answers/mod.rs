use axum::{
	Router,
	routing::{delete, get, post},
};

pub mod answers_controller;
pub mod answers_dto;
pub mod answers_repository;
pub mod answers_schema;
pub mod answers_service;

pub use answers_controller::*;
pub use answers_dto::*;
pub use answers_repository::*;
pub use answers_schema::*;
pub use answers_service::*;

pub fn answers_router() -> Router {
	Router::new()
		.route("/detail/{id}", get(get_answer_by_id))
		.route(
			"/detail/{test_id}/{user_id}",
			get(get_answer_by_test_and_user_id),
		)
		.route("/create", post(post_create_answer))
		.route("/delete/{id}", delete(delete_answer))
}
