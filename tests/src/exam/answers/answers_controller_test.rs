use crate::{
	authorized, create_mock_app_state, seed_user_with_one_permission,
	seed_user_with_permissions,
};
use axum::{Extension, Router};
use axum_test::TestServer;
use najm_entity::AppState;
use najm_exam::v1::answers::{
	AnswerEntryDto, AnswersCreateAkademikRequestDto, AnswersSchema, answers_router,
};
use najm_iam::v1::permissions::PermissionsEnum;
use najm_util::get_iso_date;
use surrealdb::Uuid;

fn create_test_app(state: AppState) -> TestServer {
	let app = Router::new()
		.nest("/v1/answers", answers_router())
		.layer(Extension(state));
	TestServer::new(app).unwrap()
}

#[tokio::test]
async fn test_post_create_answer_should_return_200() {
	let state = create_mock_app_state().await;
	let (user_id, _) =
		seed_user_with_one_permission(&state, PermissionsEnum::CreateAnswers).await;

	let db = &state.surrealdb_ws;
	let test_id = Uuid::new_v4().to_string();
	let session_id = Uuid::new_v4().to_string();
	let question_id = Uuid::new_v4().to_string();
	let option_id = Uuid::new_v4().to_string();
	let now = get_iso_date();
	let _ = db
		.query(format!(
			"CREATE app_options SET id = app_options:⟨{}⟩, label = 'Option A', is_correct = true, image_url = 'https://example.com/img.png', is_deleted = false, created_at = '{}', updated_at = '{}'",
			option_id, now, now
		))
		.await;
	let _ = db
		.query(format!(
			"CREATE app_questions SET id = app_questions:⟨{}⟩, question = 'What is Rust?', discussion = 'Rust is a system programming language', question_image_url = 'https://example.com/q.png', discussion_image_url = 'https://example.com/d.png', options = [app_options:⟨{}⟩], is_deleted = false, created_at = '{}', updated_at = '{}'",
			question_id, option_id, now, now
		))
		.await;
	let _ = db
		.query(format!(
			"CREATE app_tests SET id = app_tests:⟨{}⟩, name = 'Dummy Test', questions = [app_questions:⟨{}⟩], category = 'Test', sub_tests = NONE, banner = NONE, is_deleted = false, created_at = '{}', updated_at = '{}'",
			test_id, question_id, now, now
		))
		.await;
	let _ = db
		.query(format!(
			"CREATE app_sessions SET id = app_sessions:⟨{}⟩, name = 'Dummy Session', category = 'Dummy Category', description = 'Dummy Description', student_type = 'Dummy Type', tests = [{{ test: app_tests:⟨{}⟩, weight: '25%', shuffle: true, multiplier: 1.0, start_date: '2025-01-01T00:00:00Z', end_date: '2025-12-31T23:59:59Z' }}], banner = NONE, is_active = true, is_deleted = false, created_at = '{}', updated_at = '{}'",
			session_id, test_id, now, now
		))
		.await;
	let server = create_test_app(state);
	let payload = AnswersCreateAkademikRequestDto {
		user_id: user_id.clone(),
		test_id: test_id.clone(),
		session_id: session_id.clone(),
		answers: vec![AnswerEntryDto {
			question_id,
			option_id,
		}],
	};
	let res = authorized(
		&server,
		"POST",
		"/v1/answers/create-akademik",
		vec![&PermissionsEnum::CreateAnswers.to_string()],
		Some(&payload),
	)
	.await;
	assert_eq!(res.status_code(), 201);
}

#[tokio::test]
async fn test_delete_answer_should_return_200() {
	let state = create_mock_app_state().await;
	let (user_id, _) = seed_user_with_permissions(
		&state,
		vec![
			PermissionsEnum::CreateAnswers,
			PermissionsEnum::DeleteAnswers,
		],
	)
	.await;
	let db = &state.surrealdb_ws;
	let test_id = Uuid::new_v4().to_string();
	let session_id = Uuid::new_v4().to_string();
	let question_id = Uuid::new_v4().to_string();
	let option_id = Uuid::new_v4().to_string();
	let now = get_iso_date();
	let _ = db
		.query(format!(
			"CREATE app_options SET id = app_options:⟨{}⟩, label = 'Option A', is_correct = true, image_url = 'https://example.com/img.png', is_deleted = false, created_at = '{}', updated_at = '{}'",
			option_id, now, now
		))
		.await;
	let _ = db
		.query(format!(
			"CREATE app_questions SET id = app_questions:⟨{}⟩, question = 'What is Rust?', discussion = 'Rust is a system programming language', question_image_url = 'https://example.com/q.png', discussion_image_url = 'https://example.com/d.png', options = [app_options:⟨{}⟩], is_deleted = false, created_at = '{}', updated_at = '{}'",
			question_id, option_id, now, now
		))
		.await;
	let _ = db
		.query(format!(
			"CREATE app_tests SET id = app_tests:⟨{}⟩, name = 'Dummy Test', questions = [app_questions:⟨{}⟩], category = 'Test', sub_tests = NONE, banner = NONE, is_deleted = false, created_at = '{}', updated_at = '{}'",
			test_id, question_id, now, now
		))
		.await;
	let _ = db
		.query(format!(
			"CREATE app_sessions SET id = app_sessions:⟨{}⟩, name = 'Dummy Session', category = 'Dummy Category', description = 'Dummy Description', student_type = 'Dummy Type', tests = [{{ test: app_tests:⟨{}⟩, weight: '25%', shuffle: true, multiplier: 1.0, start_date: '2025-01-01T00:00:00Z', end_date: '2025-12-31T23:59:59Z' }}], banner = NONE, is_active = true, is_deleted = false, created_at = '{}', updated_at = '{}'",
			session_id, test_id, now, now
		))
		.await;
	let server = create_test_app(state.clone());
	let payload = AnswersCreateAkademikRequestDto {
		user_id: user_id.clone(),
		test_id: test_id.clone(),
		session_id: session_id.clone(),
		answers: vec![AnswerEntryDto {
			question_id: question_id.clone(),
			option_id: option_id.clone(),
		}],
	};
	let _ = authorized(
		&server,
		"POST",
		"/v1/answers/create-akademik",
		vec![&PermissionsEnum::CreateAnswers.to_string()],
		Some(&payload),
	)
	.await;
	let answer_id = {
		let results: Vec<AnswersSchema> = db
			.query(format!(
				"SELECT * FROM app_answers WHERE test = app_tests:⟨{}⟩ AND user = app_users:⟨{}⟩ ORDER BY created_at DESC LIMIT 1",
				test_id, user_id
			))
			.await
			.unwrap()
			.take(0)
			.unwrap();

		results.first().unwrap().id.id.to_raw()
	};
	let res = authorized::<()>(
		&server,
		"DELETE",
		&format!("/v1/answers/delete/{}", answer_id),
		vec![&PermissionsEnum::DeleteAnswers.to_string()],
		None,
	)
	.await;
	assert_eq!(res.status_code(), 200);
}
