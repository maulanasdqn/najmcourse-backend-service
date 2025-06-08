pub mod landing;

use axum::Router;
pub use landing::*;

pub fn cms_public_routes() -> Router {
	Router::new().nest("/landing", landing_public_router())
}

pub fn cms_protected_routes() -> Router {
	Router::new().nest("/landing", landing_protected_router())
}
