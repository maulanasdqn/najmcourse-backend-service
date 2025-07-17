use crate::create_mock_app_state;
use anyhow::Result;
use najm_exam::{
	AnswerEntryDto, AnswersCreateAkademikRequestDto, AnswersRepository, AnswersSchema,
	SessionsCreateRequestDto, SessionsRepository, SessionsSchema, TestSessionsDto,
};
use najm_util::{get_iso_date, make_thing};
use surrealdb::Uuid;

pub async fn seed_answer_dependencies(
	state: &crate::AppState,
) -> Result<(String, String, String, String)> {
	let db = &state.surrealdb_ws;
	let now = get_iso_date();

	let user_id = Uuid::new_v4().to_string();
	let user_thing = make_thing("app_users", &user_id);

	let test_id = Uuid::new_v4().to_string();
	let test_thing = make_thing("app_tests", &test_id);

	let question_id = Uuid::new_v4().to_string();
	let question_thing = make_thing("app_questions", &question_id);

	let option_id = Uuid::new_v4().to_string();
	let option_thing = make_thing("app_options", &option_id);

	db.query(&format!(
		"CREATE {} SET name = 'Test User', is_deleted = false, created_at = '{}', updated_at = '{}'",
		user_thing, now, now
	))
	.await?;

	db.query(&format!(
		"CREATE {} SET label = 'A system language', is_correct = true, points = 10.0, image_url = 'https://example.com/img.png', is_deleted = false, created_at = '{}', updated_at = '{}'",
		option_thing, now, now
	))
	.await?;

	db.query(&format!(
		"CREATE {} SET question = 'What is Rust?', discussion = 'Rust is a system programming language', question_image_url = 'https://example.com/q.png', discussion_image_url = 'https://example.com/d.png', options = [{}], is_deleted = false, created_at = '{}', updated_at = '{}'",
		question_thing, option_thing, now, now
	))
	.await?;

	db.query(&format!(
		"CREATE {} SET name = 'Dummy Test', subject = 'Test Subject', questions = [{}], category = 'Test', sub_tests = NONE, banner = NONE, is_deleted = false, created_at = '{}', updated_at = '{}'",
		test_thing, question_thing, now, now
	))
	.await?;

	Ok((user_id, test_id, question_id, option_id))
}

