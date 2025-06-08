use axum::{Extension, Router};
use axum_test::TestServer;
use najm_course_entities::AppState;
use najm_course_utils::{authorized, create_mock_app_state};

use crate::{
	sessions::{SessionsRepository, SessionsUpdateRequestDto},
	PermissionsEnum,
};

use super::{SessionsCreateRequestDto, TestSessionsDto};

fn create_test_app(state: AppState) -> TestServer {
	let app = Router::new()
		.nest("/v1/sessions", crate::v1::sessions::sessions_router())
		.layer(Extension(state));
	TestServer::new(app).unwrap()
}

fn generate_payload(name: &str) -> SessionsCreateRequestDto {
	SessionsCreateRequestDto {
		name: name.to_string(),
		banner: None,
		category: "Saintek".into(),
		is_active: true,
		description: "Tryout Description".into(),
		student_type: "SMA".into(),
		tests: vec![TestSessionsDto {
			test_id: "mock_test_999".into(),
			weight: "25%".to_string(),
			shuffle: true,
			multiplier: 1.0,
			start_date: "2025-01-01T00:00:00Z".into(),
			end_date: "2025-01-10T00:00:00Z".into(),
		}],
	}
}

#[tokio::test]
async fn test_create_session_should_return_201() {
	let state = create_mock_app_state().await;
	let _ = state.surrealdb_ws.query(
		"CREATE app_tests:mock_test_999 SET name = 'Mock Test', questions = [], is_deleted = false, created_at = time::now(), updated_at = time::now();"
	).await.unwrap();
	let server = create_test_app(state);
	let payload = generate_payload("Session Create");
	let res = authorized(
		&server,
		"POST",
		"/v1/sessions/create",
		vec![&PermissionsEnum::CreateSessions.to_string()],
		Some(&payload),
	)
	.await;

	assert_eq!(res.status_code(), 201);
}

#[tokio::test]
async fn test_get_session_list_should_return_200() {
	let state = create_mock_app_state().await;
	let server = create_test_app(state);
	let res = authorized::<()>(
		&server,
		"GET",
		"/v1/sessions?page=1&per_page=5",
		vec![&PermissionsEnum::ReadListSessions.to_string()],
		None,
	)
	.await;
	dbg!(res.text());
	assert_eq!(res.status_code(), 200);
}

#[tokio::test]
async fn test_get_session_detail_should_return_200() {
	let state = create_mock_app_state().await;
	let _ = state.surrealdb_ws.query(
		"CREATE app_tests:mock_test_999 SET name = 'Mock Detail Test', questions = [], sub_tests = null, is_deleted = false, created_at = time::now(), updated_at = time::now(), category = 'Test', banner = null;"
	).await.unwrap();
	let repo = SessionsRepository::new(&state);
	let payload = generate_payload("Session Detail");
	let id = repo.query_create_session(payload).await.unwrap();

	let server = create_test_app(state);
	let res = authorized::<()>(
		&server,
		"GET",
		&format!("/v1/sessions/detail/{}", id),
		vec![&PermissionsEnum::ReadDetailSessions.to_string()],
		None,
	)
	.await;
	assert_eq!(res.status_code(), 200);
}

#[tokio::test]
async fn test_update_session_should_return_200() {
	let state = create_mock_app_state().await;
	let _ = state.surrealdb_ws.query(
		"CREATE app_tests:mock_test_update SET name = 'Mock Update Test', questions = [], is_deleted = false, created_at = time::now(), updated_at = time::now();"
	).await.unwrap();
	let repo = SessionsRepository::new(&state);
	let payload = generate_payload("To Update");
	let id = repo.query_create_session(payload).await.unwrap();
	let update = SessionsUpdateRequestDto {
		name: "Updated Session".into(),
		banner: None,
		category: "Soshum".into(),
		description: "Updated Description".into(),
		student_type: "SMA".into(),
		tests: vec![TestSessionsDto {
			test_id: "mock_test_update".into(),
			weight: "25%".to_string(),
			shuffle: true,
			multiplier: 1.25,
			start_date: "2025-02-01T00:00:00Z".into(),
			end_date: "2025-02-10T00:00:00Z".into(),
		}],
		is_active: true,
	};
	let server = create_test_app(state);
	let res = authorized(
		&server,
		"PUT",
		&format!("/v1/sessions/update/{}", id),
		vec![&PermissionsEnum::UpdateSessions.to_string()],
		Some(&update),
	)
	.await;
	assert_eq!(res.status_code(), 200);
}

#[tokio::test]
async fn test_delete_session_should_return_200() {
	let state = create_mock_app_state().await;
	let _ = state.surrealdb_ws.query(
		"CREATE app_tests:mock_test_delete SET name = 'Mock Delete Test', questions = [], is_deleted = false, created_at = time::now(), updated_at = time::now();"
	).await.unwrap();
	let repo = SessionsRepository::new(&state);
	let payload = generate_payload("To Be Deleted");
	let id = repo.query_create_session(payload).await.unwrap();
	let server = create_test_app(state);
	let res = authorized::<()>(
		&server,
		"DELETE",
		&format!("/v1/sessions/delete/{}", id),
		vec![&PermissionsEnum::DeleteSessions.to_string()],
		None,
	)
	.await;
	assert_eq!(res.status_code(), 200);
}

#[tokio::test]
async fn test_get_session_detail_should_return_404_if_not_found() {
	let state = create_mock_app_state().await;
	let server = create_test_app(state);
	let res = authorized::<()>(
		&server,
		"GET",
		"/v1/sessions/detail/not-found-id",
		vec![&PermissionsEnum::ReadDetailSessions.to_string()],
		None,
	)
	.await;
	assert_eq!(res.status_code(), 404);
}
