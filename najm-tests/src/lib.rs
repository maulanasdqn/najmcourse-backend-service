use najm_lib::AppState;
use najm_util::{get_iso_date, make_thing};
use surrealdb::Uuid;

pub mod answers;

/// Create a mock app state for testing
pub async fn create_mock_app_state() -> AppState {
	use najm_lib::{surrealdb_init_mem, surrealdb_init_ws};
	let surrealdb_ws = surrealdb_init_ws()
		.await
		.expect("Failed to create WebSocket database connection");
	let surrealdb_mem = surrealdb_init_mem()
		.await
		.expect("Failed to create in-memory database connection");
	AppState {
		surrealdb_ws,
		surrealdb_mem,
	}
}

/// Generate a unique email for testing
pub fn generate_unique_email(prefix: &str) -> String {
	format!("{}_{}@example.com", prefix, Uuid::new_v4())
}

/// Create test data dependencies for exams
pub async fn seed_exam_dependencies(
	state: &AppState,
) -> anyhow::Result<(String, String, String, String, String)> {
	let db = &state.surrealdb_ws;
	let now = get_iso_date();

	// Create user
	let user_id = Uuid::new_v4().to_string();
	let user_thing = make_thing("app_users", &user_id);

	// Create session
	let session_id = Uuid::new_v4().to_string();
	let session_thing = make_thing("app_sessions", &session_id);

	// Create test
	let test_id = Uuid::new_v4().to_string();
	let test_thing = make_thing("app_tests", &test_id);

	// Create question
	let question_id = Uuid::new_v4().to_string();
	let question_thing = make_thing("app_questions", &question_id);

	// Create option
	let option_id = Uuid::new_v4().to_string();
	let option_thing = make_thing("app_options", &option_id);

	// Insert test data
	db.query(&format!(
		"CREATE {} SET name = 'Test User', email = '{}', is_deleted = false, created_at = '{}', updated_at = '{}'",
		user_thing, generate_unique_email("test"), now, now
	))
	.await?;

	db.query(&format!(
		"CREATE {} SET name = 'Test Session', category = 'Test', description = 'Test session', student_type = 'All', is_active = true, is_deleted = false, created_at = '{}', updated_at = '{}'",
		session_thing, now, now
	))
	.await?;

	db.query(&format!(
		"CREATE {} SET label = 'Correct Answer', is_correct = true, points = 10.0, is_deleted = false, created_at = '{}', updated_at = '{}'",
		option_thing, now, now
	))
	.await?;

	db.query(&format!(
		"CREATE {} SET question = 'What is the answer?', discussion = 'This explains the answer', options = [{}], is_deleted = false, created_at = '{}', updated_at = '{}'",
		question_thing, option_thing, now, now
	))
	.await?;

	db.query(&format!(
		"CREATE {} SET name = 'Test Exam', subject = 'General', questions = [{}], category = 'Academic', is_deleted = false, created_at = '{}', updated_at = '{}'",
		test_thing, question_thing, now, now
	))
	.await?;

	Ok((user_id, session_id, test_id, question_id, option_id))
}