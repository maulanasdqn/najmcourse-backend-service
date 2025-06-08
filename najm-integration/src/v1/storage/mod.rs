use axum::{Router, routing::post};

pub mod storage_controller;
pub mod storage_dto;
pub mod storage_repository;
pub mod storage_state;

pub use storage_controller::*;
pub use storage_dto::*;
pub use storage_repository::*;
pub use storage_state::*;

pub async fn storage_router() -> Router {
	let state = storage_state().await.unwrap();
	Router::new().route(
		"/upload",
		post(storage_controller::post_upload).with_state(state),
	)
}