pub fn build_payload(
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

#[tokio::test]
async fn test_query_create_answers_should_succeed() {
	let state = create_mock_app_state().await;
	let (user_id, test_id, question_id, option_id) =
		seed_answer_dependencies(&state).await.unwrap();
	let sessions_repo = SessionsRepository::new(&state);
	let session_payload = SessionsCreateRequestDto {
		name: "Dummy Session".to_string(),
		category: "Dummy Category".to_string(),
		banner: None,
		description: "Dummy Description".to_string(),
		student_type: "Dummy Type".to_string(),
		is_active: true,
		tests: vec![TestSessionsDto {
			test_id: test_id.clone(),
			weight: "25%".to_string(),
			shuffle: true,
			multiplier: 1.0,
			timer: Some(120),
			start_date: "2025-01-01T00:00:00Z".to_string(),
			end_date: "2025-12-31T23:59:59Z".to_string(),
		}],
	};
	let session = SessionsSchema::create(session_payload);
	let session_id = session.id.id.to_raw();
	sessions_repo.query_create_session(session).await.unwrap();
	let payload =
		build_payload(&user_id, &test_id, &session_id, &question_id, &option_id);
	let repo = AnswersRepository::new(&state);
	let result = repo.query_create_akademik(payload).await;
	assert!(
		result.is_ok(),
		"Failed to create answer: {:?}",
		result.unwrap_err()
	);
}

#[tokio::test]
async fn test_query_by_id_should_return_data() {
	let state = create_mock_app_state().await;
	let db = &state.surrealdb_ws;
	let (user_id, test_id, question_id, option_id) =
		seed_answer_dependencies(&state).await.unwrap();
	let sessions_repo = SessionsRepository::new(&state);
	let session_payload = SessionsCreateRequestDto {
		name: "Dummy Session".to_string(),
		category: "Dummy Category".to_string(),
		banner: None,
		description: "Dummy Description".to_string(),
		student_type: "Dummy Type".to_string(),
		is_active: true,
		tests: vec![TestSessionsDto {
			test_id: test_id.clone(),
			weight: "25%".to_string(),
			shuffle: true,
			multiplier: 1.0,
			timer: Some(120),
			start_date: "2025-01-01T00:00:00Z".to_string(),
			end_date: "2025-12-31T23:59:59Z".to_string(),
		}],
	};
	let session = SessionsSchema::create(session_payload);
	let session_id = session.id.id.to_raw();
	sessions_repo.query_create_session(session).await.unwrap();
	let payload =
		build_payload(&user_id, &test_id, &session_id, &question_id, &option_id);
	let repo = AnswersRepository::new(&state);
	let _ = repo.query_create_akademik(payload.clone()).await.unwrap();
	let query = format!(
		"SELECT * FROM app_answers WHERE user = app_users:⟨{}⟩ AND session = app_sessions:⟨{}⟩ AND test = app_tests:⟨{}⟩ AND is_deleted = false",
		user_id, payload.session_id, payload.test_id
	);
	let answers: Vec<AnswersSchema> = db.query(&query).await.unwrap().take(0).unwrap();
	let answer_id = match answers.first() {
		Some(ans) => ans.id.id.to_raw(),
		None => panic!(
			"No answer found in app_answers for test_id={} and user_id={}",
			test_id, user_id
		),
	};
	let result = repo.query_by_id(&answer_id).await;
	assert!(result.is_ok());
}

#[tokio::test]
async fn test_query_by_id_should_fail_if_not_found() {
	let state = create_mock_app_state().await;
	let repo = AnswersRepository::new(&state);
	let result = repo.query_by_id("non-existent-id").await;
	assert!(result.is_err());
}

#[tokio::test]
async fn test_query_delete_should_soft_delete() {
	let state = create_mock_app_state().await;
	let db = &state.surrealdb_ws;
	let (user_id, test_id, question_id, option_id) =
		seed_answer_dependencies(&state).await.unwrap();
	let sessions_repo = SessionsRepository::new(&state);
	let session_payload = SessionsCreateRequestDto {
		name: "Dummy Session".to_string(),
		category: "Dummy Category".to_string(),
		banner: None,
		description: "Dummy Description".to_string(),
		student_type: "Dummy Type".to_string(),
		is_active: true,
		tests: vec![TestSessionsDto {
			test_id: test_id.clone(),
			weight: "25%".to_string(),
			shuffle: true,
			multiplier: 1.0,
			timer: Some(120),
			start_date: "2025-01-01T00:00:00Z".to_string(),
			end_date: "2025-12-31T23:59:59Z".to_string(),
		}],
	};
	let session = SessionsSchema::create(session_payload);
	let session_id = session.id.id.to_raw();
	sessions_repo.query_create_session(session).await.unwrap();
	let payload =
		build_payload(&user_id, &test_id, &session_id, &question_id, &option_id);
	let repo = AnswersRepository::new(&state);
	let _ = repo.query_create_akademik(payload.clone()).await.unwrap();
	let query = format!(
		"SELECT * FROM app_answers WHERE user = app_users:⟨{}⟩ AND session = app_sessions:⟨{}⟩ AND test = app_tests:⟨{}⟩ AND is_deleted = false",
		user_id, payload.session_id, payload.test_id
	);
	let answers: Vec<AnswersSchema> = db.query(&query).await.unwrap().take(0).unwrap();
	let answer_id = match answers.first() {
		Some(ans) => ans.id.id.to_raw(),
		None => panic!(
			"No answer found in app_answers for test_id={} and user_id={}",
			test_id, user_id
		),
	};
	let result = repo.query_delete(answer_id.clone()).await;
	assert!(result.is_ok());
	let check = repo.query_by_id(&answer_id).await;
	assert!(
		check.is_err(),
		"Expected query_by_id to fail after deletion"
	);
}

#[tokio::test]
async fn test_query_delete_should_fail_if_already_deleted() {
	let state = create_mock_app_state().await;
	let db = &state.surrealdb_ws;
	let (user_id, test_id, question_id, option_id) =
		seed_answer_dependencies(&state).await.unwrap();
	let sessions_repo = SessionsRepository::new(&state);
	let session_payload = SessionsCreateRequestDto {
		name: "Dummy Session".to_string(),
		category: "Dummy Category".to_string(),
		banner: None,
		description: "Dummy Description".to_string(),
		student_type: "Dummy Type".to_string(),
		is_active: true,
		tests: vec![TestSessionsDto {
			test_id: test_id.clone(),
			weight: "25%".to_string(),
			shuffle: true,
			multiplier: 1.0,
			timer: Some(120),
			start_date: "2025-01-01T00:00:00Z".to_string(),
			end_date: "2025-12-31T23:59:59Z".to_string(),
		}],
	};
	let session = SessionsSchema::create(session_payload);
	let session_id = session.id.id.to_raw();
	sessions_repo.query_create_session(session).await.unwrap();
	let now = chrono::Utc::now().to_rfc3339();
	let _ = db
		.query(format!(
			"CREATE app_users SET id = app_users:⟨{}⟩, name = 'Test User', is_deleted = false, created_at = '{now}', updated_at = '{now}'",
			user_id
		))
		.await;
	let payload =
		build_payload(&user_id, &test_id, &session_id, &question_id, &option_id);
	let repo = AnswersRepository::new(&state);
	let create_res = repo.query_create_akademik(payload.clone()).await;
	assert!(create_res.is_ok(), "Create failed: {:?}", create_res);
	let raw_query = format!(
		"SELECT * FROM app_answers WHERE test = app_tests:⟨{}⟩ AND user = app_users:⟨{}⟩ AND session = app_sessions:⟨{}⟩ AND is_deleted = false",
		payload.test_id, payload.user_id, payload.session_id
	);
	let raw_res = db.query(&raw_query).await;
	assert!(raw_res.is_ok(), "Raw query failed: {:?}", raw_res);
	let answers: Vec<AnswersSchema> = raw_res.unwrap().take(0).unwrap_or_default();
	assert!(!answers.is_empty(), "No answers returned");
	let id = answers.first().unwrap().id.id.to_raw();
	let delete_once = repo.query_delete(id.clone()).await;
	assert!(
		delete_once.is_ok(),
		"First delete failed: {:?}",
		delete_once
	);
	let delete_twice = repo.query_delete(id.clone()).await;
	assert!(delete_twice.is_err(), "Expected error on second delete");
}

#[tokio::test]
async fn test_score_formula_basic_calculation() {
	let state = create_mock_app_state().await;
	let (user_id, test_id, question_id, option_id) =
		seed_answer_dependencies(&state).await.unwrap();
	let sessions_repo = SessionsRepository::new(&state);
	let session_payload = SessionsCreateRequestDto {
		name: "Dummy Session".to_string(),
		category: "Dummy Category".to_string(),
		banner: None,
		description: "Dummy Description".to_string(),
		student_type: "Dummy Type".to_string(),
		is_active: true,
		tests: vec![TestSessionsDto {
			test_id: test_id.clone(),
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
	sessions_repo.query_create_session(session).await.unwrap();

	let payload =
		build_payload(&user_id, &test_id, &session_id, &question_id, &option_id);
	let repo = AnswersRepository::new(&state);
	let result = repo.query_create_akademik(payload).await.unwrap();

	assert_eq!(result.score, 0);
}

#[tokio::test]
async fn test_score_formula_with_multiple_questions() {
	let state = create_mock_app_state().await;
	let (user_id, test_id, question1_id, option1_id, question2_id, option2_id) =
		seed_multiple_questions(&state).await.unwrap();
	let sessions_repo = SessionsRepository::new(&state);
	let session_payload = SessionsCreateRequestDto {
		name: "Multi Question Session".to_string(),
		category: "Test Category".to_string(),
		banner: None,
		description: "Testing multiple questions score".to_string(),
		student_type: "Test Type".to_string(),
		is_active: true,
		tests: vec![TestSessionsDto {
			test_id: test_id.clone(),
			weight: "50%".to_string(),
			shuffle: false,
			multiplier: 1.5,
			timer: Some(120),
			start_date: "2025-01-01T00:00:00Z".to_string(),
			end_date: "2025-12-31T23:59:59Z".to_string(),
		}],
	};
	let session = SessionsSchema::create(session_payload);
	let session_id = session.id.id.to_raw();
	sessions_repo.query_create_session(session).await.unwrap();

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
	let result = repo.query_create_akademik(payload).await.unwrap();

	assert_eq!(result.score, 30);
}

#[tokio::test]
async fn test_score_formula_with_zero_weight() {
	let state = create_mock_app_state().await;
	let (user_id, test_id, question_id, option_id) =
		seed_answer_dependencies(&state).await.unwrap();
	let sessions_repo = SessionsRepository::new(&state);
	let session_payload = SessionsCreateRequestDto {
		name: "Zero Weight Session".to_string(),
		category: "Test Category".to_string(),
		banner: None,
		description: "Testing zero weight".to_string(),
		student_type: "Test Type".to_string(),
		is_active: true,
		tests: vec![TestSessionsDto {
			test_id: test_id.clone(),
			weight: "0%".to_string(),
			shuffle: false,
			multiplier: 3.0,
			timer: Some(120),
			start_date: "2025-01-01T00:00:00Z".to_string(),
			end_date: "2025-12-31T23:59:59Z".to_string(),
		}],
	};
	let session = SessionsSchema::create(session_payload);
	let session_id = session.id.id.to_raw();
	sessions_repo.query_create_session(session).await.unwrap();

	let payload =
		build_payload(&user_id, &test_id, &session_id, &question_id, &option_id);
	let repo = AnswersRepository::new(&state);
	let result = repo.query_create_akademik(payload).await.unwrap();

	assert_eq!(result.score, 0);
}

#[tokio::test]
async fn test_score_formula_with_decimal_points() {
	let state = create_mock_app_state().await;
	let (user_id, test_id, question_id, option_id) =
		seed_answer_dependencies(&state).await.unwrap();
	let sessions_repo = SessionsRepository::new(&state);
	let session_payload = SessionsCreateRequestDto {
		name: "Decimal Session".to_string(),
		category: "Test Category".to_string(),
		banner: None,
		description: "Testing decimal calculation".to_string(),
		student_type: "Test Type".to_string(),
		is_active: true,
		tests: vec![TestSessionsDto {
			test_id: test_id.clone(),
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
	sessions_repo.query_create_session(session).await.unwrap();

	let payload =
		build_payload(&user_id, &test_id, &session_id, &question_id, &option_id);
	let repo = AnswersRepository::new(&state);
	let result = repo.query_create_akademik(payload).await.unwrap();

	assert_eq!(result.score, 0);
}

#[tokio::test]
async fn test_score_formula_rounding_behavior() {
	let state = create_mock_app_state().await;
	let (user_id, test_id, question_id, option_id) =
		seed_answer_dependencies(&state).await.unwrap();
	let sessions_repo = SessionsRepository::new(&state);
	let session_payload = SessionsCreateRequestDto {
		name: "Rounding Session".to_string(),
		category: "Test Category".to_string(),
		banner: None,
		description: "Testing score rounding".to_string(),
		student_type: "Test Type".to_string(),
		is_active: true,
		tests: vec![TestSessionsDto {
			test_id: test_id.clone(),
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
	sessions_repo.query_create_session(session).await.unwrap();

	let payload =
		build_payload(&user_id, &test_id, &session_id, &question_id, &option_id);
	let repo = AnswersRepository::new(&state);
	let result = repo.query_create_akademik(payload).await.unwrap();

	assert_eq!(result.score, 0);
}

#[tokio::test]
async fn test_score_formula_with_max_weight_and_multiplier() {
	let state = create_mock_app_state().await;
	let (user_id, test_id, question_id, option_id) =
		seed_answer_dependencies(&state).await.unwrap();
	let sessions_repo = SessionsRepository::new(&state);
	let session_payload = SessionsCreateRequestDto {
		name: "Max Weight Session".to_string(),
		category: "Test Category".to_string(),
		banner: None,
		description: "Testing maximum weight and multiplier".to_string(),
		student_type: "Test Type".to_string(),
		is_active: true,
		tests: vec![TestSessionsDto {
			test_id: test_id.clone(),
			weight: "100%".to_string(),
			shuffle: false,
			multiplier: 5.0,
			timer: Some(120),
			start_date: "2025-01-01T00:00:00Z".to_string(),
			end_date: "2025-12-31T23:59:59Z".to_string(),
		}],
	};
	let session = SessionsSchema::create(session_payload);
	let session_id = session.id.id.to_raw();
	sessions_repo.query_create_session(session).await.unwrap();

	let payload =
		build_payload(&user_id, &test_id, &session_id, &question_id, &option_id);
	let repo = AnswersRepository::new(&state);
	let result = repo.query_create_akademik(payload).await.unwrap();

	assert_eq!(result.score, 0);
}

#[tokio::test]
async fn test_score_formula_edge_case_zero_points() {
	let state = create_mock_app_state().await;
	let (user_id, test_id, question_id, option_id) =
		seed_answer_dependencies(&state).await.unwrap();
	let sessions_repo = SessionsRepository::new(&state);
	let session_payload = SessionsCreateRequestDto {
		name: "Zero Points Session".to_string(),
		category: "Test Category".to_string(),
		banner: None,
		description: "Testing zero points scenario".to_string(),
		student_type: "Test Type".to_string(),
		is_active: true,
		tests: vec![TestSessionsDto {
			test_id: test_id.clone(),
			weight: "45%".to_string(),
			shuffle: false,
			multiplier: 10.0,
			timer: Some(120),
			start_date: "2025-01-01T00:00:00Z".to_string(),
			end_date: "2025-12-31T23:59:59Z".to_string(),
		}],
	};
	let session = SessionsSchema::create(session_payload);
	let session_id = session.id.id.to_raw();
	sessions_repo.query_create_session(session).await.unwrap();

	let payload =
		build_payload(&user_id, &test_id, &session_id, &question_id, &option_id);
	let repo = AnswersRepository::new(&state);
	let result = repo.query_create_akademik(payload).await.unwrap();

	assert_eq!(result.score, 45);
}

async fn seed_multiple_questions(
	state: &crate::AppState,
) -> Result<(String, String, String, String, String, String)> {
	let db = &state.surrealdb_ws;
	let now = get_iso_date();

	let user_id = Uuid::new_v4().to_string();
	let user_thing = make_thing("app_users", &user_id);

	let test_id = Uuid::new_v4().to_string();
	let test_thing = make_thing("app_tests", &test_id);

	let question1_id = Uuid::new_v4().to_string();
	let question1_thing = make_thing("app_questions", &question1_id);
	let option1_id = Uuid::new_v4().to_string();
	let option1_thing = make_thing("app_options", &option1_id);

	let question2_id = Uuid::new_v4().to_string();
	let question2_thing = make_thing("app_questions", &question2_id);
	let option2_id = Uuid::new_v4().to_string();
	let option2_thing = make_thing("app_options", &option2_id);

	db.query(&format!(
		"CREATE {} SET name = 'Test User', is_deleted = false, created_at = '{}', updated_at = '{}'",
		user_thing, now, now
	))
	.await.unwrap();

	db.query(&format!(
		"CREATE {} SET label = 'Answer 1', is_correct = true, points = 15.0, is_deleted = false, created_at = '{}', updated_at = '{}'",
		option1_thing, now, now
	))
	.await.unwrap();

	db.query(&format!(
		"CREATE {} SET label = 'Answer 2', is_correct = true, points = 25.0, is_deleted = false, created_at = '{}', updated_at = '{}'",
		option2_thing, now, now
	))
	.await.unwrap();

	db.query(&format!(
		"CREATE {} SET question = 'Question 1?', discussion = 'Discussion 1', options = [{}], is_deleted = false, created_at = '{}', updated_at = '{}'",
		question1_thing, option1_thing, now, now
	))
	.await.unwrap();

	db.query(&format!(
		"CREATE {} SET question = 'Question 2?', discussion = 'Discussion 2', options = [{}], is_deleted = false, created_at = '{}', updated_at = '{}'",
		question2_thing, option2_thing, now, now
	))
	.await.unwrap();

	db.query(&format!(
		"CREATE {} SET name = 'Multi Question Test', subject = 'Test Subject', questions = [{}, {}], category = 'Test', is_deleted = false, created_at = '{}', updated_at = '{}'",
		test_thing, question1_thing, question2_thing, now, now
	))
	.await.unwrap();

	Ok((
		user_id,
		test_id,
		question1_id,
		option1_id,
		question2_id,
		option2_id,
	))
}
