use super::{
	AchievementsDto, ActiveSessionDto, BestSubjectDto, CategoryPerformanceDto,
	ComparisonDataDto, CurrentSessionsDto, DailyStudyTimeDto, FocusAreaDto,
	MilestoneDto, MissedSessionDto, MonthlyProgressDto, NextStepDto, PerfectScoreDto,
	PerformanceMetricsDto, PersonalBestDto, PlatformComparisonDto, ProfileOverviewDto,
	QuestionStatsDto, RecentActivityDto, RecentTestDto, RecommendationsDto,
	ScoreDistributionDto, StreakDataDto, StudentDashboardResponseDto, StudyTimeDto,
	SubjectBreakdownDto, SubjectPerformanceDto, SuggestedScheduleDto,
	UpcomingSessionDto, WeakAreaDto,
};
use anyhow::Result;
use chrono::{DateTime, NaiveDate, Utc};
use najm_lib::AppState;
use serde_json::Value;

pub struct StudentStatsRepository<'a> {
	state: &'a AppState,
}

impl<'a> StudentStatsRepository<'a> {
	pub fn new(state: &'a AppState) -> Self {
		Self { state }
	}

	pub async fn query_student_dashboard(
		&self,
		user_id: &str,
	) -> Result<StudentDashboardResponseDto> {
		let (
			profile_overview,
			performance_metrics,
			current_sessions,
			subject_breakdown,
			achievements,
			recent_activity,
			recommendations,
			comparison_data,
		) = tokio::try_join!(
			self.query_profile_overview(user_id),
			self.query_performance_metrics(user_id),
			self.query_current_sessions(user_id),
			self.query_subject_breakdown(user_id),
			self.query_achievements(user_id),
			self.query_recent_activity(user_id),
			self.query_recommendations(user_id),
			self.query_comparison_data(user_id),
		)?;

		Ok(StudentDashboardResponseDto {
			profile_overview,
			performance_metrics,
			current_sessions,
			subject_breakdown,
			achievements,
			recent_activity,
			recommendations,
			comparison_data,
		})
	}

	async fn query_profile_overview(
		&self,
		user_id: &str,
	) -> Result<ProfileOverviewDto> {
		let db = &self.state.surrealdb_ws;

		let user_query = format!(
			"SELECT * FROM app_users WHERE id = app_users:⟨{}⟩ AND is_deleted = false",
			user_id
		);

		let users: Vec<Value> = db.query(&user_query).await?.take(0)?;
		let user = users
			.get(0)
			.ok_or_else(|| anyhow::anyhow!("User not found"))?;

		// Calculate days since registration
		let created_at = user["created_at"].as_str().unwrap_or("");
		let created_date = DateTime::parse_from_rfc3339(created_at)
			.unwrap_or_else(|_| DateTime::from(Utc::now()));
		let days_since_registration =
			(Utc::now() - created_date.with_timezone(&Utc)).num_days() as u64;

		// Get last activity
		let last_activity_query = format!(
			"SELECT created_at FROM app_answers 
			WHERE user = app_users:⟨{}⟩ AND is_deleted = false 
			ORDER BY created_at DESC LIMIT 1",
			user_id
		);

		let last_activities: Vec<Value> =
			db.query(&last_activity_query).await?.take(0)?;
		let last_active_date = last_activities
			.get(0)
			.and_then(|a| a["created_at"].as_str())
			.map(|s| s.to_string());

		Ok(ProfileOverviewDto {
			user_id: user_id.to_string(),
			fullname: user["fullname"].as_str().unwrap_or("").to_string(),
			email: user["email"].as_str().unwrap_or("").to_string(),
			is_profile_completed: user["is_profile_completed"].as_bool().unwrap_or(false),
			is_payment_completed: user["is_payment_completed"].as_bool().unwrap_or(false),
			days_since_registration,
			last_active_date,
			referral_code: user["referral_code"].as_str().map(|s| s.to_string()),
			student_type: user["student_type"].as_str().map(|s| s.to_string()),
		})
	}

