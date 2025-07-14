use super::{
	SessionsDetailResponseDto, SessionsDetailSchema, SessionsResponseDto,
	SessionsSchema, StudentStatsResponseDto, MonthlyScoreDto, TestTakenDto,
};
use anyhow::{Result, bail};
use najm_lib::{AppState, MetaRequestDto, ResourceEnum, ResponseListSuccessDto};
use najm_util::{DetailQueryBuilder, QueryListBuilder, get_id, get_iso_date};

pub struct SessionsRepository<'a> {
	state: &'a AppState,
}

impl<'a> SessionsRepository<'a> {
	pub fn new(state: &'a AppState) -> Self {
		Self { state }
	}

	pub async fn query_session_list(
		&self,
		meta: MetaRequestDto,
	) -> Result<ResponseListSuccessDto<Vec<SessionsResponseDto>>> {
		let result: ResponseListSuccessDto<Vec<SessionsSchema>> = QueryListBuilder::new(
			&self.state.surrealdb_ws,
			&ResourceEnum::Sessions.to_string(),
			&meta,
		)
		.with_condition("is_deleted = false")
		.search_field("name")
		.select_fields(vec!["*"])
		.build()
		.await?;
		let data = result
			.data
			.into_iter()
			.map(SessionsResponseDto::from)
			.collect();
		Ok(ResponseListSuccessDto {
			data,
			meta: result.meta,
		})
	}

	pub async fn query_session_by_id(
		&self,
		id: String,
	) -> Result<SessionsDetailResponseDto> {
		let db = &self.state.surrealdb_ws;
		let builder = DetailQueryBuilder::new(ResourceEnum::Sessions.to_string())
			.with_id(&id)
			.with_select_fields(vec!["*"])
			.with_fetch("tests.test")
			.with_fetch("tests.test.questions")
			.with_fetch("tests.test.questions.options")
			.with_fetch("tests.test.sub_tests")
			.with_fetch("tests.test.sub_tests.questions")
			.with_fetch("tests.test.sub_tests.questions.options");
		let sql = builder.build();
		let result: Option<SessionsDetailSchema> =
			builder.apply_bindings(db.query(sql)).await?.take(0)?;
		let Some(session) = result else {
			bail!("Session not found");
		};
		if session.is_deleted {
			bail!("Session not found");
		}
		Ok(SessionsDetailResponseDto::from(session))
	}

	pub async fn query_create_session(&self, data: SessionsSchema) -> Result<String> {
		let db = &self.state.surrealdb_ws;
		let record: Option<SessionsSchema> = db
			.create(ResourceEnum::Sessions.to_string())
			.content(data)
			.await?;
		match record {
			Some(_) => Ok("Success create session".into()),
			None => bail!("Failed to create session"),
		}
	}

	pub async fn query_update_session(&self, data: SessionsSchema) -> Result<String> {
		let db = &self.state.surrealdb_ws;
		let record_key = get_id(&data.id)?;
		let existing = self.query_raw_session_by_id(&data.id.id.to_raw()).await?;
		if existing.is_deleted {
			bail!("Session already deleted");
		}
		let merged = SessionsSchema {
			created_at: existing.created_at,
			updated_at: get_iso_date(),
			..data.clone()
		};
		let record: Option<SessionsSchema> = db.update(record_key).merge(merged).await?;
		match record {
			Some(_) => Ok("Success update session".into()),
			None => bail!("Failed to update session"),
		}
	}

	pub async fn query_delete_session(&self, id: String) -> Result<String> {
		let db = &self.state.surrealdb_ws;
		let session = self.query_raw_session_by_id(&id).await?;
		if session.is_deleted {
			bail!("Session not found");
		}
		let record_key = get_id(&session.id)?;
		let record: Option<SessionsSchema> = db
			.update(record_key)
			.merge(serde_json::json!({ "is_deleted": true }))
			.await?;
		match record {
			Some(_) => Ok("Success delete session".into()),
			None => bail!("Failed to delete session"),
		}
	}

