use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct StudentDashboardResponseDto {
	pub profile_overview: ProfileOverviewDto,
	pub performance_metrics: PerformanceMetricsDto,
	pub current_sessions: CurrentSessionsDto,
	pub subject_breakdown: SubjectBreakdownDto,
	pub achievements: AchievementsDto,
	pub recent_activity: RecentActivityDto,
	pub recommendations: RecommendationsDto,
	pub comparison_data: ComparisonDataDto,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct ProfileOverviewDto {
	#[schema(example = "uuid")]
	pub user_id: String,

	#[schema(example = "John Doe")]
	pub fullname: String,

	#[schema(example = "john.doe@example.com")]
	pub email: String,

	#[schema(example = true)]
	pub is_profile_completed: bool,

	#[schema(example = true)]
	pub is_payment_completed: bool,

	#[schema(example = 45)]
	pub days_since_registration: u64,

	#[schema(example = "2024-01-15T10:30:00Z")]
	pub last_active_date: Option<String>,

	#[schema(example = "REF123")]
	pub referral_code: Option<String>,

	#[schema(example = "Akademik")]
	pub student_type: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct PerformanceMetricsDto {
	#[schema(example = 78.5)]
	pub overall_average_score: f64,

	#[schema(example = 5.2)]
	pub improvement_percentage: f64,

	#[schema(example = "positive")]
	pub trend_direction: String,

	pub score_distribution: ScoreDistributionDto,

	#[schema(example = 45)]
	pub tests_completed: u32,

	#[schema(example = 48)]
	pub tests_started: u32,

	#[schema(example = 93.75)]
	pub completion_rate: f64,

	#[schema(example = 3600)]
	pub average_time_per_test: u32,

	#[schema(example = 75)]
	pub student_percentile: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct ScoreDistributionDto {
	#[schema(example = 5)]
	pub range_0_20: u32,

	#[schema(example = 8)]
	pub range_21_40: u32,

	#[schema(example = 15)]
	pub range_41_60: u32,

	#[schema(example = 12)]
	pub range_61_80: u32,

	#[schema(example = 5)]
	pub range_81_100: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct CurrentSessionsDto {
	pub active_sessions: Vec<ActiveSessionDto>,
	pub upcoming_sessions: Vec<UpcomingSessionDto>,
	pub missed_sessions: Vec<MissedSessionDto>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct ActiveSessionDto {
	#[schema(example = "uuid")]
	pub session_id: String,

	#[schema(example = "Tryout Saintek 2025")]
	pub session_name: String,

	#[schema(example = "Saintek")]
	pub category: String,

	#[schema(example = "2025-05-31T23:59:59Z")]
	pub end_date: String,

	#[schema(example = 5)]
	pub days_remaining: u32,

	#[schema(example = 0)]
	pub progress_percentage: u32,

	#[schema(example = false)]
	pub is_started: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct UpcomingSessionDto {
	#[schema(example = "uuid")]
	pub session_id: String,

	#[schema(example = "Tryout Soshum 2025")]
	pub session_name: String,

	#[schema(example = "Soshum")]
	pub category: String,

	#[schema(example = "2025-06-01T00:00:00Z")]
	pub start_date: String,

	#[schema(example = 3)]
	pub days_until_start: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct MissedSessionDto {
	#[schema(example = "uuid")]
	pub session_id: String,

	#[schema(example = "Tryout Campuran 2025")]
	pub session_name: String,

	#[schema(example = "Campuran")]
	pub category: String,

	#[schema(example = "2025-04-30T23:59:59Z")]
	pub end_date: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct SubjectBreakdownDto {
	pub subjects: Vec<SubjectPerformanceDto>,
	pub categories: Vec<CategoryPerformanceDto>,
	pub weak_areas: Vec<WeakAreaDto>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct SubjectPerformanceDto {
	#[schema(example = "Matematika")]
	pub subject: String,

	#[schema(example = 82.5)]
	pub average_score: f64,

	#[schema(example = 150)]
	pub questions_attempted: u32,

	#[schema(example = 120)]
	pub questions_correct: u32,

	#[schema(example = 80.0)]
	pub accuracy_rate: f64,

	#[schema(example = 12)]
	pub tests_taken: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct CategoryPerformanceDto {
	#[schema(example = "akademik")]
	pub category: String,

	#[schema(example = 78.5)]
	pub average_score: f64,

	#[schema(example = 2400)]
	pub time_spent_seconds: u32,

	#[schema(example = 25)]
	pub tests_taken: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct WeakAreaDto {
	#[schema(example = "Trigonometri")]
	pub topic: String,

	#[schema(example = "Matematika")]
	pub subject: String,

	#[schema(example = 45.5)]
	pub accuracy_rate: f64,

	#[schema(example = 20)]
	pub questions_attempted: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct AchievementsDto {
	pub milestones: Vec<MilestoneDto>,
	pub streaks: StreakDataDto,
	pub perfect_scores: Vec<PerfectScoreDto>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct MilestoneDto {
	#[schema(example = "first_test")]
	pub milestone_type: String,

	#[schema(example = "Completed First Test")]
	pub title: String,

	#[schema(example = "You completed your first test!")]
	pub description: String,

	#[schema(example = "2024-01-10T08:00:00Z")]
	pub achieved_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct StreakDataDto {
	#[schema(example = 7)]
	pub current_streak_days: u32,

	#[schema(example = 15)]
	pub longest_streak_days: u32,

	#[schema(example = "2024-01-08")]
	pub streak_start_date: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct PerfectScoreDto {
	#[schema(example = "uuid")]
	pub test_id: String,

	#[schema(example = "Quiz Matematika Dasar")]
	pub test_name: String,

	#[schema(example = "2024-01-12T14:30:00Z")]
	pub achieved_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct RecentActivityDto {
	pub last_tests: Vec<RecentTestDto>,
	pub study_time_last_7_days: StudyTimeDto,
	pub question_stats: QuestionStatsDto,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct RecentTestDto {
	#[schema(example = "uuid")]
	pub test_id: String,

	#[schema(example = "Tryout Matematika")]
	pub test_name: String,

	#[schema(example = 85)]
	pub score: i32,

	#[schema(example = "2024-01-14T10:00:00Z")]
	pub taken_at: String,

	#[schema(example = 3600)]
	pub duration_seconds: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct StudyTimeDto {
	#[schema(example = 25200)]
	pub total_seconds: u32,

	pub daily_breakdown: Vec<DailyStudyTimeDto>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct DailyStudyTimeDto {
	#[schema(example = "2024-01-14")]
	pub date: String,

	#[schema(example = 3600)]
	pub seconds: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct QuestionStatsDto {
	#[schema(example = 500)]
	pub total_answered: u32,

	#[schema(example = 380)]
	pub total_correct: u32,

	#[schema(example = 76.0)]
	pub accuracy_rate: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct RecommendationsDto {
	pub next_steps: Vec<NextStepDto>,
	pub focus_areas: Vec<FocusAreaDto>,
	pub suggested_schedule: Vec<SuggestedScheduleDto>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct NextStepDto {
	#[schema(example = "test")]
	pub action_type: String,

	#[schema(example = "Take Tryout Fisika Dasar")]
	pub title: String,

	#[schema(
		example = "Based on your performance, we recommend taking this test next"
	)]
	pub reason: String,

	#[schema(example = "uuid")]
	pub resource_id: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct FocusAreaDto {
	#[schema(example = "Geometri")]
	pub topic: String,

	#[schema(example = "Matematika")]
	pub subject: String,

	#[schema(example = "high")]
	pub priority: String,

	#[schema(example = "Your accuracy in this topic is below 50%")]
	pub reason: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct SuggestedScheduleDto {
	#[schema(example = "Monday")]
	pub day: String,

	#[schema(example = "Matematika")]
	pub subject: String,

	#[schema(example = 120)]
	pub suggested_minutes: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct ComparisonDataDto {
	pub personal_best: PersonalBestDto,
	pub platform_comparison: PlatformComparisonDto,
	pub monthly_progress: Vec<MonthlyProgressDto>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct PersonalBestDto {
	#[schema(example = 95)]
	pub highest_score: i32,

	#[schema(example = "Tryout Matematika Advanced")]
	pub best_test_name: String,

	#[schema(example = "2024-01-05T09:00:00Z")]
	pub achieved_at: String,

	pub best_subjects: Vec<BestSubjectDto>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct BestSubjectDto {
	#[schema(example = "Fisika")]
	pub subject: String,

	#[schema(example = 92)]
	pub highest_score: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct PlatformComparisonDto {
	#[schema(example = 78.5)]
	pub your_average: f64,

	#[schema(example = 72.3)]
	pub platform_average: f64,

	#[schema(example = "above")]
	pub position: String,

	#[schema(example = 6.2)]
	pub difference_percentage: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct MonthlyProgressDto {
	#[schema(example = "2024-01")]
	pub month: String,

	#[schema(example = 75.5)]
	pub average_score: f64,

	#[schema(example = 8)]
	pub tests_taken: u32,

	#[schema(example = 3.2)]
	pub improvement_from_previous: f64,
}
