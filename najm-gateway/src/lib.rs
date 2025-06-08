use axum::{
	Extension, Router, middleware::from_fn, response::Redirect, routing::get,
};
use najm_cms::{cms_protected_routes, cms_public_routes};
use najm_entity::{AppState, SurrealMemClient, SurrealWsClient};
use najm_exam::exam_protected_routes;
use najm_iam::{iam_protected_routes, iam_public_routes};
use najm_integration::integration_router;
use najm_middleware::{auth_middleware, cors_middleware};
use utoipa_swagger_ui::SwaggerUi;

pub mod docs;
pub use docs::*;

pub async fn gateway_service(
	surrealdb_ws: SurrealWsClient,
	surrealdb_mem: SurrealMemClient,
) -> Router {
	let state = AppState {
		surrealdb_ws,
		surrealdb_mem,
	};

	let public_routes = Router::new()
		.merge(iam_public_routes())
		.merge(cms_public_routes());

	let protected_routes = Router::new()
		.merge(iam_protected_routes())
		.merge(cms_protected_routes())
		.merge(exam_protected_routes())
		.merge(integration_router().await)
		.layer(from_fn(auth_middleware));

	let routes = public_routes.merge(protected_routes);

	Router::new()
		.route("/", get(Redirect::to("/docs")))
		.nest("/v1", routes)
		.merge(SwaggerUi::new("/docs").url("/openapi.json", docs_router()))
		.layer(cors_middleware())
		.layer(Extension(state))
}
