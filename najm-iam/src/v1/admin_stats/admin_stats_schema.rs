use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserStatsSchema {
	pub total_users: u64,
	pub active_users: u64,
	pub inactive_users: u64,
	pub completed_profiles: u64,
	pub completed_payments: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UsersByRoleSchema {
	pub role_name: String,
	pub count: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RegistrationTrendSchema {
	pub month: String,
	pub registrations: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExaminationStatsSchema {
	pub total_tests: u64,
	pub active_tests: u64,
	pub total_sessions: u64,
	pub active_sessions: u64,
	pub total_questions: u64,
	pub total_answers_submitted: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TestsByCategorySchema {
	pub category: String,
	pub count: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SessionParticipationSchema {
	pub session_id: String,
	pub session_name: String,
	pub participants: u64,
	pub completion_rate: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PerformanceStatsSchema {
	pub monthly_active_users: u64,
	pub total_test_attempts: u64,
	pub overall_average_score: f64,
	pub completion_rate: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MonthlyPerformanceSchema {
	pub month: String,
	pub tests_taken: u64,
	pub average_score: f64,
	pub active_users: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TopStudentSchema {
	pub user_id: String,
	pub fullname: String,
	pub email: String,
	pub average_score: f64,
	pub tests_taken: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ContentStatsSchema {
	pub total_questions: u64,
	pub total_options: u64,
	pub average_correct_rate: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QuestionDifficultySchema {
	pub difficulty: String,
	pub count: u64,
	pub correct_rate: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SubjectDistributionSchema {
	pub subject: String,
	pub questions_count: u64,
	pub tests_count: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DatabaseMetricsSchema {
	pub total_users: u64,
	pub total_tests: u64,
	pub total_sessions: u64,
	pub total_questions: u64,
	pub total_options: u64,
	pub total_answers: u64,
	pub total_roles: u64,
	pub total_permissions: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GrowthMetricsSchema {
	pub user_growth_rate: f64,
	pub test_creation_rate: f64,
	pub activity_growth_rate: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MonthlyGrowthSchema {
	pub month: String,
	pub new_users: u64,
	pub new_tests: u64,
	pub new_answers: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DataIntegritySchema {
	pub deleted_users: u64,
	pub deleted_tests: u64,
	pub deleted_sessions: u64,
	pub deleted_questions: u64,
	pub data_integrity_score: f64,
}