	async fn query_performance_metrics(
		&self,
		user_id: &str,
	) -> Result<PerformanceMetricsDto> {
		let db = &self.state.surrealdb_ws;

		// Get overall performance stats
		let performance_query = format!(
			r#"
			SELECT 
				COUNT() as total_questions,
				COUNT(is_correct = true) as correct_answers,
				(COUNT(is_correct = true) * 100.0 / COUNT()) as accuracy_rate
			FROM app_answers 
			WHERE user = app_users:⟨{}⟩ AND is_deleted = false
			"#,
			user_id
		);

		let performance: Vec<Value> = db.query(&performance_query).await?.take(0)?;
		let perf_data = performance
			.get(0)
			.ok_or_else(|| anyhow::anyhow!("No performance data"))?;

		let overall_average_score = perf_data["accuracy_rate"].as_f64().unwrap_or(0.0);

		// Get test completion stats
		let test_stats_query = format!(
			r#"
			SELECT 
				COUNT(DISTINCT test) as tests_completed,
				COUNT(DISTINCT session) as sessions_started
			FROM app_answers 
			WHERE user = app_users:⟨{}⟩ AND is_deleted = false
			"#,
			user_id
		);

		let test_stats: Vec<Value> = db.query(&test_stats_query).await?.take(0)?;
		let test_data = test_stats
			.get(0)
			.ok_or_else(|| anyhow::anyhow!("No test data"))?;

		let tests_completed = test_data["tests_completed"].as_u64().unwrap_or(0) as u32;
		let tests_started = test_data["sessions_started"].as_u64().unwrap_or(0) as u32;
		let completion_rate = if tests_started > 0 {
			(tests_completed as f64 / tests_started as f64) * 100.0
		} else {
			0.0
		};

		// Calculate improvement percentage (compare last month to previous month)
		let improvement_query = format!(
			r#"
			LET current_month = (
				SELECT AVG(CASE WHEN is_correct = true THEN 100 ELSE 0 END) as avg_score
				FROM app_answers
				WHERE user = app_users:⟨{}⟩ 
				AND is_deleted = false
				AND created_at >= time::now() - 30d
			);
			LET previous_month = (
				SELECT AVG(CASE WHEN is_correct = true THEN 100 ELSE 0 END) as avg_score
				FROM app_answers
				WHERE user = app_users:⟨{}⟩ 
				AND is_deleted = false
				AND created_at >= time::now() - 60d
				AND created_at < time::now() - 30d
			);
			SELECT 
				$current_month[0].avg_score as current_avg,
				$previous_month[0].avg_score as previous_avg
			"#,
			user_id, user_id
		);

		let improvement: Vec<Value> = db.query(&improvement_query).await?.take(0)?;
		let improvement_data = improvement.get(0);

		let current_avg = improvement_data
			.and_then(|d| d["current_avg"].as_f64())
			.unwrap_or(overall_average_score);
		let previous_avg = improvement_data
			.and_then(|d| d["previous_avg"].as_f64())
			.unwrap_or(current_avg);

		let improvement_percentage = if previous_avg > 0.0 {
			((current_avg - previous_avg) / previous_avg) * 100.0
		} else {
			0.0
		};

		let trend_direction = if improvement_percentage > 0.0 {
			"positive"
		} else if improvement_percentage < 0.0 {
			"negative"
		} else {
			"stable"
		}
		.to_string();

		// Get score distribution
		let distribution_query = format!(
			r#"
			SELECT 
				test,
				(COUNT(is_correct = true) * 100.0 / COUNT()) as score
			FROM app_answers 
			WHERE user = app_users:⟨{}⟩ AND is_deleted = false
			GROUP BY test
			"#,
			user_id
		);

		let distribution: Vec<Value> = db.query(&distribution_query).await?.take(0)?;
		let mut score_dist = ScoreDistributionDto {
			range_0_20: 0,
			range_21_40: 0,
			range_41_60: 0,
			range_61_80: 0,
			range_81_100: 0,
		};

		for item in distribution {
			let score = item["score"].as_f64().unwrap_or(0.0);
			if score <= 20.0 {
				score_dist.range_0_20 += 1;
			} else if score <= 40.0 {
				score_dist.range_21_40 += 1;
			} else if score <= 60.0 {
				score_dist.range_41_60 += 1;
			} else if score <= 80.0 {
				score_dist.range_61_80 += 1;
			} else {
				score_dist.range_81_100 += 1;
			}
		}

		// Calculate percentile
		let percentile_query = format!(
			r#"
			LET my_score = {};
			LET all_scores = (
				SELECT 
					user,
					AVG(CASE WHEN is_correct = true THEN 100 ELSE 0 END) as avg_score
				FROM app_answers
				WHERE is_deleted = false
				GROUP BY user
			);
			SELECT COUNT() as users_below
			FROM $all_scores
			WHERE avg_score < $my_score
			"#,
			overall_average_score
		);

		let percentile: Vec<Value> = db.query(&percentile_query).await?.take(0)?;
		let users_below = percentile
			.get(0)
			.and_then(|p| p["users_below"].as_u64())
			.unwrap_or(0);

		// For simplicity, assume 100 total users for percentile calculation
		let student_percentile = ((users_below as f64 / 100.0) * 100.0) as u32;

		Ok(PerformanceMetricsDto {
			overall_average_score,
			improvement_percentage,
			trend_direction,
			score_distribution: score_dist,
			tests_completed,
			tests_started,
			completion_rate,
			average_time_per_test: 3600, // Default 1 hour
			student_percentile,
		})
	}

