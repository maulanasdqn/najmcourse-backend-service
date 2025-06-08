pub mod events;

use axum::Router;
pub use events::*;

pub fn landing_public_router() -> Router {
	Router::new().nest("/events", events_public_routes())
}

pub fn landing_protected_router() -> Router {
	Router::new().nest("/events", events_protected_routes())
}
