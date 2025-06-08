use axum::{
	Router,
	routing::{delete, get, post, put},
};

pub mod options_controller;
pub mod options_dto;
pub mod options_repository;
pub mod options_schema;
pub mod options_service;

pub use options_controller::*;
pub use options_dto::*;
pub use options_repository::*;
pub use options_schema::*;
pub use options_service::*;

pub fn options_router() -> Router {
	Router::new()
		.route("/", get(get_option_list))
		.route("/create", post(post_create_option))
		.route("/detail/{id}", get(get_option_by_id))
		.route("/update/{id}", put(put_update_option))
		.route("/delete/{id}", delete(delete_option))
}
