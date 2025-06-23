use crate::{
	AppState, OptionsCreateRequestDto, OptionsUpdateRequestDto, PermissionsEnum,
	authorized, create_mock_app_state, options_router,
};
use axum::{Extension, Router};
use axum_test::TestServer;
use surrealdb::Uuid;

fn create_test_app(state: AppState) -> TestServer {
	let app = Router::new()
		.nest("/v1/options", options_router())
		.layer(Extension(state));
	TestServer::new(app).unwrap()
}

fn generate_payload() -> OptionsCreateRequestDto {
	OptionsCreateRequestDto {
		label: Some(format!("Option {}", Uuid::new_v4())),
		image_url: Some("https://example.com/img.png".into()),
		is_correct: true,
		points: Some(10.0),
	}
}

#[tokio::test]
async fn test_post_create_option_should_return_201() {
	let state = create_mock_app_state().await;
	let server = create_test_app(state);
	let payload = generate_payload();
	let res = authorized(
		&server,
		"POST",
		"/v1/options/create",
		vec![&PermissionsEnum::CreateOptions.to_string()],
		Some(&payload),
	)
	.await;
	assert_eq!(res.status_code(), 201);
}

#[tokio::test]
async fn test_get_option_list_should_return_200() {
	let state = create_mock_app_state().await;
	let server = create_test_app(state);
	let res = authorized::<()>(
		&server,
		"GET",
		"/v1/options?page=1&per_page=10",
		vec![&PermissionsEnum::ReadListOptions.to_string()],
		None,
	)
	.await;
	assert_eq!(res.status_code(), 200);
}

#[tokio::test]
async fn test_get_option_detail_should_return_200() {
	let state = create_mock_app_state().await;
	let server = create_test_app(state.clone());
	let payload = generate_payload();
	let _ = authorized(
		&server,
		"POST",
		"/v1/options/create",
		vec![&PermissionsEnum::CreateOptions.to_string()],
		Some(&payload),
	)
	.await;
	let repo = crate::v1::options::OptionsRepository::new(&state);
	let item = repo
		.query_option_by_label(payload.label.clone())
		.await
		.unwrap();
	let res = authorized::<()>(
		&server,
		"GET",
		&format!("/v1/options/detail/{}", item.id),
		vec![&PermissionsEnum::ReadDetailOptions.to_string()],
		None,
	)
	.await;
	assert_eq!(res.status_code(), 200);
}

#[tokio::test]
async fn test_put_update_option_should_return_200() {
	let state = create_mock_app_state().await;
	let server = create_test_app(state.clone());
	let payload = generate_payload();
	let _ = authorized(
		&server,
		"POST",
		"/v1/options/create",
		vec![&PermissionsEnum::CreateOptions.to_string()],
		Some(&payload),
	)
	.await;
	let repo = crate::v1::options::OptionsRepository::new(&state);
	let item = repo
		.query_option_by_label(payload.label.clone())
		.await
		.unwrap();
	let update = OptionsUpdateRequestDto {
		id: item.id.clone(),
		label: Some(format!("Updated {}", item.label)),
		image_url: Some("https://example.com/updated.png".into()),
		is_correct: false,
		points: Some(20.0),
	};
	let res = authorized(
		&server,
		"PUT",
		&format!("/v1/options/update/{}", item.id),
		vec![&PermissionsEnum::UpdateOptions.to_string()],
		Some(&update),
	)
	.await;
	assert_eq!(res.status_code(), 200);
}

#[tokio::test]
async fn test_delete_option_should_return_200() {
	let state = create_mock_app_state().await;
	let server = create_test_app(state.clone());
	let payload = generate_payload();
	let _ = authorized(
		&server,
		"POST",
		"/v1/options/create",
		vec![&PermissionsEnum::CreateOptions.to_string()],
		Some(&payload),
	)
	.await;
	let repo = crate::v1::options::OptionsRepository::new(&state);
	let item = repo
		.query_option_by_label(payload.label.clone())
		.await
		.unwrap();
	let res = authorized::<()>(
		&server,
		"DELETE",
		&format!("/v1/options/delete/{}", item.id),
		vec![&PermissionsEnum::DeleteOptions.to_string()],
		None,
	)
	.await;
	assert_eq!(res.status_code(), 200);
}

#[tokio::test]
async fn test_post_create_option_should_fail_with_invalid_payload() {
	let state = create_mock_app_state().await;
	let server = create_test_app(state);
	let invalid_payload = serde_json::json!({});
	let res = authorized(
		&server,
		"POST",
		"/v1/options/create",
		vec![&PermissionsEnum::CreateOptions.to_string()],
		Some(&invalid_payload),
	)
	.await;
	assert_eq!(res.status_code(), 422);
}

#[tokio::test]
async fn test_put_update_option_should_fail_if_not_found() {
	let state = create_mock_app_state().await;
	let server = create_test_app(state);
	let update = OptionsUpdateRequestDto {
		id: "non-existent-id".into(),
		label: Some("Non-existent".into()),
		image_url: None,
		is_correct: false,
		points: Some(2.2),
	};
	let res = authorized(
		&server,
		"PUT",
		"/v1/options/update/non-existent-id",
		vec![&PermissionsEnum::UpdateOptions.to_string()],
		Some(&update),
	)
	.await;
	assert_eq!(res.status_code(), 400);
	let body = res.text();
	assert!(body.contains("not found"));
}

#[tokio::test]
async fn test_delete_option_should_fail_if_not_found() {
	let state = create_mock_app_state().await;
	let server = create_test_app(state);
	let res = authorized::<()>(
		&server,
		"DELETE",
		"/v1/options/delete/non-existent-id",
		vec![&PermissionsEnum::DeleteOptions.to_string()],
		None,
	)
	.await;
	assert_eq!(res.status_code(), 404);
	let body = res.text();
	dbg!(res.status_code());
	dbg!(body.clone());
	assert!(body.contains("not found"));
}

#[tokio::test]
async fn test_get_option_list_should_fail_without_permission() {
	let state = create_mock_app_state().await;
	let server = create_test_app(state);
	let res = authorized::<()>(
		&server,
		"GET",
		"/v1/options?page=1&per_page=10",
		vec![],
		None,
	)
	.await;
	assert_eq!(res.status_code(), 403);
}
