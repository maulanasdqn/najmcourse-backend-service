use super::{TestsCreateRequestDto, TestsUpdateRequestDto};
use crate::QuestionsDetailSchema;
use najm_lib::ResourceEnum;
use najm_util::{get_iso_date, make_thing};
use serde::{Deserialize, Serialize};
use surrealdb::{Uuid, sql::Thing};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SubTestsSchema {
	pub id: Thing,
	pub name: String,
	pub questions: Vec<Thing>,
	pub banner: Option<String>,
	pub category: String,
	pub is_deleted: bool,
	pub created_at: String,
	pub updated_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SubTestQuestionSchema {
	pub id: Thing,
	pub question: Option<String>,
	pub discussion: Option<String>,
	pub question_image_url: Option<String>,
	pub discussion_image_url: Option<String>,
	pub options: Vec<SubTestOptionSchema>,
	pub is_deleted: bool,
	pub created_at: String,
	pub updated_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SubTestOptionSchema {
	pub id: Thing,
	pub label: Option<String>,
	pub points: Option<f32>,
	pub image_url: Option<String>,
	pub is_correct: bool,
	pub is_deleted: bool,
	pub created_at: String,
	pub updated_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TestsSchema {
	pub id: Thing,
	pub name: String,
	pub questions: Vec<Thing>,
	pub sub_tests: Vec<Thing>,
	pub banner: Option<String>,
	pub category: String,
	pub is_deleted: bool,
	pub created_at: String,
	pub updated_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TestsDetailSchema {
	pub id: Thing,
	pub name: String,
	pub questions: Option<Vec<QuestionsDetailSchema>>,
	pub sub_tests: Option<Vec<SubTestsDetailSchema>>,
	pub banner: Option<String>,
	pub category: String,
	pub is_deleted: bool,
	pub created_at: String,
	pub updated_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SubTestsDetailSchema {
	pub id: Thing,
	pub name: String,
	pub questions: Vec<QuestionsDetailSchema>,
	pub banner: Option<String>,
	pub category: String,
	pub is_deleted: bool,
	pub created_at: String,
	pub updated_at: String,
}

impl Default for TestsSchema {
	fn default() -> Self {
		Self {
			id: make_thing(
				&ResourceEnum::Tests.to_string(),
				&Uuid::new_v4().to_string(),
			),
			name: String::new(),
			questions: Vec::new(),
			sub_tests: Vec::new(),
			banner: None,
			category: String::new(),
			is_deleted: false,
			created_at: get_iso_date(),
			updated_at: get_iso_date(),
		}
	}
}

impl TestsSchema {
	pub fn create(test: TestsCreateRequestDto) -> Self {
		// For akademik: questions array should contain Thing references, sub_tests should be empty
		// For psikologi: questions should be empty, sub_tests array should contain Thing references
		let (questions, sub_tests) = match test.category.to_lowercase().as_str() {
			"akademik" => {
				// Questions will be populated with Thing references by the repository
				(Vec::new(), Vec::new())
			}
			"psikologi" => {
				// Sub-tests will be populated with Thing references by the repository
				(Vec::new(), Vec::new())
			}
			_ => {
				// Other categories have empty arrays
				(Vec::new(), Vec::new())
			}
		};

		Self {
			id: make_thing(
				&ResourceEnum::Tests.to_string(),
				&Uuid::new_v4().to_string(),
			),
			name: test.name,
			questions,
			sub_tests,
			banner: test.banner,
			category: test.category,
			is_deleted: false,
			created_at: get_iso_date(),
			updated_at: get_iso_date(),
		}
	}

	pub fn update(test: TestsUpdateRequestDto, id: String) -> Self {
		// For akademik: questions array should contain Thing references, sub_tests should be empty
		// For psikologi: questions should be empty, sub_tests array should contain Thing references
		let (questions, sub_tests) = match test.category.to_lowercase().as_str() {
			"akademik" => {
				// Questions will be populated with Thing references by the repository
				(Vec::new(), Vec::new())
			}
			"psikologi" => {
				// Sub-tests will be populated with Thing references by the repository
				(Vec::new(), Vec::new())
			}
			_ => {
				// Other categories have empty arrays
				(Vec::new(), Vec::new())
			}
		};

		Self {
			id: make_thing(&ResourceEnum::Tests.to_string(), &id),
			name: test.name,
			questions,
			sub_tests,
			banner: test.banner,
			category: test.category,
			is_deleted: false,
			updated_at: get_iso_date(),
			..Default::default()
		}
	}
}
