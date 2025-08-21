use super::{
	AdminDashboardStatsResponseDto, ContentStatsDto, DataIntegrityDto,
	DatabaseMetricsDto, ExaminationStatsDto, GrowthMetricsDto, MonthlyGrowthDto,
	MonthlyPerformanceDto, PerformanceStatsDto, QuestionDifficultyDto,
	RegistrationTrendDto, SessionParticipationDto, SubjectDistributionDto,
	SystemStatsDto, TestsByCategoryDto, TopStudentDto, UserStatsDto, UsersByRoleDto,
};
use anyhow::Result;
use najm_lib::AppState;
use serde_json::Value;
use std::collections::HashMap;

pub struct AdminStatsRepository<'a> {
	state: &'a AppState,
}

impl<'a> AdminStatsRepository<'a> {
	pub fn new(state: &'a AppState) -> Self {
		Self { state }
	}

	pub async fn query_admin_dashboard_stats(
		&self,
	) -> Result<AdminDashboardStatsResponseDto> {
		// Fetch all required data in parallel
		let (
			user_stats,
			examination_stats,
			performance_stats,
			content_stats,
			system_stats,
		) = tokio::try_join!(
			self.query_user_stats(),
			self.query_examination_stats(),
			self.query_performance_stats(),
			self.query_content_stats(),
			self.query_system_stats(),
		)?;

		Ok(AdminDashboardStatsResponseDto {
			user_stats,
			examination_stats,
			performance_stats,
			content_stats,
			system_stats,
		})
	}

	async fn query_user_stats(&self) -> Result<UserStatsDto> {
		let db = &self.state.surrealdb_ws;

		// Get basic user counts
		let user_counts_query = "
			SELECT 
				COUNT() as total_users,
				COUNT(is_active = true) as active_users,
				COUNT(is_active = false) as inactive_users,
				COUNT(is_profile_completed = true) as completed_profiles,
				COUNT(is_payment_completed = true) as completed_payments
			FROM app_users 
			WHERE is_deleted = false
		";

		let user_counts: Vec<Value> = db.query(user_counts_query).await?.take(0)?;
		let user_count_data = user_counts
			.get(0)
			.ok_or_else(|| anyhow::anyhow!("No user count data"))?;

		// Get users by role
		let users_by_role_query = "
			SELECT 
				role.name as role_name,
				COUNT() as count
			FROM app_users 
			WHERE is_deleted = false
			GROUP BY role.name
			ORDER BY count DESC
		";

		let users_by_role: Vec<Value> = db.query(users_by_role_query).await?.take(0)?;
		let users_by_role_data: Vec<UsersByRoleDto> = users_by_role
			.iter()
			.map(|item| UsersByRoleDto {
				role_name: item["role_name"].as_str().unwrap_or("Unknown").to_string(),
				count: item["count"].as_u64().unwrap_or(0),
			})
			.collect();

		// Get registration trends (last 12 months)
		let registration_trends_query = "
			SELECT 
				time::format(created_at, '%Y-%m') as month,
				COUNT() as registrations
			FROM app_users 
			WHERE is_deleted = false 
			AND created_at >= time::now() - 1y
			GROUP BY month
			ORDER BY month DESC
		";

		let registration_trends: Vec<Value> =
			db.query(registration_trends_query).await?.take(0)?;
		let registration_trends_data: Vec<RegistrationTrendDto> = registration_trends
			.iter()
			.map(|item| RegistrationTrendDto {
				month: item["month"].as_str().unwrap_or("").to_string(),
				registrations: item["registrations"].as_u64().unwrap_or(0),
			})
			.collect();

		Ok(UserStatsDto {
			total_users: user_count_data["total_users"].as_u64().unwrap_or(0),
			active_users: user_count_data["active_users"].as_u64().unwrap_or(0),
			inactive_users: user_count_data["inactive_users"].as_u64().unwrap_or(0),
			completed_profiles: user_count_data["completed_profiles"]
				.as_u64()
				.unwrap_or(0),
			completed_payments: user_count_data["completed_payments"]
				.as_u64()
				.unwrap_or(0),
			users_by_role: users_by_role_data,
			registration_trends: registration_trends_data,
		})
	}

