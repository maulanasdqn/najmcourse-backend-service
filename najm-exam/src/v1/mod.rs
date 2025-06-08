use axum::Router;

pub mod answers;
pub mod options;
pub mod questions;
pub mod sessions;
pub mod tests;

pub use answers::*;
pub use options::*;
pub use questions::*;
pub use sessions::*;
pub use tests::*;

pub fn exam_protected_routes() -> Router {
	Router::new()
		.nest("/answers", answers_router())
		.nest("/sessions", sessions_router())
		.nest("/tests", tests_router())
		.nest("/questions", questions_router())
		.nest("/options", options_router())
}