	async fn query_current_sessions(
		&self,
		user_id: &str,
	) -> Result<CurrentSessionsDto> {
		let db = &self.state.surrealdb_ws;
		let now = Utc::now();

		// Get active sessions
		let active_query = format!(
			r#"
			SELECT 
				s.*,
				(SELECT COUNT() FROM app_answers WHERE session = s.id AND user = app_users:⟨{}⟩) as answer_count,
				(SELECT COUNT(DISTINCT test) FROM s.tests) as total_tests
			FROM app_sessions s
			WHERE s.is_deleted = false 
			AND s.is_active = true
			AND time::now() BETWEEN s.tests[0].start_date AND s.tests[0].end_date
			ORDER BY s.created_at DESC
			"#,
			user_id
		);

		let active_sessions: Vec<Value> = db.query(&active_query).await?.take(0)?;

		let active_sessions_dto: Vec<ActiveSessionDto> = active_sessions
			.into_iter()
			.map(|session| {
				let end_date = session["tests"][0]["end_date"].as_str().unwrap_or("");
				let end_datetime = DateTime::parse_from_rfc3339(end_date)
					.unwrap_or_else(|_| DateTime::from(Utc::now()));
				let days_remaining =
					(end_datetime.with_timezone(&Utc) - now).num_days().max(0) as u32;

				let answer_count = session["answer_count"].as_u64().unwrap_or(0);
				let total_tests = session["total_tests"].as_u64().unwrap_or(1);
				let progress_percentage = if total_tests > 0 {
					((answer_count as f64 / total_tests as f64) * 100.0) as u32
				} else {
					0
				};

				ActiveSessionDto {
					session_id: session["id"].as_str().unwrap_or("").to_string(),
					session_name: session["name"].as_str().unwrap_or("").to_string(),
					category: session["category"].as_str().unwrap_or("").to_string(),
					end_date: end_date.to_string(),
					days_remaining,
					progress_percentage,
					is_started: answer_count > 0,
				}
			})
			.collect();

		// Get upcoming sessions
		let upcoming_query = r#"
			SELECT *
			FROM app_sessions
			WHERE is_deleted = false 
			AND is_active = true
			AND time::now() < tests[0].start_date
			ORDER BY tests[0].start_date ASC
			LIMIT 5
		"#;

		let upcoming_sessions: Vec<Value> = db.query(upcoming_query).await?.take(0)?;

		let upcoming_sessions_dto: Vec<UpcomingSessionDto> = upcoming_sessions
			.into_iter()
			.map(|session| {
				let start_date = session["tests"][0]["start_date"].as_str().unwrap_or("");
				let start_datetime = DateTime::parse_from_rfc3339(start_date)
					.unwrap_or_else(|_| DateTime::from(Utc::now()));
				let days_until_start =
					(start_datetime.with_timezone(&Utc) - now).num_days().max(0) as u32;

				UpcomingSessionDto {
					session_id: session["id"].as_str().unwrap_or("").to_string(),
					session_name: session["name"].as_str().unwrap_or("").to_string(),
					category: session["category"].as_str().unwrap_or("").to_string(),
					start_date: start_date.to_string(),
					days_until_start,
				}
			})
			.collect();

		// Get missed sessions (ended in last 30 days that user didn't participate in)
		let missed_query = format!(
			r#"
			SELECT s.*
			FROM app_sessions s
			WHERE s.is_deleted = false 
			AND time::now() > s.tests[0].end_date
			AND s.tests[0].end_date > time::now() - 30d
			AND NOT EXISTS (
				SELECT 1 FROM app_answers 
				WHERE session = s.id AND user = app_users:⟨{}⟩
			)
			ORDER BY s.tests[0].end_date DESC
			LIMIT 5
			"#,
			user_id
		);

		let missed_sessions: Vec<Value> = db.query(&missed_query).await?.take(0)?;

		let missed_sessions_dto: Vec<MissedSessionDto> = missed_sessions
			.into_iter()
			.map(|session| MissedSessionDto {
				session_id: session["id"].as_str().unwrap_or("").to_string(),
				session_name: session["name"].as_str().unwrap_or("").to_string(),
				category: session["category"].as_str().unwrap_or("").to_string(),
				end_date: session["tests"][0]["end_date"]
					.as_str()
					.unwrap_or("")
					.to_string(),
			})
			.collect();

		Ok(CurrentSessionsDto {
			active_sessions: active_sessions_dto,
			upcoming_sessions: upcoming_sessions_dto,
			missed_sessions: missed_sessions_dto,
		})
	}