	async fn query_examination_stats(&self) -> Result<ExaminationStatsDto> {
		let db = &self.state.surrealdb_ws;

		// Get basic examination counts
		let exam_counts_query = "
			SELECT 
				(SELECT COUNT() FROM app_tests WHERE is_deleted = false) as total_tests,
				(SELECT COUNT() FROM app_tests WHERE is_deleted = false AND is_active = true) as active_tests,
				(SELECT COUNT() FROM app_sessions WHERE is_deleted = false) as total_sessions,
				(SELECT COUNT() FROM app_sessions WHERE is_deleted = false AND is_active = true) as active_sessions,
				(SELECT COUNT() FROM app_questions WHERE is_deleted = false) as total_questions,
				(SELECT COUNT() FROM app_answers WHERE is_deleted = false) as total_answers_submitted
			FROM (SELECT 1) as dummy
		";

		let exam_counts: Vec<Value> = db.query(exam_counts_query).await?.take(0)?;
		let exam_count_data = exam_counts
			.get(0)
			.ok_or_else(|| anyhow::anyhow!("No exam count data"))?;

		// Get tests by category
		let tests_by_category_query = "
			SELECT 
				category,
				COUNT() as count
			FROM app_tests 
			WHERE is_deleted = false
			GROUP BY category
			ORDER BY count DESC
		";

		let tests_by_category: Vec<Value> =
			db.query(tests_by_category_query).await?.take(0)?;
		let tests_by_category_data: Vec<TestsByCategoryDto> = tests_by_category
			.iter()
			.map(|item| TestsByCategoryDto {
				category: item["category"].as_str().unwrap_or("Unknown").to_string(),
				count: item["count"].as_u64().unwrap_or(0),
			})
			.collect();

		// Get session participation
		let session_participation_query = "
			SELECT 
				s.id as session_id,
				s.name as session_name,
				COUNT(DISTINCT a.user) as participants,
				(COUNT(DISTINCT a.user) * 100.0 / 
					(SELECT COUNT() FROM app_users WHERE is_deleted = false AND is_active = true)) as completion_rate
			FROM app_sessions s
			LEFT JOIN app_answers a ON a.session = s.id
			WHERE s.is_deleted = false
			GROUP BY s.id, s.name
			ORDER BY participants DESC
			LIMIT 10
		";

		let session_participation: Vec<Value> =
			db.query(session_participation_query).await?.take(0)?;
		let session_participation_data: Vec<SessionParticipationDto> =
			session_participation
				.iter()
				.map(|item| SessionParticipationDto {
					session_id: item["session_id"].as_str().unwrap_or("").to_string(),
					session_name: item["session_name"].as_str().unwrap_or("").to_string(),
					participants: item["participants"].as_u64().unwrap_or(0),
					completion_rate: item["completion_rate"].as_f64().unwrap_or(0.0),
				})
				.collect();

		Ok(ExaminationStatsDto {
			total_tests: exam_count_data["total_tests"].as_u64().unwrap_or(0),
			active_tests: exam_count_data["active_tests"].as_u64().unwrap_or(0),
			total_sessions: exam_count_data["total_sessions"].as_u64().unwrap_or(0),
			active_sessions: exam_count_data["active_sessions"].as_u64().unwrap_or(0),
			total_questions: exam_count_data["total_questions"].as_u64().unwrap_or(0),
			total_answers_submitted: exam_count_data["total_answers_submitted"]
				.as_u64()
				.unwrap_or(0),
			tests_by_category: tests_by_category_data,
			sessions_participation: session_participation_data,
		})
	}

