use axum::{
	Router,
	routing::{delete, get, patch, post},
};

pub mod events_controller;
pub mod events_dto;
pub mod events_repository;
pub mod events_schema;
pub mod events_service;

pub use events_controller::*;
pub use events_dto::*;
pub use events_repository::*;
pub use events_schema::*;
pub use events_service::*;

pub fn events_public_routes() -> Router {
	Router::new()
		.route("/", get(get_event_list))
		.route("/detail/{id}", get(get_event_by_id))
}

pub fn events_protected_routes() -> Router {
	Router::new()
		.route("/create", post(post_create_event))
		.route("/update/{id}", patch(patch_update_event))
		.route("/delete/{id}", delete(delete_event))
}