	async fn query_subject_breakdown(
		&self,
		user_id: &str,
	) -> Result<SubjectBreakdownDto> {
		let db = &self.state.surrealdb_ws;

		// Get performance by subject
		let subject_query = format!(
			r#"
			SELECT 
				t.subject,
				COUNT(a.id) as questions_attempted,
				COUNT(a.is_correct = true) as questions_correct,
				AVG(CASE WHEN a.is_correct = true THEN 100 ELSE 0 END) as average_score,
				COUNT(DISTINCT a.test) as tests_taken
			FROM app_answers a
			JOIN app_tests t ON a.test = t.id
			WHERE a.user = app_users:⟨{}⟩ 
			AND a.is_deleted = false
			AND t.is_deleted = false
			GROUP BY t.subject
			ORDER BY average_score DESC
			"#,
			user_id
		);

		let subjects: Vec<Value> = db.query(&subject_query).await?.take(0)?;

		let subjects_dto: Vec<SubjectPerformanceDto> = subjects
			.into_iter()
			.map(|item| {
				let questions_attempted =
					item["questions_attempted"].as_u64().unwrap_or(0) as u32;
				let questions_correct =
					item["questions_correct"].as_u64().unwrap_or(0) as u32;
				let accuracy_rate = if questions_attempted > 0 {
					(questions_correct as f64 / questions_attempted as f64) * 100.0
				} else {
					0.0
				};

				SubjectPerformanceDto {
					subject: item["subject"].as_str().unwrap_or("Unknown").to_string(),
					average_score: item["average_score"].as_f64().unwrap_or(0.0),
					questions_attempted,
					questions_correct,
					accuracy_rate,
					tests_taken: item["tests_taken"].as_u64().unwrap_or(0) as u32,
				}
			})
			.collect();

		// Get performance by category
		let category_query = format!(
			r#"
			SELECT 
				t.category,
				AVG(CASE WHEN a.is_correct = true THEN 100 ELSE 0 END) as average_score,
				COUNT(DISTINCT a.test) as tests_taken,
				SUM(CASE WHEN a.created_at >= time::now() - 7d THEN 1 ELSE 0 END) * 60 as time_spent_seconds
			FROM app_answers a
			JOIN app_tests t ON a.test = t.id
			WHERE a.user = app_users:⟨{}⟩ 
			AND a.is_deleted = false
			AND t.is_deleted = false
			GROUP BY t.category
			"#,
			user_id
		);

		let categories: Vec<Value> = db.query(&category_query).await?.take(0)?;

		let categories_dto: Vec<CategoryPerformanceDto> = categories
			.into_iter()
			.map(|item| CategoryPerformanceDto {
				category: item["category"].as_str().unwrap_or("Unknown").to_string(),
				average_score: item["average_score"].as_f64().unwrap_or(0.0),
				time_spent_seconds: item["time_spent_seconds"].as_u64().unwrap_or(0) as u32,
				tests_taken: item["tests_taken"].as_u64().unwrap_or(0) as u32,
			})
			.collect();

		// Identify weak areas (topics with < 60% accuracy)
		let weak_areas: Vec<WeakAreaDto> = subjects_dto
			.iter()
			.filter(|s| s.accuracy_rate < 60.0)
			.map(|s| WeakAreaDto {
				topic: format!("{} - General", s.subject),
				subject: s.subject.clone(),
				accuracy_rate: s.accuracy_rate,
				questions_attempted: s.questions_attempted,
			})
			.collect();

		Ok(SubjectBreakdownDto {
			subjects: subjects_dto,
			categories: categories_dto,
			weak_areas,
		})
	}