	async fn query_performance_stats(&self) -> Result<PerformanceStatsDto> {
		let db = &self.state.surrealdb_ws;

		// Get monthly active users (users who submitted answers in the last 30 days)
		let monthly_active_query = "
			SELECT COUNT(DISTINCT user) as monthly_active_users
			FROM app_answers 
			WHERE is_deleted = false 
			AND created_at >= time::now() - 30d
		";

		let monthly_active: Vec<Value> =
			db.query(monthly_active_query).await?.take(0)?;
		let monthly_active_count = monthly_active
			.get(0)
			.and_then(|v| v["monthly_active_users"].as_u64())
			.unwrap_or(0);

		// Get total test attempts and overall performance
		let performance_query = "
			SELECT 
				COUNT() as total_test_attempts,
				AVG(CASE WHEN is_correct = true THEN 100 ELSE 0 END) as overall_average_score,
				(COUNT(DISTINCT user) * 100.0 / 
					(SELECT COUNT() FROM app_users WHERE is_deleted = false AND is_active = true)) as completion_rate
			FROM app_answers 
			WHERE is_deleted = false
		";

		let performance: Vec<Value> = db.query(performance_query).await?.take(0)?;
		let performance_data = performance
			.get(0)
			.ok_or_else(|| anyhow::anyhow!("No performance data"))?;

		// Get monthly performance trends
		let monthly_performance_query = "
			SELECT 
				time::format(created_at, '%Y-%m') as month,
				COUNT() as tests_taken,
				AVG(CASE WHEN is_correct = true THEN 100 ELSE 0 END) as average_score,
				COUNT(DISTINCT user) as active_users
			FROM app_answers 
			WHERE is_deleted = false 
			AND created_at >= time::now() - 1y
			GROUP BY month
			ORDER BY month DESC
		";

		let monthly_performance: Vec<Value> =
			db.query(monthly_performance_query).await?.take(0)?;
		let monthly_performance_data: Vec<MonthlyPerformanceDto> = monthly_performance
			.iter()
			.map(|item| MonthlyPerformanceDto {
				month: item["month"].as_str().unwrap_or("").to_string(),
				tests_taken: item["tests_taken"].as_u64().unwrap_or(0),
				average_score: item["average_score"].as_f64().unwrap_or(0.0),
				active_users: item["active_users"].as_u64().unwrap_or(0),
			})
			.collect();

		// Get top performing students
		let top_students_query = "
			SELECT 
				u.id as user_id,
				u.fullname,
				u.email,
				AVG(CASE WHEN a.is_correct = true THEN 100 ELSE 0 END) as average_score,
				COUNT(DISTINCT a.test) as tests_taken
			FROM app_users u
			JOIN app_answers a ON a.user = u.id
			WHERE u.is_deleted = false 
			AND a.is_deleted = false
			GROUP BY u.id, u.fullname, u.email
			HAVING tests_taken >= 3
			ORDER BY average_score DESC
			LIMIT 10
		";

		let top_students: Vec<Value> = db.query(top_students_query).await?.take(0)?;
		let top_students_data: Vec<TopStudentDto> = top_students
			.iter()
			.map(|item| TopStudentDto {
				user_id: item["user_id"].as_str().unwrap_or("").to_string(),
				fullname: item["fullname"].as_str().unwrap_or("").to_string(),
				email: item["email"].as_str().unwrap_or("").to_string(),
				average_score: item["average_score"].as_f64().unwrap_or(0.0),
				tests_taken: item["tests_taken"].as_u64().unwrap_or(0),
			})
			.collect();

		Ok(PerformanceStatsDto {
			monthly_active_users: monthly_active_count,
			total_test_attempts: performance_data["total_test_attempts"]
				.as_u64()
				.unwrap_or(0),
			overall_average_score: performance_data["overall_average_score"]
				.as_f64()
				.unwrap_or(0.0),
			completion_rate: performance_data["completion_rate"].as_f64().unwrap_or(0.0),
			monthly_performance: monthly_performance_data,
			top_performing_students: top_students_data,
		})
	}

