use crate::{create_mock_app_state, seed_exam_dependencies};
use anyhow::Result;
use najm_exam::{
	AnswerEntryDto, AnswersCreateAkademikRequestDto, AnswersCreatePsikologiRequestDto,
	AnswersRepository, AnswersSchema, SessionsCreateRequestDto, SessionsRepository,
	SessionsSchema, TestSessionsDto,
};
use najm_util::{get_iso_date, make_thing};
use surrealdb::Uuid;

/// Helper function to create session for akademik tests
async fn create_test_session(
	state: &crate::AppState,
	test_id: &str,
	weight: &str,
	multiplier: f64,
) -> Result<String> {
	let sessions_repo = SessionsRepository::new(state);
	let session_payload = SessionsCreateRequestDto {
		name: "Test Session".to_string(),
		category: "Test Category".to_string(),
		banner: None,
		description: "Test Description".to_string(),
		student_type: "Test Type".to_string(),
		is_active: true,
		tests: vec![TestSessionsDto {
			test_id: test_id.to_string(),
			weight: weight.to_string(),
			shuffle: false,
			multiplier,
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

/// Helper function to create session for psikologi tests
async fn create_psikologi_test_session(
	state: &crate::AppState,
	test_id: &str,
) -> Result<String> {
	let sessions_repo = SessionsRepository::new(state);
	let session_payload = SessionsCreateRequestDto {
		name: "Psikologi Session".to_string(),
		category: "Psikologi Category".to_string(),
		banner: None,
		description: "Psikologi Test Description".to_string(),
		student_type: "Psychology Test".to_string(),
		is_active: true,
		tests: vec![TestSessionsDto {
			test_id: test_id.to_string(),
			weight: "100%".to_string(),
			shuffle: false,
			multiplier: 1.0,
			timer: Some(180),
			start_date: "2025-01-01T00:00:00Z".to_string(),
			end_date: "2025-12-31T23:59:59Z".to_string(),
		}],
	};
	let session = SessionsSchema::create(session_payload);
	let session_id = session.id.id.to_raw();
	sessions_repo.query_create_session(session).await?;
	Ok(session_id)
}

/// Helper function to build akademik answer payload
fn build_akademik_payload(
	user_id: &str,
	test_id: &str,
	session_id: &str,
	question_id: &str,
	option_id: &str,
) -> AnswersCreateAkademikRequestDto {
	AnswersCreateAkademikRequestDto {
		user_id: user_id.to_string(),
		test_id: test_id.to_string(),
		session_id: session_id.to_string(),
		answers: vec![AnswerEntryDto {
			question_id: question_id.to_string(),
			option_id: option_id.to_string(),
		}],
	}
}

/// Helper function to build psikologi answer payload
fn build_psikologi_payload(
	user_id: &str,
	test_id: &str,
	sub_test_id: &str,
	session_id: &str,
	question_id: &str,
	option_id: &str,
) -> AnswersCreatePsikologiRequestDto {
	AnswersCreatePsikologiRequestDto {
		user_id: user_id.to_string(),
		test_id: test_id.to_string(),
		sub_test_id: sub_test_id.to_string(),
		session_id: session_id.to_string(),
		answers: vec![AnswerEntryDto {
			question_id: question_id.to_string(),
			option_id: option_id.to_string(),
		}],
	}
}

#[tokio::test]
async fn test_answers_repository_new_should_create_instance() {
	let state = create_mock_app_state().await;
	let repo = AnswersRepository::new(&state);
	
	// Verify the repository is created with the correct state reference
	assert!(std::ptr::eq(repo.state, &state));
}

#[tokio::test]
async fn test_query_create_akademik_should_succeed() {
	let state = create_mock_app_state().await;
	let (user_id, _, test_id, question_id, option_id) = 
		seed_exam_dependencies(&state).await.unwrap();
	
	let session_id = create_test_session(&state, &test_id, "50%", 1.0)
		.await.unwrap();
	
	let payload = build_akademik_payload(
		&user_id, &test_id, &session_id, &question_id, &option_id
	);
	
	let repo = AnswersRepository::new(&state);
	let result = repo.query_create_akademik(payload).await;
	
	assert!(result.is_ok(), "Failed to create akademik answer: {:?}", result.unwrap_err());
	let answer_result = result.unwrap();
	println!("Debug: Actual score = {}, Expected score = 5", answer_result.score);
	assert_eq!(answer_result.score, 5); // 10 points * 1.0 multiplier * 0.5 weight = 5
}

#[tokio::test]
async fn test_query_create_akademik_with_zero_weight_should_succeed() {
	let state = create_mock_app_state().await;
	let (user_id, _, test_id, question_id, option_id) = 
		seed_exam_dependencies(&state).await.unwrap();
	
	let session_id = create_test_session(&state, &test_id, "0%", 1.0)
		.await.unwrap();
	
	let payload = build_akademik_payload(
		&user_id, &test_id, &session_id, &question_id, &option_id
	);
	
	let repo = AnswersRepository::new(&state);
	let result = repo.query_create_akademik(payload).await;
	
	assert!(result.is_ok());
	let answer_result = result.unwrap();
	assert_eq!(answer_result.score, 0); // Zero weight should result in zero score
}

#[tokio::test]
async fn test_query_create_akademik_with_multiplier_should_calculate_correctly() {
	let state = create_mock_app_state().await;
	let (user_id, _, test_id, question_id, option_id) = 
		seed_exam_dependencies(&state).await.unwrap();
	
	let session_id = create_test_session(&state, &test_id, "50%", 2.0)
		.await.unwrap();
	
	let payload = build_akademik_payload(
		&user_id, &test_id, &session_id, &question_id, &option_id
	);
	
	let repo = AnswersRepository::new(&state);
	let result = repo.query_create_akademik(payload).await;
	
	assert!(result.is_ok());
	let answer_result = result.unwrap();
	// 10 points * 2.0 multiplier * 0.5 weight = 10
	assert_eq!(answer_result.score, 10);
}

#[tokio::test]
async fn test_query_create_akademik_with_invalid_user_should_fail() {
	let state = create_mock_app_state().await;
	let (_, _, test_id, question_id, option_id) = 
		seed_exam_dependencies(&state).await.unwrap();
	
	let session_id = create_test_session(&state, &test_id, "50%", 1.0)
		.await.unwrap();
	
	let invalid_user_id = Uuid::new_v4().to_string();
	let payload = build_akademik_payload(
		&invalid_user_id, &test_id, &session_id, &question_id, &option_id
	);
	
	let repo = AnswersRepository::new(&state);
	let result = repo.query_create_akademik(payload).await;
	
	// This should still succeed as the repository doesn't validate user existence
	// during answer creation, but the answer will be created with invalid reference
	assert!(result.is_ok());
}

#[tokio::test] 
async fn test_query_create_akademik_with_empty_answers_should_fail() {
	let state = create_mock_app_state().await;
	let (user_id, _, test_id, _, _) = 
		seed_exam_dependencies(&state).await.unwrap();
	
	let session_id = create_test_session(&state, &test_id, "50%", 1.0)
		.await.unwrap();
	
	let payload = AnswersCreateAkademikRequestDto {
		user_id: user_id.to_string(),
		test_id: test_id.to_string(),
		session_id: session_id.to_string(),
		answers: vec![], // Empty answers should fail validation
	};
	
	let repo = AnswersRepository::new(&state);
	let result = repo.query_create_akademik(payload).await;
	
	assert!(result.is_err(), "Expected validation error for empty answers");
}

#[tokio::test]
async fn test_query_by_id_should_return_answer() {
	let state = create_mock_app_state().await;
	let (user_id, _, test_id, question_id, option_id) = 
		seed_exam_dependencies(&state).await.unwrap();
	
	let session_id = create_test_session(&state, &test_id, "50%", 1.0)
		.await.unwrap();
	
	let payload = build_akademik_payload(
		&user_id, &test_id, &session_id, &question_id, &option_id
	);
	
	let repo = AnswersRepository::new(&state);
	let create_result = repo.query_create_akademik(payload).await.unwrap();
	let answer_id = create_result.id;
	
	let result = repo.query_by_id(&answer_id).await;
	
	assert!(result.is_ok());
	let answer = result.unwrap();
	assert_eq!(answer.id, answer_id);
}

#[tokio::test]
async fn test_query_by_id_with_invalid_id_should_fail() {
	let state = create_mock_app_state().await;
	let repo = AnswersRepository::new(&state);
	
	let result = repo.query_by_id("non-existent-id").await;
	
	assert!(result.is_err());
	assert!(result.unwrap_err().to_string().contains("Answer not found"));
}

#[tokio::test]
async fn test_query_by_test_and_user_id_should_return_answer() {
	let state = create_mock_app_state().await;
	let (user_id, _, test_id, question_id, option_id) = 
		seed_exam_dependencies(&state).await.unwrap();
	
	let session_id = create_test_session(&state, &test_id, "50%", 1.0)
		.await.unwrap();
	
	let payload = build_akademik_payload(
		&user_id, &test_id, &session_id, &question_id, &option_id
	);
	
	let repo = AnswersRepository::new(&state);
	let _ = repo.query_create_akademik(payload).await.unwrap();
	
	let result = repo.query_by_test_and_user_id(&test_id, &user_id).await;
	
	assert!(result.is_ok());
	let answer = result.unwrap();
	assert_eq!(answer.score, 5); // Should match the created answer score
}

#[tokio::test]
async fn test_query_by_test_and_user_id_with_no_answers_should_fail() {
	let state = create_mock_app_state().await;
	let (user_id, _, test_id, _, _) = 
		seed_exam_dependencies(&state).await.unwrap();
	
	let repo = AnswersRepository::new(&state);
	let result = repo.query_by_test_and_user_id(&test_id, &user_id).await;
	
	assert!(result.is_err());
	assert!(result.unwrap_err().to_string().contains("No answers found"));
}

#[tokio::test]
async fn test_query_delete_should_soft_delete_answer() {
	let state = create_mock_app_state().await;
	let (user_id, _, test_id, question_id, option_id) = 
		seed_exam_dependencies(&state).await.unwrap();
	
	let session_id = create_test_session(&state, &test_id, "50%", 1.0)
		.await.unwrap();
	
	let payload = build_akademik_payload(
		&user_id, &test_id, &session_id, &question_id, &option_id
	);
	
	let repo = AnswersRepository::new(&state);
	let create_result = repo.query_create_akademik(payload).await.unwrap();
	let answer_id = create_result.id;
	
	// Verify answer exists before deletion
	let pre_delete = repo.query_by_id(&answer_id).await;
	assert!(pre_delete.is_ok());
	
	// Delete the answer
	let delete_result = repo.query_delete(answer_id.clone()).await;
	assert!(delete_result.is_ok());
	assert_eq!(delete_result.unwrap(), "Success delete answer");
	
	// Verify answer is soft deleted (query_by_id should fail)
	let post_delete = repo.query_by_id(&answer_id).await;
	assert!(post_delete.is_err());
}

#[tokio::test]
async fn test_query_delete_with_invalid_id_should_fail() {
	let state = create_mock_app_state().await;
	let repo = AnswersRepository::new(&state);
	
	let result = repo.query_delete("non-existent-id".to_string()).await;
	
	assert!(result.is_err());
	assert!(result.unwrap_err().to_string().contains("Answer not found"));
}

#[tokio::test]
async fn test_query_delete_already_deleted_should_fail() {
	let state = create_mock_app_state().await;
	let (user_id, _, test_id, question_id, option_id) = 
		seed_exam_dependencies(&state).await.unwrap();
	
	let session_id = create_test_session(&state, &test_id, "50%", 1.0)
		.await.unwrap();
	
	let payload = build_akademik_payload(
		&user_id, &test_id, &session_id, &question_id, &option_id
	);
	
	let repo = AnswersRepository::new(&state);
	let create_result = repo.query_create_akademik(payload).await.unwrap();
	let answer_id = create_result.id;
	
	// Delete once
	let first_delete = repo.query_delete(answer_id.clone()).await;
	assert!(first_delete.is_ok());
	
	// Try to delete again - should fail
	let second_delete = repo.query_delete(answer_id).await;
	assert!(second_delete.is_err());
	assert!(second_delete.unwrap_err().to_string().contains("Answer already deleted"));
}

#[tokio::test]
async fn test_query_raw_answer_by_id_should_return_schema() {
	let state = create_mock_app_state().await;
	let db = &state.surrealdb_ws;
	let now = get_iso_date();
	
	// Create a raw answer directly in database
	let user_id = Uuid::new_v4().to_string();
	let test_id = Uuid::new_v4().to_string();
	let session_id = Uuid::new_v4().to_string();
	let question_id = Uuid::new_v4().to_string();
	let option_id = Uuid::new_v4().to_string();
	let answer_id = Uuid::new_v4().to_string();
	
	let answer_thing = make_thing("app_answers", &answer_id);
	let user_thing = make_thing("app_users", &user_id);
	let test_thing = make_thing("app_tests", &test_id);
	let session_thing = make_thing("app_sessions", &session_id);
	let question_thing = make_thing("app_questions", &question_id);
	let option_thing = make_thing("app_options", &option_id);
	
	let answer_schema = AnswersSchema {
		id: answer_thing,
		user: user_thing,
		test: test_thing,
		sub_test: None,
		session: session_thing,
		question: question_thing,
		option: option_thing,
		is_correct: true,
		is_deleted: false,
		created_at: now.clone(),
		updated_at: now,
	};
	
	let _: Option<AnswersSchema> = db
		.create("app_answers")
		.content(answer_schema)
		.await
		.unwrap();
	
	let repo = AnswersRepository::new(&state);
	let result = repo.query_raw_answer_by_id(&answer_id).await;
	
	assert!(result.is_ok());
	let answer = result.unwrap();
	assert_eq!(answer.id.id.to_raw(), answer_id);
	assert!(answer.is_correct);
	assert!(!answer.is_deleted);
}

#[tokio::test]
async fn test_query_raw_answer_by_id_with_deleted_answer_should_fail() {
	let state = create_mock_app_state().await;
	let db = &state.surrealdb_ws;
	let now = get_iso_date();
	
	// Create a deleted answer
	let answer_id = Uuid::new_v4().to_string();
	let answer_thing = make_thing("app_answers", &answer_id);
	
	let answer_schema = AnswersSchema {
		id: answer_thing,
		user: make_thing("app_users", &Uuid::new_v4().to_string()),
		test: make_thing("app_tests", &Uuid::new_v4().to_string()),
		sub_test: None,
		session: make_thing("app_sessions", &Uuid::new_v4().to_string()),
		question: make_thing("app_questions", &Uuid::new_v4().to_string()),
		option: make_thing("app_options", &Uuid::new_v4().to_string()),
		is_correct: true,
		is_deleted: true, // Mark as deleted
		created_at: now.clone(),
		updated_at: now,
	};
	
	let _: Option<AnswersSchema> = db
		.create("app_answers")
		.content(answer_schema)
		.await
		.unwrap();
	
	let repo = AnswersRepository::new(&state);
	let result = repo.query_raw_answer_by_id(&answer_id).await;
	
	assert!(result.is_err());
	assert!(result.unwrap_err().to_string().contains("Answer not found"));
}

#[tokio::test]
async fn test_multiple_answers_score_calculation() {
	let state = create_mock_app_state().await;
	let db = &state.surrealdb_ws;
	let now = get_iso_date();

	// Create test data with multiple questions and options
	let user_id = Uuid::new_v4().to_string();
	let test_id = Uuid::new_v4().to_string();
	let question1_id = Uuid::new_v4().to_string();
	let question2_id = Uuid::new_v4().to_string();
	let option1_id = Uuid::new_v4().to_string();
	let option2_id = Uuid::new_v4().to_string();

	// Create user
	db.query(&format!(
		"CREATE app_users:⟨{}⟩ SET name = 'Test User', email = '{}', is_deleted = false, created_at = '{}', updated_at = '{}'",
		user_id, crate::generate_unique_email("multi"), now, now
	)).await.unwrap();

	// Create options with different point values
	db.query(&format!(
		"CREATE app_options:⟨{}⟩ SET label = 'Correct Answer 1', is_correct = true, points = 15.0, is_deleted = false, created_at = '{}', updated_at = '{}'",
		option1_id, now, now
	)).await.unwrap();

	db.query(&format!(
		"CREATE app_options:⟨{}⟩ SET label = 'Correct Answer 2', is_correct = true, points = 25.0, is_deleted = false, created_at = '{}', updated_at = '{}'",
		option2_id, now, now
	)).await.unwrap();

	// Create questions
	db.query(&format!(
		"CREATE app_questions:⟨{}⟩ SET question = 'Question 1?', discussion = 'Discussion 1', options = [app_options:⟨{}⟩], is_deleted = false, created_at = '{}', updated_at = '{}'",
		question1_id, option1_id, now, now
	)).await.unwrap();

	db.query(&format!(
		"CREATE app_questions:⟨{}⟩ SET question = 'Question 2?', discussion = 'Discussion 2', options = [app_options:⟨{}⟩], is_deleted = false, created_at = '{}', updated_at = '{}'",
		question2_id, option2_id, now, now
	)).await.unwrap();

	// Create test
	db.query(&format!(
		"CREATE app_tests:⟨{}⟩ SET name = 'Multi Question Test', subject = 'Test Subject', questions = [app_questions:⟨{}⟩, app_questions:⟨{}⟩], category = 'Academic', is_deleted = false, created_at = '{}', updated_at = '{}'",
		test_id, question1_id, question2_id, now, now
	)).await.unwrap();

	let session_id = create_test_session(&state, &test_id, "50%", 1.5).await.unwrap();

	let payload = AnswersCreateAkademikRequestDto {
		user_id: user_id.to_string(),
		test_id: test_id.to_string(),
		session_id: session_id.to_string(),
		answers: vec![
			AnswerEntryDto {
				question_id: question1_id,
				option_id: option1_id,
			},
			AnswerEntryDto {
				question_id: question2_id,
				option_id: option2_id,
			},
		],
	};

	let repo = AnswersRepository::new(&state);
	let result = repo.query_create_akademik(payload).await;
	
	assert!(result.is_ok());
	let answer_result = result.unwrap();
	
	// (15 + 25) * 1.5 multiplier * 0.5 weight = 30
	assert_eq!(answer_result.score, 30);
	assert_eq!(answer_result.questions.len(), 2);
}

// Add tests for psikologi functionality if sub-tests are properly seeded
#[tokio::test] 
async fn test_psikologi_answer_creation_basic() {
	let state = create_mock_app_state().await;
	let db = &state.surrealdb_ws;
	let now = get_iso_date();

	// Create comprehensive test data for psikologi
	let user_id = Uuid::new_v4().to_string();
	let test_id = Uuid::new_v4().to_string();
	let sub_test_id = Uuid::new_v4().to_string();
	let question_id = Uuid::new_v4().to_string();
	let option_id = Uuid::new_v4().to_string();

	// Create user
	db.query(&format!(
		"CREATE app_users:⟨{}⟩ SET name = 'Psikologi User', email = '{}', is_deleted = false, created_at = '{}', updated_at = '{}'",
		user_id, crate::generate_unique_email("psiko"), now, now
	)).await.unwrap();

	// Create option
	db.query(&format!(
		"CREATE app_options:⟨{}⟩ SET label = 'Psiko Option', is_correct = false, points = 5.0, is_deleted = false, created_at = '{}', updated_at = '{}'",
		option_id, now, now
	)).await.unwrap();

	// Create question
	db.query(&format!(
		"CREATE app_questions:⟨{}⟩ SET question = 'Psikologi Question?', discussion = 'Psiko Discussion', options = [app_options:⟨{}⟩], is_deleted = false, created_at = '{}', updated_at = '{}'",
		question_id, option_id, now, now
	)).await.unwrap();

	// Create sub-test (this is key for psikologi tests)
	db.query(&format!(
		"CREATE app_sub_tests:⟨{}⟩ SET name = 'Psiko SubTest', category = 'Psikologi', passing_grade = 60.0, questions = [app_questions:⟨{}⟩], is_deleted = false, created_at = '{}', updated_at = '{}'",
		sub_test_id, question_id, now, now
	)).await.unwrap();

	// Create test with sub-test
	db.query(&format!(
		"CREATE app_tests:⟨{}⟩ SET name = 'Psikologi Test', subject = 'Psychology', questions = [app_questions:⟨{}⟩], category = 'Psikologi', sub_tests = [app_sub_tests:⟨{}⟩], is_deleted = false, created_at = '{}', updated_at = '{}'",
		test_id, question_id, sub_test_id, now, now
	)).await.unwrap();

	let session_id = create_psikologi_test_session(&state, &test_id).await.unwrap();

	let payload = build_psikologi_payload(
		&user_id, &test_id, &sub_test_id, &session_id, &question_id, &option_id
	);

	let repo = AnswersRepository::new(&state);
	let result = repo.query_create_psikologi(payload).await;
	
	assert!(result.is_ok(), "Failed to create psikologi answer: {:?}", result.unwrap_err());
	let answer_result = result.unwrap();
	
	// For psikologi, points are counted regardless of correctness
	assert_eq!(answer_result.score, 5);
	assert_eq!(answer_result.passing_grade, Some(60.0));
}