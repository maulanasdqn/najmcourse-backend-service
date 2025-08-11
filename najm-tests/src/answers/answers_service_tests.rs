use crate::{create_mock_app_state, seed_exam_dependencies};
use najm_exam::{
	AnswerEntryDto, AnswersCreateAkademikRequestDto, AnswersCreatePsikologiRequestDto,
	AnswersRepository, AnswersService, SessionsCreateRequestDto, SessionsRepository,
	SessionsSchema, TestSessionsDto,
};
use axum::http::StatusCode;
use najm_util::get_iso_date;
use surrealdb::Uuid;

/// Helper to create test session
async fn create_test_session(
	state: &crate::AppState,
	test_id: &str,
) -> anyhow::Result<String> {
	let sessions_repo = SessionsRepository::new(state);
	let session_payload = SessionsCreateRequestDto {
		name: "Service Test Session".to_string(),
		category: "Service Test".to_string(),
		banner: None,
		description: "Test Description for Service".to_string(),
		student_type: "Service Test Type".to_string(),
		is_active: true,
		tests: vec![TestSessionsDto {
			test_id: test_id.to_string(),
			weight: "100%".to_string(),
			shuffle: false,
			multiplier: 1.0,
			timer: Some(120),
			start_date: "2025-01-01T00:00:00Z".to_string(),
			end_date: "2025-12-31T23:59:59Z".to_string(),
		}],
	};
	let session = SessionsSchema::create(session_payload);
	let session_id = session.id.id.to_raw();
	sessions_repo.query_create_session(session).await?;
	Ok(session_id)
}

/// Helper to extract response body as string
async fn extract_response_body(response: axum::response::Response) -> String {
	let (_, body) = response.into_parts();
	let bytes = axum::body::to_bytes(body, usize::MAX).await.unwrap();
	String::from_utf8(bytes.to_vec()).unwrap()
}

/// Helper to check if response is successful (2xx status)
fn is_success_response(response: &axum::response::Response) -> bool {
	response.status().is_success()
}

#[tokio::test]
async fn test_get_answer_by_id_should_return_success() {
	let state = create_mock_app_state().await;
	let (user_id, _, test_id, question_id, option_id) = 
		seed_exam_dependencies(&state).await.unwrap();
	
	let session_id = create_test_session(&state, &test_id).await.unwrap();
	
	let payload = AnswersCreateAkademikRequestDto {
		user_id: user_id.to_string(),
		test_id: test_id.to_string(),
		session_id: session_id.to_string(),
		answers: vec![AnswerEntryDto {
			question_id: question_id.to_string(),
			option_id: option_id.to_string(),
		}],
	};
	
	// Create an answer first
	let repo = AnswersRepository::new(&state);
	let created_answer = repo.query_create_akademik(payload).await.unwrap();
	let answer_id = created_answer.id;
	
	// Test the service method
	let response = AnswersService::get_answer_by_id(&state, answer_id.clone()).await;
	
	assert!(is_success_response(&response));
	let body = extract_response_body(response).await;
	assert!(body.contains(&answer_id));
}

#[tokio::test]
async fn test_get_answer_by_id_with_invalid_id_should_return_not_found() {
	let state = create_mock_app_state().await;
	
	let response = AnswersService::get_answer_by_id(&state, "invalid-id".to_string()).await;
	
	assert_eq!(response.status(), StatusCode::NOT_FOUND);
	let body = extract_response_body(response).await;
	assert!(body.contains("Answer not found"));
}

#[tokio::test]
async fn test_get_answer_by_test_and_user_id_should_return_success() {
	let state = create_mock_app_state().await;
	let (user_id, _, test_id, question_id, option_id) = 
		seed_exam_dependencies(&state).await.unwrap();
	
	let session_id = create_test_session(&state, &test_id).await.unwrap();
	
	let payload = AnswersCreateAkademikRequestDto {
		user_id: user_id.to_string(),
		test_id: test_id.to_string(),
		session_id: session_id.to_string(),
		answers: vec![AnswerEntryDto {
			question_id: question_id.to_string(),
			option_id: option_id.to_string(),
		}],
	};
	
	// Create an answer first
	let repo = AnswersRepository::new(&state);
	let _ = repo.query_create_akademik(payload).await.unwrap();
	
	// Test the service method
	let response = AnswersService::get_answer_by_test_and_user_id(
		&state, 
		test_id.clone(), 
		user_id.clone()
	).await;
	
	let status = response.status();
	let body = extract_response_body(response).await;
	println!("Debug: Status = {:?}, Body = {}", status, body);
	assert!(status.is_success(), "Response was not successful: {:?}", status);
	assert!(body.contains(&test_id) || body.contains("score"), "Body doesn't contain test_id or score");
}