	async fn query_content_stats(&self) -> Result<ContentStatsDto> {
		let db = &self.state.surrealdb_ws;

		// Get basic content counts
		let content_counts_query = "
			SELECT 
				(SELECT COUNT() FROM app_questions WHERE is_deleted = false) as total_questions,
				(SELECT COUNT() FROM app_options WHERE is_deleted = false) as total_options,
				(SELECT AVG(CASE WHEN is_correct = true THEN 100 ELSE 0 END) 
					FROM app_answers WHERE is_deleted = false) as average_correct_rate
			FROM (SELECT 1) as dummy
		";

		let content_counts: Vec<Value> =
			db.query(content_counts_query).await?.take(0)?;
		let content_count_data = content_counts
			.get(0)
			.ok_or_else(|| anyhow::anyhow!("No content count data"))?;

		// Get question difficulty analysis (based on correct answer rates)
		let question_difficulty_query = "
			SELECT 
				q.id as question_id,
				AVG(CASE WHEN a.is_correct = true THEN 100 ELSE 0 END) as correct_rate,
				COUNT(a.id) as answer_count
			FROM app_questions q
			LEFT JOIN app_answers a ON a.question = q.id
			WHERE q.is_deleted = false 
			AND (a.is_deleted = false OR a.id IS NULL)
			GROUP BY q.id
			HAVING answer_count > 0
		";

		let question_difficulty: Vec<Value> =
			db.query(question_difficulty_query).await?.take(0)?;
		let mut difficulty_counts = HashMap::new();
		difficulty_counts.insert("Easy".to_string(), (0u64, 0.0));
		difficulty_counts.insert("Medium".to_string(), (0u64, 0.0));
		difficulty_counts.insert("Hard".to_string(), (0u64, 0.0));

		for item in question_difficulty {
			let correct_rate = item["correct_rate"].as_f64().unwrap_or(0.0);
			let difficulty = if correct_rate >= 70.0 {
				"Easy"
			} else if correct_rate >= 40.0 {
				"Medium"
			} else {
				"Hard"
			};

			let entry = difficulty_counts
				.entry(difficulty.to_string())
				.or_insert((0, 0.0));
			entry.0 += 1;
			entry.1 += correct_rate;
		}

		let question_difficulty_data: Vec<QuestionDifficultyDto> = difficulty_counts
			.into_iter()
			.map(|(difficulty, (count, total_rate))| QuestionDifficultyDto {
				difficulty,
				count,
				correct_rate: if count > 0 {
					total_rate / count as f64
				} else {
					0.0
				},
			})
			.collect();

		// Get subject distribution
		let subject_distribution_query = "
			SELECT 
				t.subject,
				COUNT(DISTINCT q.id) as questions_count,
				COUNT(DISTINCT t.id) as tests_count
			FROM app_tests t
			JOIN app_questions q ON q.id IN t.questions
			WHERE t.is_deleted = false 
			AND q.is_deleted = false
			GROUP BY t.subject
			ORDER BY questions_count DESC
		";

		let subject_distribution: Vec<Value> =
			db.query(subject_distribution_query).await?.take(0)?;
		let subject_distribution_data: Vec<SubjectDistributionDto> =
			subject_distribution
				.iter()
				.map(|item| SubjectDistributionDto {
					subject: item["subject"].as_str().unwrap_or("Unknown").to_string(),
					questions_count: item["questions_count"].as_u64().unwrap_or(0),
					tests_count: item["tests_count"].as_u64().unwrap_or(0),
				})
				.collect();

		Ok(ContentStatsDto {
			total_questions: content_count_data["total_questions"].as_u64().unwrap_or(0),
			total_options: content_count_data["total_options"].as_u64().unwrap_or(0),
			average_correct_rate: content_count_data["average_correct_rate"]
				.as_f64()
				.unwrap_or(0.0),
			question_difficulty: question_difficulty_data,
			subject_distribution: subject_distribution_data,
		})
	}

