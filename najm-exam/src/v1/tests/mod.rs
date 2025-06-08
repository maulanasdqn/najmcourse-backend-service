use axum::{
	Router,
	routing::{delete, get, post, put},
};

pub mod tests_controller;
pub mod tests_dto;
pub mod tests_repository;
pub mod tests_schema;
pub mod tests_service;

pub use tests_controller::*;
pub use tests_dto::*;
pub use tests_repository::*;
pub use tests_schema::*;
pub use tests_service::*;

pub fn tests_router() -> Router {
	Router::new()
		.route("/", get(get_test_list))
		.route("/create", post(post_create_test))
		.route("/detail/{id}", get(get_test_by_id))
		.route("/update/{id}", put(put_update_test))
		.route("/delete/{id}", delete(delete_test))
}