	pub async fn query_raw_session_by_id(&self, id: &str) -> Result<SessionsSchema> {
		let db = &self.state.surrealdb_ws;
		let session: Option<SessionsSchema> =
			db.select((ResourceEnum::Sessions.to_string(), id)).await?;
		match session {
			Some(s) if !s.is_deleted => Ok(s),
			_ => bail!("Session not found"),
		}
	}

	pub async fn query_student_stats(&self, user_id: &str) -> Result<StudentStatsResponseDto> {
		let db = &self.state.surrealdb_ws;
		
		// Query all answers for the user with test and session information
		// This query gets all answers for the user and joins with test and session data
		let answers_query = format!(
			r#"
			SELECT 
				a.*,
				t.name as test_name,
				t.category as test_category,
				s.name as session_name,
				s.id as session_id
			FROM app_answers a
			LET test = a.test,
			LET session = a.session
			WHERE a.user = app_users:⟨{}⟩ AND a.is_deleted = false
			ORDER BY a.created_at DESC
			"#,
			user_id
		);
		
		// Execute the query
		let answers: Vec<serde_json::Value> = db.query(&answers_query).await?.take(0)?;
		
		// Group answers by test and session to calculate scores
		let mut test_scores: std::collections::HashMap<String, (String, String, String, i32, String, String)> = std::collections::HashMap::new();
		let mut monthly_scores: std::collections::HashMap<String, Vec<i32>> = std::collections::HashMap::new();
		
		for answer in answers {
			let test_id = answer["test"].as_str().unwrap_or("");
			let test_name = answer["test_name"].as_str().unwrap_or("");
			let test_category = answer["test_category"].as_str().unwrap_or("");
			let session_id = answer["session_id"].as_str().unwrap_or("");
			let session_name = answer["session_name"].as_str().unwrap_or("");
			let created_at = answer["created_at"].as_str().unwrap_or("");
			let is_correct = answer["is_correct"].as_bool().unwrap_or(false);
			
			// Calculate score (simplified - you may need to adjust based on your scoring logic)
			let score = if is_correct { 1 } else { 0 };
			
			// Group by test
			let entry = test_scores.entry(test_id.to_string()).or_insert((
				test_name.to_string(),
				test_category.to_string(),
				session_id.to_string(),
				0,
				session_name.to_string(),
				created_at.to_string(),
			));
			entry.3 += score;
			
			// Group by month for monthly averages
			if let Some(date) = created_at.split('T').next() {
				let parts: Vec<&str> = date.split('-').collect();
				if parts.len() >= 2 {
					let month_key = format!("{}-{}", parts[0], parts[1]);
					monthly_scores.entry(month_key).or_insert_with(Vec::new).push(score);
				}
			}
		}
		
		// Convert to DTOs
		let tests_taken: Vec<TestTakenDto> = test_scores
			.into_iter()
			.map(|(test_id, (test_name, category, session_id, score, session_name, taken_at))| {
				TestTakenDto {
					test_id,
					test_name,
					category,
					score,
					taken_at,
					session_id,
					session_name,
				}
			})
			.collect();
		
		let average_score_per_month: Vec<MonthlyScoreDto> = monthly_scores
			.into_iter()
			.map(|(month, scores)| {
				let average = if scores.is_empty() {
					0.0
				} else {
					scores.iter().sum::<i32>() as f64 / scores.len() as f64
				};
				MonthlyScoreDto {
					month,
					average_score: average,
					tests_count: scores.len() as u32,
				}
			})
			.collect();
		
		let total_tests_taken = tests_taken.len() as u32;
		let average_score_overall = if total_tests_taken > 0 {
			tests_taken.iter().map(|t| t.score as f64).sum::<f64>() / total_tests_taken as f64
		} else {
			0.0
		};
		
		Ok(StudentStatsResponseDto {
			average_score_per_month,
			tests_taken,
			total_tests_taken,
			average_score_overall,
		})
	}
}