	async fn query_system_stats(&self) -> Result<SystemStatsDto> {
		let db = &self.state.surrealdb_ws;

		// Get database metrics
		let db_metrics_query = "
			SELECT 
				(SELECT COUNT() FROM app_users WHERE is_deleted = false) as total_users,
				(SELECT COUNT() FROM app_tests WHERE is_deleted = false) as total_tests,
				(SELECT COUNT() FROM app_sessions WHERE is_deleted = false) as total_sessions,
				(SELECT COUNT() FROM app_questions WHERE is_deleted = false) as total_questions,
				(SELECT COUNT() FROM app_options WHERE is_deleted = false) as total_options,
				(SELECT COUNT() FROM app_answers WHERE is_deleted = false) as total_answers,
				(SELECT COUNT() FROM app_roles WHERE is_deleted = false) as total_roles,
				(SELECT COUNT() FROM app_permissions WHERE is_deleted = false) as total_permissions
			FROM (SELECT 1) as dummy
		";

		let db_metrics: Vec<Value> = db.query(db_metrics_query).await?.take(0)?;
		let db_metrics_data = db_metrics
			.get(0)
			.ok_or_else(|| anyhow::anyhow!("No database metrics data"))?;

		let database_metrics = DatabaseMetricsDto {
			total_users: db_metrics_data["total_users"].as_u64().unwrap_or(0),
			total_tests: db_metrics_data["total_tests"].as_u64().unwrap_or(0),
			total_sessions: db_metrics_data["total_sessions"].as_u64().unwrap_or(0),
			total_questions: db_metrics_data["total_questions"].as_u64().unwrap_or(0),
			total_options: db_metrics_data["total_options"].as_u64().unwrap_or(0),
			total_answers: db_metrics_data["total_answers"].as_u64().unwrap_or(0),
			total_roles: db_metrics_data["total_roles"].as_u64().unwrap_or(0),
			total_permissions: db_metrics_data["total_permissions"].as_u64().unwrap_or(0),
		};

		// Get growth metrics
		let growth_query = "
			SELECT 
				(SELECT COUNT() FROM app_users 
					WHERE is_deleted = false 
					AND created_at >= time::now() - 30d) as new_users_this_month,
				(SELECT COUNT() FROM app_users 
					WHERE is_deleted = false 
					AND created_at >= time::now() - 60d 
					AND created_at < time::now() - 30d) as new_users_last_month,
				(SELECT COUNT() FROM app_tests 
					WHERE is_deleted = false 
					AND created_at >= time::now() - 30d) as new_tests_this_month,
				(SELECT COUNT() FROM app_answers 
					WHERE is_deleted = false 
					AND created_at >= time::now() - 30d) as new_answers_this_month
			FROM (SELECT 1) as dummy
		";

		let growth: Vec<Value> = db.query(growth_query).await?.take(0)?;
		let growth_data = growth
			.get(0)
			.ok_or_else(|| anyhow::anyhow!("No growth data"))?;

		let new_users_this_month =
			growth_data["new_users_this_month"].as_u64().unwrap_or(0);
		let new_users_last_month =
			growth_data["new_users_last_month"].as_u64().unwrap_or(0);
		let new_tests_this_month =
			growth_data["new_tests_this_month"].as_u64().unwrap_or(0);

		let user_growth_rate = if new_users_last_month > 0 {
			((new_users_this_month as f64 - new_users_last_month as f64)
				/ new_users_last_month as f64)
				* 100.0
		} else {
			0.0
		};

		// Get monthly growth trends
		let monthly_growth_query = "
			SELECT 
				time::format(created_at, '%Y-%m') as month,
				COUNT() as new_users,
				0 as new_tests,
				0 as new_answers
			FROM app_users 
			WHERE is_deleted = false 
			AND created_at >= time::now() - 1y
			GROUP BY month
			ORDER BY month DESC
		";

		let monthly_growth: Vec<Value> =
			db.query(monthly_growth_query).await?.take(0)?;
		let monthly_growth_data: Vec<MonthlyGrowthDto> = monthly_growth
			.iter()
			.map(|item| MonthlyGrowthDto {
				month: item["month"].as_str().unwrap_or("").to_string(),
				new_users: item["new_users"].as_u64().unwrap_or(0),
				new_tests: new_tests_this_month,
				new_answers: growth_data["new_answers_this_month"].as_u64().unwrap_or(0),
			})
			.collect();

		let growth_metrics = GrowthMetricsDto {
			user_growth_rate,
			test_creation_rate: 0.0,
			activity_growth_rate: 0.0,
			monthly_growth: monthly_growth_data,
		};

		// Get data integrity metrics
		let integrity_query = "
			SELECT 
				(SELECT COUNT() FROM app_users WHERE is_deleted = true) as deleted_users,
				(SELECT COUNT() FROM app_tests WHERE is_deleted = true) as deleted_tests,
				(SELECT COUNT() FROM app_sessions WHERE is_deleted = true) as deleted_sessions,
				(SELECT COUNT() FROM app_questions WHERE is_deleted = true) as deleted_questions
			FROM (SELECT 1) as dummy
		";

		let integrity: Vec<Value> = db.query(integrity_query).await?.take(0)?;
		let integrity_data = integrity
			.get(0)
			.ok_or_else(|| anyhow::anyhow!("No integrity data"))?;

		let total_records = database_metrics.total_users
			+ database_metrics.total_tests
			+ database_metrics.total_sessions
			+ database_metrics.total_questions;
		let deleted_records = integrity_data["deleted_users"].as_u64().unwrap_or(0)
			+ integrity_data["deleted_tests"].as_u64().unwrap_or(0)
			+ integrity_data["deleted_sessions"].as_u64().unwrap_or(0)
			+ integrity_data["deleted_questions"].as_u64().unwrap_or(0);

		let data_integrity_score = if total_records > 0 {
			(total_records as f64 / (total_records + deleted_records) as f64) * 100.0
		} else {
			100.0
		};

		let data_integrity = DataIntegrityDto {
			deleted_users: integrity_data["deleted_users"].as_u64().unwrap_or(0),
			deleted_tests: integrity_data["deleted_tests"].as_u64().unwrap_or(0),
			deleted_sessions: integrity_data["deleted_sessions"].as_u64().unwrap_or(0),
			deleted_questions: integrity_data["deleted_questions"].as_u64().unwrap_or(0),
			data_integrity_score,
		};

		Ok(SystemStatsDto {
			database_metrics,
			growth_metrics,
			data_integrity,
		})
	}
}
