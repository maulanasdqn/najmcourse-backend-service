use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct AdminDashboardStatsResponseDto {
	pub user_stats: UserStatsDto,
	pub examination_stats: ExaminationStatsDto,
	pub performance_stats: PerformanceStatsDto,
	pub content_stats: ContentStatsDto,
	pub system_stats: SystemStatsDto,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct UserStatsDto {
	#[schema(example = 1250)]
	pub total_users: u64,

	#[schema(example = 1200)]
	pub active_users: u64,

	#[schema(example = 50)]
	pub inactive_users: u64,

	pub users_by_role: Vec<UsersByRoleDto>,

	#[schema(example = 1100)]
	pub completed_profiles: u64,

	#[schema(example = 950)]
	pub completed_payments: u64,

	pub registration_trends: Vec<RegistrationTrendDto>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct UsersByRoleDto {
	#[schema(example = "Student")]
	pub role_name: String,

	#[schema(example = 1150)]
	pub count: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct RegistrationTrendDto {
	#[schema(example = "2024-01")]
	pub month: String,

	#[schema(example = 125)]
	pub registrations: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct ExaminationStatsDto {
	#[schema(example = 85)]
	pub total_tests: u64,

	#[schema(example = 78)]
	pub active_tests: u64,

	#[schema(example = 25)]
	pub total_sessions: u64,

	#[schema(example = 22)]
	pub active_sessions: u64,

	#[schema(example = 2450)]
	pub total_questions: u64,

	#[schema(example = 5500)]
	pub total_answers_submitted: u64,

	pub tests_by_category: Vec<TestsByCategoryDto>,

	pub sessions_participation: Vec<SessionParticipationDto>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct TestsByCategoryDto {
	#[schema(example = "Saintek")]
	pub category: String,

	#[schema(example = 45)]
	pub count: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct SessionParticipationDto {
	#[schema(example = "uuid")]
	pub session_id: String,

	#[schema(example = "Tryout Saintek 2025")]
	pub session_name: String,

	#[schema(example = 150)]
	pub participants: u64,

	#[schema(example = 85.5)]
	pub completion_rate: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct PerformanceStatsDto {
	#[schema(example = 1500)]
	pub monthly_active_users: u64,

	#[schema(example = 3250)]
	pub total_test_attempts: u64,

	#[schema(example = 78.5)]
	pub overall_average_score: f64,

	#[schema(example = 85.2)]
	pub completion_rate: f64,

	pub monthly_performance: Vec<MonthlyPerformanceDto>,

	pub top_performing_students: Vec<TopStudentDto>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct MonthlyPerformanceDto {
	#[schema(example = "2024-01")]
	pub month: String,

	#[schema(example = 450)]
	pub tests_taken: u64,

	#[schema(example = 82.3)]
	pub average_score: f64,

	#[schema(example = 320)]
	pub active_users: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct TopStudentDto {
	#[schema(example = "uuid")]
	pub user_id: String,

	#[schema(example = "John Doe")]
	pub fullname: String,

	#[schema(example = "john.doe@example.com")]
	pub email: String,

	#[schema(example = 92.5)]
	pub average_score: f64,

	#[schema(example = 15)]
	pub tests_taken: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct ContentStatsDto {
	#[schema(example = 2450)]
	pub total_questions: u64,

	#[schema(example = 9800)]
	pub total_options: u64,

	pub question_difficulty: Vec<QuestionDifficultyDto>,

	pub subject_distribution: Vec<SubjectDistributionDto>,

	#[schema(example = 72.5)]
	pub average_correct_rate: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct QuestionDifficultyDto {
	#[schema(example = "Easy")]
	pub difficulty: String,

	#[schema(example = 850)]
	pub count: u64,

	#[schema(example = 85.2)]
	pub correct_rate: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct SubjectDistributionDto {
	#[schema(example = "Matematika")]
	pub subject: String,

	#[schema(example = 450)]
	pub questions_count: u64,

	#[schema(example = 15)]
	pub tests_count: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct SystemStatsDto {
	pub database_metrics: DatabaseMetricsDto,

	pub growth_metrics: GrowthMetricsDto,

	pub data_integrity: DataIntegrityDto,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct DatabaseMetricsDto {
	#[schema(example = 1250)]
	pub total_users: u64,

	#[schema(example = 85)]
	pub total_tests: u64,

	#[schema(example = 25)]
	pub total_sessions: u64,

	#[schema(example = 2450)]
	pub total_questions: u64,

	#[schema(example = 9800)]
	pub total_options: u64,

	#[schema(example = 5500)]
	pub total_answers: u64,

	#[schema(example = 15)]
	pub total_roles: u64,

	#[schema(example = 45)]
	pub total_permissions: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct GrowthMetricsDto {
	#[schema(example = 15.5)]
	pub user_growth_rate: f64,

	#[schema(example = 8.2)]
	pub test_creation_rate: f64,

	#[schema(example = 25.7)]
	pub activity_growth_rate: f64,

	pub monthly_growth: Vec<MonthlyGrowthDto>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct MonthlyGrowthDto {
	#[schema(example = "2024-01")]
	pub month: String,

	#[schema(example = 125)]
	pub new_users: u64,

	#[schema(example = 8)]
	pub new_tests: u64,

	#[schema(example = 450)]
	pub new_answers: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct DataIntegrityDto {
	#[schema(example = 25)]
	pub deleted_users: u64,

	#[schema(example = 7)]
	pub deleted_tests: u64,

	#[schema(example = 3)]
	pub deleted_sessions: u64,

	#[schema(example = 85)]
	pub deleted_questions: u64,

	#[schema(example = 95.8)]
	pub data_integrity_score: f64,
}
