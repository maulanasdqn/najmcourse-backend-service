use axum::http::{HeaderValue, Method, header};
use najm_lib::Env;
use tower_http::cors::CorsLayer;

pub fn cors_middleware() -> CorsLayer {
	let env = Env::new();
	let cors_origins = match env.rust_env.as_str() {
		"development" => vec!["http://localhost:3000", "http://localhost:3002"],
		"production" => {
			vec![
				"https://cat.najmcourse.com",
				"https://backoffice.najmcourse.com",
			]
		}
		_ => vec![
			"http://localhost:3000",
			"http://localhost:3002",
			"https://cat.najmcourse.com",
			"https://backoffice.najmcourse.com",
		],
	};
	let allowed_origins: Vec<HeaderValue> = cors_origins
		.into_iter()
		.filter_map(|origin| origin.parse::<HeaderValue>().ok())
		.collect();

	CorsLayer::new()
		.allow_origin(allowed_origins)
		.allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
		.allow_headers([header::AUTHORIZATION, header::CONTENT_TYPE])
		.allow_credentials(true)
}