#[tokio::test]
async fn test_get_answer_by_test_and_user_id_with_no_answers_should_return_not_found() {
	let state = create_mock_app_state().await;
	let (user_id, _, test_id, _, _) = 
		seed_exam_dependencies(&state).await.unwrap();
	
	let response = AnswersService::get_answer_by_test_and_user_id(
		&state, 
		test_id, 
		user_id
	).await;
	
	assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_create_answer_akademik_should_return_success() {
	let state = create_mock_app_state().await;
	let (user_id, _, test_id, question_id, option_id) = 
		seed_exam_dependencies(&state).await.unwrap();
	
	let session_id = create_test_session(&state, &test_id).await.unwrap();
	
	let payload = AnswersCreateAkademikRequestDto {
		user_id: user_id.to_string(),
		test_id: test_id.to_string(),
		session_id: session_id.to_string(),
		answers: vec![AnswerEntryDto {
			question_id: question_id.to_string(),
			option_id: option_id.to_string(),
		}],
	};
	
	let response = AnswersService::create_answer_akademik(&state, payload).await;
	
	assert!(is_success_response(&response));
	let body = extract_response_body(response).await;
	assert!(body.contains("score"));
}

#[tokio::test]
async fn test_create_answer_akademik_with_invalid_test_should_return_bad_request() {
	let state = create_mock_app_state().await;
	let (user_id, _, _, question_id, option_id) = 
		seed_exam_dependencies(&state).await.unwrap();
	
	let invalid_test_id = Uuid::new_v4().to_string();
	let session_id = create_test_session(&state, &invalid_test_id).await.unwrap();
	
	let payload = AnswersCreateAkademikRequestDto {
		user_id: user_id.to_string(),
		test_id: invalid_test_id.to_string(),
		session_id: session_id.to_string(),
		answers: vec![AnswerEntryDto {
			question_id: question_id.to_string(),
			option_id: option_id.to_string(),
		}],
	};
	
	let response = AnswersService::create_answer_akademik(&state, payload).await;
	
	// Should return error due to test not found or other validation failure
	assert!(!is_success_response(&response));
}

#[tokio::test]
async fn test_create_answer_akademik_with_empty_answers_should_fail_validation() {
	let state = create_mock_app_state().await;
	let (user_id, _, test_id, _, _) = 
		seed_exam_dependencies(&state).await.unwrap();
	
	let session_id = create_test_session(&state, &test_id).await.unwrap();
	
	let payload = AnswersCreateAkademikRequestDto {
		user_id: user_id.to_string(),
		test_id: test_id.to_string(),
		session_id: session_id.to_string(),
		answers: vec![], // Empty answers should fail validation
	};
	
	let response = AnswersService::create_answer_akademik(&state, payload).await;
	
	assert!(!is_success_response(&response));
}

#[tokio::test]
async fn test_create_answer_psikologi_should_return_success() {
	let state = create_mock_app_state().await;
	let db = &state.surrealdb_ws;
	let now = get_iso_date();

	// Create comprehensive test data for psikologi
	let user_id = Uuid::new_v4().to_string();
	let test_id = Uuid::new_v4().to_string();
	let sub_test_id = Uuid::new_v4().to_string();
	let question_id = Uuid::new_v4().to_string();
	let option_id = Uuid::new_v4().to_string();

	// Create all necessary entities
	db.query(&format!(
		"CREATE app_users:⟨{}⟩ SET name = 'Service Psikologi User', email = '{}', is_deleted = false, created_at = '{}', updated_at = '{}'",
		user_id, crate::generate_unique_email("service_psiko"), now, now
	)).await.unwrap();

	db.query(&format!(
		"CREATE app_options:⟨{}⟩ SET label = 'Service Psiko Option', is_correct = false, points = 8.0, is_deleted = false, created_at = '{}', updated_at = '{}'",
		option_id, now, now
	)).await.unwrap();

	db.query(&format!(
		"CREATE app_questions:⟨{}⟩ SET question = 'Service Psikologi Question?', discussion = 'Service Psiko Discussion', options = [app_options:⟨{}⟩], is_deleted = false, created_at = '{}', updated_at = '{}'",
		question_id, option_id, now, now
	)).await.unwrap();

	db.query(&format!(
		"CREATE app_sub_tests:⟨{}⟩ SET name = 'Service Psiko SubTest', category = 'Psikologi', passing_grade = 70.0, questions = [app_questions:⟨{}⟩], is_deleted = false, created_at = '{}', updated_at = '{}'",
		sub_test_id, question_id, now, now
	)).await.unwrap();

	db.query(&format!(
		"CREATE app_tests:⟨{}⟩ SET name = 'Service Psikologi Test', subject = 'Service Psychology', questions = [app_questions:⟨{}⟩], category = 'Psikologi', sub_tests = [app_sub_tests:⟨{}⟩], is_deleted = false, created_at = '{}', updated_at = '{}'",
		test_id, question_id, sub_test_id, now, now
	)).await.unwrap();

	let session_id = create_test_session(&state, &test_id).await.unwrap();

	let payload = AnswersCreatePsikologiRequestDto {
		user_id: user_id.to_string(),
		test_id: test_id.to_string(),
		sub_test_id: sub_test_id.to_string(),
		session_id: session_id.to_string(),
		answers: vec![AnswerEntryDto {
			question_id: question_id.to_string(),
			option_id: option_id.to_string(),
		}],
	};
	
	let response = AnswersService::create_answer_psikologi(&state, payload).await;
	
	assert!(is_success_response(&response));
	let body = extract_response_body(response).await;
	assert!(body.contains("score"));
	assert!(body.contains("passing_grade"));
}

#[tokio::test]
async fn test_create_answer_psikologi_with_invalid_sub_test_should_return_bad_request() {
	let state = create_mock_app_state().await;
	let (user_id, _, test_id, question_id, option_id) = 
		seed_exam_dependencies(&state).await.unwrap();
	
	let session_id = create_test_session(&state, &test_id).await.unwrap();
	let invalid_sub_test_id = Uuid::new_v4().to_string();
	
	let payload = AnswersCreatePsikologiRequestDto {
		user_id: user_id.to_string(),
		test_id: test_id.to_string(),
		sub_test_id: invalid_sub_test_id,
		session_id: session_id.to_string(),
		answers: vec![AnswerEntryDto {
			question_id: question_id.to_string(),
			option_id: option_id.to_string(),
		}],
	};
	
	let response = AnswersService::create_answer_psikologi(&state, payload).await;
	
	assert!(!is_success_response(&response));
}

#[tokio::test]
async fn test_delete_answer_should_return_success() {
	let state = create_mock_app_state().await;
	let (user_id, _, test_id, question_id, option_id) = 
		seed_exam_dependencies(&state).await.unwrap();
	
	let session_id = create_test_session(&state, &test_id).await.unwrap();
	
	let payload = AnswersCreateAkademikRequestDto {
		user_id: user_id.to_string(),
		test_id: test_id.to_string(),
		session_id: session_id.to_string(),
		answers: vec![AnswerEntryDto {
			question_id: question_id.to_string(),
			option_id: option_id.to_string(),
		}],
	};
	
	// Create an answer first
	let repo = AnswersRepository::new(&state);
	let created_answer = repo.query_create_akademik(payload).await.unwrap();
	let answer_id = created_answer.id;
	
	// Test the delete service method
	let response = AnswersService::delete_answer(&state, answer_id).await;
	
	assert_eq!(response.status(), StatusCode::OK);
	let body = extract_response_body(response).await;
	assert!(body.contains("Success delete answer"));
}

#[tokio::test]
async fn test_delete_answer_with_invalid_id_should_return_not_found() {
	let state = create_mock_app_state().await;
	
	let response = AnswersService::delete_answer(&state, "invalid-id".to_string()).await;
	
	assert_eq!(response.status(), StatusCode::NOT_FOUND);
	let body = extract_response_body(response).await;
	assert!(body.contains("Answer not found"));
}

#[tokio::test]
async fn test_delete_answer_twice_should_return_bad_request_on_second_delete() {
	let state = create_mock_app_state().await;
	let (user_id, _, test_id, question_id, option_id) = 
		seed_exam_dependencies(&state).await.unwrap();
	
	let session_id = create_test_session(&state, &test_id).await.unwrap();
	
	let payload = AnswersCreateAkademikRequestDto {
		user_id: user_id.to_string(),
		test_id: test_id.to_string(),
		session_id: session_id.to_string(),
		answers: vec![AnswerEntryDto {
			question_id: question_id.to_string(),
			option_id: option_id.to_string(),
		}],
	};
	
	// Create an answer first
	let repo = AnswersRepository::new(&state);
	let created_answer = repo.query_create_akademik(payload).await.unwrap();
	let answer_id = created_answer.id;
	
	// First delete should succeed
	let first_response = AnswersService::delete_answer(&state, answer_id.clone()).await;
	assert_eq!(first_response.status(), StatusCode::OK);
	
	// Second delete should fail
	let second_response = AnswersService::delete_answer(&state, answer_id).await;
	assert_eq!(second_response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_get_answer_by_test_sub_test_and_user_id_should_return_success() {
	let state = create_mock_app_state().await;
	let db = &state.surrealdb_ws;
	let now = get_iso_date();

	// Create comprehensive test data for sub-test scenario
	let user_id = Uuid::new_v4().to_string();
	let test_id = Uuid::new_v4().to_string();
	let sub_test_id = Uuid::new_v4().to_string();
	let question_id = Uuid::new_v4().to_string();
	let option_id = Uuid::new_v4().to_string();

	// Create all necessary entities
	db.query(&format!(
		"CREATE app_users:⟨{}⟩ SET name = 'SubTest User', email = '{}', is_deleted = false, created_at = '{}', updated_at = '{}'",
		user_id, crate::generate_unique_email("subtest"), now, now
	)).await.unwrap();

	db.query(&format!(
		"CREATE app_options:⟨{}⟩ SET label = 'SubTest Option', is_correct = true, points = 12.0, is_deleted = false, created_at = '{}', updated_at = '{}'",
		option_id, now, now
	)).await.unwrap();

	db.query(&format!(
		"CREATE app_questions:⟨{}⟩ SET question = 'SubTest Question?', discussion = 'SubTest Discussion', options = [app_options:⟨{}⟩], is_deleted = false, created_at = '{}', updated_at = '{}'",
		question_id, option_id, now, now
	)).await.unwrap();

	db.query(&format!(
		"CREATE app_sub_tests:⟨{}⟩ SET name = 'SubTest Name', category = 'Akademik', passing_grade = 80.0, questions = [app_questions:⟨{}⟩], is_deleted = false, created_at = '{}', updated_at = '{}'",
		sub_test_id, question_id, now, now
	)).await.unwrap();

	db.query(&format!(
		"CREATE app_tests:⟨{}⟩ SET name = 'Main Test with SubTest', subject = 'Complex Subject', questions = [app_questions:⟨{}⟩], category = 'Academic', sub_tests = [app_sub_tests:⟨{}⟩], is_deleted = false, created_at = '{}', updated_at = '{}'",
		test_id, question_id, sub_test_id, now, now
	)).await.unwrap();

	let session_id = create_test_session(&state, &test_id).await.unwrap();

	// Create psikologi answer with sub_test
	let payload = AnswersCreatePsikologiRequestDto {
		user_id: user_id.to_string(),
		test_id: test_id.to_string(),
		sub_test_id: sub_test_id.to_string(),
		session_id: session_id.to_string(),
		answers: vec![AnswerEntryDto {
			question_id: question_id.to_string(),
			option_id: option_id.to_string(),
		}],
	};
	
	// Create the answer first
	let _ = AnswersService::create_answer_psikologi(&state, payload).await;
	
	// Test the service method
	let response = AnswersService::get_answer_by_test_sub_test_and_user_id(
		&state, 
		test_id.clone(),
		sub_test_id.clone(),
		user_id.clone()
	).await;
	
	let status = response.status();
	let body = extract_response_body(response).await;
	println!("Debug Sub Test: Status = {:?}, Body = {}", status, body);
	
	if !status.is_success() {
		// Try to understand why it failed
		println!("Service failed with status: {:?}, body: {}", status, body);
	}
	
	assert!(status.is_success(), "Response was not successful: {:?}", status);
	assert!(body.contains(&test_id) || body.contains("score"), "Body doesn't contain expected content");
	assert!(body.contains("passing_grade") || body.contains("score"), "Body doesn't contain passing_grade or score");
}

#[tokio::test]
async fn test_service_error_handling_with_database_error() {
	let state = create_mock_app_state().await;
	
	// Try to get an answer with completely invalid format ID that will cause database error
	let response = AnswersService::get_answer_by_id(&state, "".to_string()).await;
	
	assert!(!is_success_response(&response));
}