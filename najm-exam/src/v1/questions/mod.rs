use axum::{
	Router,
	routing::{delete, get, post, put},
};

pub mod questions_controller;
pub mod questions_dto;
pub mod questions_repository;
pub mod questions_schema;
pub mod questions_service;

pub use questions_controller::*;
pub use questions_dto::*;
pub use questions_repository::*;
pub use questions_schema::*;
pub use questions_service::*;

pub fn questions_router() -> Router {
	Router::new()
		.route("/", get(get_question_list))
		.route("/create", post(post_create_question))
		.route("/detail/{id}", get(get_question_by_id))
		.route("/update/{id}", put(put_update_question))
		.route("/delete/{id}", delete(delete_question))
}
