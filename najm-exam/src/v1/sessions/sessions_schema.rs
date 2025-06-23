use super::{SessionsCreateRequestDto, SessionsUpdateRequestDto};
use crate::TestsDetailSchema;
use najm_lib::ResourceEnum;
use najm_util::{get_iso_date, make_thing};
use serde::{Deserialize, Serialize};
use surrealdb::{Uuid, sql::Thing};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TestSessionsSchema {
	pub test: Thing,
	pub shuffle: bool,
	pub weight: String,
	pub multiplier: f64,
	pub timer: u32,
	pub start_date: String,
	pub end_date: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TestSessionsDetailSchema {
	pub test: TestsDetailSchema,
	pub weight: String,
	pub shuffle: bool,
	pub multiplier: f64,
	pub timer: u32,
	pub start_date: String,
	pub end_date: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SessionsSchema {
	pub id: Thing,
	pub name: String,
	pub tests: Vec<TestSessionsSchema>,
	pub category: String,
	pub banner: Option<String>,
	pub description: String,
	pub student_type: String,
	pub is_active: bool,
	pub is_deleted: bool,
	pub created_at: String,
	pub updated_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SessionsDetailSchema {
	pub id: Thing,
	pub name: String,
	pub tests: Vec<TestSessionsDetailSchema>,
	pub category: String,
	pub banner: Option<String>,
	pub description: String,
	pub student_type: String,
	pub is_active: bool,
	pub is_deleted: bool,
	pub created_at: String,
	pub updated_at: String,
}

impl Default for SessionsSchema {
	fn default() -> Self {
		Self {
			id: make_thing(
				&ResourceEnum::Sessions.to_string(),
				&Uuid::new_v4().to_string(),
			),
			name: String::new(),
			tests: Vec::new(),
			category: String::new(),
			banner: None,
			description: String::new(),
			student_type: String::new(),
			is_active: false,
			is_deleted: false,
			created_at: get_iso_date(),
			updated_at: get_iso_date(),
		}
	}
}

impl SessionsSchema {
	pub fn create(session: SessionsCreateRequestDto) -> Self {
		let tests = session
			.tests
			.into_iter()
			.map(|t| TestSessionsSchema {
				test: make_thing(&ResourceEnum::Tests.to_string(), &t.test_id),
				weight: t.weight,
				multiplier: t.multiplier,
				shuffle: t.shuffle,
				timer: t.timer,
				start_date: t.start_date,
				end_date: t.end_date,
			})
			.collect();

		Self {
			id: make_thing(
				&ResourceEnum::Sessions.to_string(),
				&Uuid::new_v4().to_string(),
			),
			name: session.name,
			tests,
			category: session.category,
			banner: session.banner,
			description: session.description,
			student_type: session.student_type,
			is_active: session.is_active,
			is_deleted: false,
			created_at: get_iso_date(),
			updated_at: get_iso_date(),
		}
	}

	pub fn update(session: SessionsUpdateRequestDto, id: String) -> Self {
		let tests = session
			.tests
			.into_iter()
			.map(|t| TestSessionsSchema {
				test: make_thing(&ResourceEnum::Tests.to_string(), &t.test_id),
				weight: t.weight,
				multiplier: t.multiplier,
				shuffle: t.shuffle,
				timer: t.timer,
				start_date: t.start_date,
				end_date: t.end_date,
			})
			.collect();

		Self {
			id: make_thing(&ResourceEnum::Sessions.to_string(), &id),
			name: session.name,
			tests,
			category: session.category,
			banner: session.banner,
			description: session.description,
			student_type: session.student_type,
			is_active: session.is_active,
			is_deleted: false,
			updated_at: get_iso_date(),
			..Default::default()
		}
	}
}