	async fn query_achievements(&self, user_id: &str) -> Result<AchievementsDto> {
		let db = &self.state.surrealdb_ws;

		// Get milestones
		let milestones_query = format!(
			r#"
			LET first_test = (
				SELECT MIN(created_at) as date 
				FROM app_answers 
				WHERE user = app_users:⟨{}⟩
			);
			LET test_count = (
				SELECT COUNT(DISTINCT test) as count 
				FROM app_answers 
				WHERE user = app_users:⟨{}⟩
			);
			SELECT 
				$first_test[0].date as first_test_date,
				$test_count[0].count as total_tests
			"#,
			user_id, user_id
		);

		let milestones_data: Vec<Value> = db.query(&milestones_query).await?.take(0)?;
		let mut milestones = Vec::new();

		if let Some(data) = milestones_data.get(0) {
			if let Some(first_test_date) = data["first_test_date"].as_str() {
				milestones.push(MilestoneDto {
					milestone_type: "first_test".to_string(),
					title: "Completed First Test".to_string(),
					description: "You completed your first test!".to_string(),
					achieved_at: first_test_date.to_string(),
				});
			}

			let total_tests = data["total_tests"].as_u64().unwrap_or(0);
			if total_tests >= 10 {
				milestones.push(MilestoneDto {
					milestone_type: "10_tests".to_string(),
					title: "10 Tests Completed".to_string(),
					description: "You've completed 10 tests!".to_string(),
					achieved_at: Utc::now().to_rfc3339(),
				});
			}
			if total_tests >= 50 {
				milestones.push(MilestoneDto {
					milestone_type: "50_tests".to_string(),
					title: "50 Tests Completed".to_string(),
					description: "Amazing! You've completed 50 tests!".to_string(),
					achieved_at: Utc::now().to_rfc3339(),
				});
			}
		}

		// Calculate streaks
		let streak_query = format!(
			r#"
			SELECT 
				DATE(created_at) as test_date,
				COUNT() as test_count
			FROM app_answers
			WHERE user = app_users:⟨{}⟩ AND is_deleted = false
			GROUP BY test_date
			ORDER BY test_date DESC
			"#,
			user_id
		);

		let streak_data: Vec<Value> = db.query(&streak_query).await?.take(0)?;
		let mut current_streak = 0;
		let mut longest_streak = 0;
		let mut streak_start_date = None;
		let mut last_date: Option<NaiveDate> = None;

		for (i, item) in streak_data.iter().enumerate() {
			if let Some(date_str) = item["test_date"].as_str() {
				if let Ok(date) = NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
					if i == 0 {
						// Check if streak continues to today
						let today = Utc::now().date_naive();
						if (today - date).num_days() <= 1 {
							current_streak = 1;
							streak_start_date = Some(date_str.to_string());
						}
					}

					if let Some(last) = last_date {
						if (last - date).num_days() == 1 {
							if i == 0 || (i == 1 && current_streak > 0) {
								current_streak += 1;
							}
						} else if current_streak > 0 {
							longest_streak = longest_streak.max(current_streak);
							if i > 0 {
								current_streak = 0;
							}
						}
					}

					last_date = Some(date);
				}
			}
		}

		longest_streak = longest_streak.max(current_streak);

		let streak_data_dto = StreakDataDto {
			current_streak_days: current_streak as u32,
			longest_streak_days: longest_streak as u32,
			streak_start_date,
		};

