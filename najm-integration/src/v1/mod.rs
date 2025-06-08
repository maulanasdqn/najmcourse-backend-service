use axum::Router;

pub mod storage;
pub use storage::*;

pub async fn integration_router() -> Router {
	Router::new().nest("/storage", storage_router().await)
}