		// Get perfect scores
		let perfect_scores_query = format!(
			r#"
			SELECT 
				t.id as test_id,
				t.name as test_name,
				MIN(a.created_at) as achieved_at,
				COUNT(a.is_correct = true) as correct_count,
				COUNT() as total_count
			FROM app_answers a
			JOIN app_tests t ON a.test = t.id
			WHERE a.user = app_users:⟨{}⟩ AND a.is_deleted = false
			GROUP BY t.id, t.name
			HAVING correct_count = total_count
			ORDER BY achieved_at DESC
			LIMIT 5
			"#,
			user_id
		);

		let perfect_scores: Vec<Value> =
			db.query(&perfect_scores_query).await?.take(0)?;

		let perfect_scores_dto: Vec<PerfectScoreDto> = perfect_scores
			.into_iter()
			.map(|item| PerfectScoreDto {
				test_id: item["test_id"].as_str().unwrap_or("").to_string(),
				test_name: item["test_name"].as_str().unwrap_or("").to_string(),
				achieved_at: item["achieved_at"].as_str().unwrap_or("").to_string(),
			})
			.collect();

		Ok(AchievementsDto {
			milestones,
			streaks: streak_data_dto,
			perfect_scores: perfect_scores_dto,
		})
	}

	async fn query_recent_activity(&self, user_id: &str) -> Result<RecentActivityDto> {
		let db = &self.state.surrealdb_ws;

		// Get last 5 tests
		let recent_tests_query = format!(
			r#"
			SELECT 
				t.id as test_id,
				t.name as test_name,
				COUNT(a.is_correct = true) * 100.0 / COUNT() as score,
				MIN(a.created_at) as taken_at,
				(MAX(a.created_at) - MIN(a.created_at)) as duration
			FROM app_answers a
			JOIN app_tests t ON a.test = t.id
			WHERE a.user = app_users:⟨{}⟩ AND a.is_deleted = false
			GROUP BY t.id, t.name
			ORDER BY taken_at DESC
			LIMIT 5
			"#,
			user_id
		);

		let recent_tests: Vec<Value> = db.query(&recent_tests_query).await?.take(0)?;

		let last_tests: Vec<RecentTestDto> = recent_tests
			.into_iter()
			.map(|item| RecentTestDto {
				test_id: item["test_id"].as_str().unwrap_or("").to_string(),
				test_name: item["test_name"].as_str().unwrap_or("").to_string(),
				score: item["score"].as_f64().unwrap_or(0.0) as i32,
				taken_at: item["taken_at"].as_str().unwrap_or("").to_string(),
				duration_seconds: 3600, // Default 1 hour
			})
			.collect();

		// Get study time for last 7 days
		let study_time_query = format!(
			r#"
			SELECT 
				DATE(created_at) as study_date,
				COUNT() * 60 as seconds
			FROM app_answers
			WHERE user = app_users:⟨{}⟩ 
			AND is_deleted = false
			AND created_at >= time::now() - 7d
			GROUP BY study_date
			ORDER BY study_date DESC
			"#,
			user_id
		);

		let study_time_data: Vec<Value> = db.query(&study_time_query).await?.take(0)?;

		let mut total_seconds = 0u32;
		let daily_breakdown: Vec<DailyStudyTimeDto> = study_time_data
			.into_iter()
			.map(|item| {
				let seconds = item["seconds"].as_u64().unwrap_or(0) as u32;
				total_seconds += seconds;
				DailyStudyTimeDto {
					date: item["study_date"].as_str().unwrap_or("").to_string(),
					seconds,
				}
			})
			.collect();

		let study_time_last_7_days = StudyTimeDto {
			total_seconds,
			daily_breakdown,
		};

		// Get question stats
		let question_stats_query = format!(
			r#"
			SELECT 
				COUNT() as total_answered,
				COUNT(is_correct = true) as total_correct
			FROM app_answers
			WHERE user = app_users:⟨{}⟩ AND is_deleted = false
			"#,
			user_id
		);

		let question_stats_data: Vec<Value> =
			db.query(&question_stats_query).await?.take(0)?;
		let stats_data = question_stats_data
			.get(0)
			.ok_or_else(|| anyhow::anyhow!("No question stats"))?;

		let total_answered = stats_data["total_answered"].as_u64().unwrap_or(0) as u32;
		let total_correct = stats_data["total_correct"].as_u64().unwrap_or(0) as u32;
		let accuracy_rate = if total_answered > 0 {
			(total_correct as f64 / total_answered as f64) * 100.0
		} else {
			0.0
		};

		let question_stats = QuestionStatsDto {
			total_answered,
			total_correct,
			accuracy_rate,
		};

		Ok(RecentActivityDto {
			last_tests,
			study_time_last_7_days,
			question_stats,
		})
	}

	async fn query_recommendations(
		&self,
		user_id: &str,
	) -> Result<RecommendationsDto> {
		let db = &self.state.surrealdb_ws;

		// Get weak subjects for recommendations
		let weak_subjects_query = format!(
			r#"
			SELECT 
				t.subject,
				AVG(CASE WHEN a.is_correct = true THEN 100 ELSE 0 END) as accuracy
			FROM app_answers a
			JOIN app_tests t ON a.test = t.id
			WHERE a.user = app_users:⟨{}⟩ 
			AND a.is_deleted = false
			GROUP BY t.subject
			HAVING accuracy < 60
			ORDER BY accuracy ASC
			LIMIT 3
			"#,
			user_id
		);

		let weak_subjects: Vec<Value> = db.query(&weak_subjects_query).await?.take(0)?;

		// Generate next steps based on performance
		let mut next_steps = Vec::new();

		// Recommend tests for weak subjects
		for (i, subject) in weak_subjects.iter().enumerate() {
			if i < 2 {
				let subject_name = subject["subject"].as_str().unwrap_or("Unknown");
				next_steps.push(NextStepDto {
					action_type: "test".to_string(),
					title: format!("Practice {} Fundamentals", subject_name),
					reason: format!(
						"Your accuracy in {} is below 60%. Practice will help improve.",
						subject_name
					),
					resource_id: None,
				});
			}
		}

		// Add general recommendation
		next_steps.push(NextStepDto {
			action_type: "review".to_string(),
			title: "Review Recent Mistakes".to_string(),
			reason: "Analyzing your errors helps prevent repeating them".to_string(),
			resource_id: None,
		});

		// Generate focus areas
		let focus_areas: Vec<FocusAreaDto> = weak_subjects
			.into_iter()
			.enumerate()
			.map(|(i, subject)| {
				let priority = if i == 0 {
					"high"
				} else if i == 1 {
					"medium"
				} else {
					"low"
				};
				FocusAreaDto {
					topic: format!(
						"{} Concepts",
						subject["subject"].as_str().unwrap_or("Unknown")
					),
					subject: subject["subject"].as_str().unwrap_or("Unknown").to_string(),
					priority: priority.to_string(),
					reason: format!(
						"Accuracy: {:.1}%",
						subject["accuracy"].as_f64().unwrap_or(0.0)
					),
				}
			})
			.collect();

		// Generate suggested schedule
		let days = vec!["Monday", "Tuesday", "Wednesday", "Thursday", "Friday"];
		let subjects = vec!["Matematika", "Fisika", "Kimia", "Biologi", "Bahasa"];

		let suggested_schedule: Vec<SuggestedScheduleDto> = days
			.into_iter()
			.zip(subjects.into_iter().cycle())
			.map(|(day, subject)| SuggestedScheduleDto {
				day: day.to_string(),
				subject: subject.to_string(),
				suggested_minutes: 120,
			})
			.collect();

		Ok(RecommendationsDto {
			next_steps,
			focus_areas,
			suggested_schedule,
		})
	}

	async fn query_comparison_data(&self, user_id: &str) -> Result<ComparisonDataDto> {
		let db = &self.state.surrealdb_ws;

		// Get personal best
		let personal_best_query = format!(
			r#"
			SELECT 
				t.id as test_id,
				t.name as test_name,
				t.subject,
				COUNT(a.is_correct = true) * 100.0 / COUNT() as score,
				MIN(a.created_at) as achieved_at
			FROM app_answers a
			JOIN app_tests t ON a.test = t.id
			WHERE a.user = app_users:⟨{}⟩ AND a.is_deleted = false
			GROUP BY t.id, t.name, t.subject
			ORDER BY score DESC
			LIMIT 1
			"#,
			user_id
		);

		let best_test: Vec<Value> = db.query(&personal_best_query).await?.take(0)?;
		let best_data = best_test.get(0);

		// Get best subjects
		let best_subjects_query = format!(
			r#"
			SELECT 
				t.subject,
				MAX(COUNT(a.is_correct = true) * 100.0 / COUNT()) as highest_score
			FROM app_answers a
			JOIN app_tests t ON a.test = t.id
			WHERE a.user = app_users:⟨{}⟩ AND a.is_deleted = false
			GROUP BY t.subject
			ORDER BY highest_score DESC
			LIMIT 3
			"#,
			user_id
		);

		let best_subjects_data: Vec<Value> =
			db.query(&best_subjects_query).await?.take(0)?;

		let best_subjects: Vec<BestSubjectDto> = best_subjects_data
			.into_iter()
			.map(|item| BestSubjectDto {
				subject: item["subject"].as_str().unwrap_or("Unknown").to_string(),
				highest_score: item["highest_score"].as_f64().unwrap_or(0.0) as i32,
			})
			.collect();

		let personal_best = PersonalBestDto {
			highest_score: best_data.and_then(|d| d["score"].as_f64()).unwrap_or(0.0)
				as i32,
			best_test_name: best_data
				.and_then(|d| d["test_name"].as_str())
				.unwrap_or("N/A")
				.to_string(),
			achieved_at: best_data
				.and_then(|d| d["achieved_at"].as_str())
				.unwrap_or("")
				.to_string(),
			best_subjects,
		};

		// Get platform comparison
		let your_avg_query = format!(
			r#"
			SELECT AVG(CASE WHEN is_correct = true THEN 100 ELSE 0 END) as avg_score
			FROM app_answers
			WHERE user = app_users:⟨{}⟩ AND is_deleted = false
			"#,
			user_id
		);

		let platform_avg_query = r#"
			SELECT AVG(CASE WHEN is_correct = true THEN 100 ELSE 0 END) as avg_score
			FROM app_answers
			WHERE is_deleted = false
		"#;

		let your_avg: Vec<Value> = db.query(&your_avg_query).await?.take(0)?;
		let platform_avg: Vec<Value> = db.query(platform_avg_query).await?.take(0)?;

		let your_average = your_avg
			.get(0)
			.and_then(|d| d["avg_score"].as_f64())
			.unwrap_or(0.0);
		let platform_average = platform_avg
			.get(0)
			.and_then(|d| d["avg_score"].as_f64())
			.unwrap_or(0.0);

		let position = if your_average > platform_average {
			"above"
		} else if your_average < platform_average {
			"below"
		} else {
			"equal"
		}
		.to_string();

		let difference_percentage = if platform_average > 0.0 {
			((your_average - platform_average) / platform_average).abs() * 100.0
		} else {
			0.0
		};

		let platform_comparison = PlatformComparisonDto {
			your_average,
			platform_average,
			position,
			difference_percentage,
		};

		// Get monthly progress
		let monthly_progress_query = format!(
			r#"
			SELECT 
				time::format(created_at, '%Y-%m') as month,
				AVG(CASE WHEN is_correct = true THEN 100 ELSE 0 END) as average_score,
				COUNT(DISTINCT test) as tests_taken
			FROM app_answers
			WHERE user = app_users:⟨{}⟩ 
			AND is_deleted = false
			AND created_at >= time::now() - 6m
			GROUP BY month
			ORDER BY month DESC
			"#,
			user_id
		);

		let monthly_data: Vec<Value> =
			db.query(&monthly_progress_query).await?.take(0)?;

		let mut monthly_progress: Vec<MonthlyProgressDto> = Vec::new();
		let mut previous_score = None;

		for item in monthly_data.into_iter().rev() {
			let average_score = item["average_score"].as_f64().unwrap_or(0.0);
			let improvement = if let Some(prev) = previous_score {
				if prev > 0.0 {
					((average_score - prev) / prev) * 100.0
				} else {
					0.0
				}
			} else {
				0.0
			};

			monthly_progress.push(MonthlyProgressDto {
				month: item["month"].as_str().unwrap_or("").to_string(),
				average_score,
				tests_taken: item["tests_taken"].as_u64().unwrap_or(0) as u32,
				improvement_from_previous: improvement,
			});

			previous_score = Some(average_score);
		}

		Ok(ComparisonDataDto {
			personal_best,
			platform_comparison,
			monthly_progress,
		})
	}
}
