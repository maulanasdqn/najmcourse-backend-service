use super::tests_schema::TestsSchema;
use crate::{
	QuestionsCreateRequestDto, QuestionsUpdateRequestDto, questions::QuestionsItemDto,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct SubTestsCreateRequestDto {
	#[validate(length(min = 1, message = "Name must not be empty"))]
	pub name: String,

	pub questions: Vec<QuestionsCreateRequestDto>,

	pub banner: Option<String>,

	pub category: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct SubTestsUpdateRequestDto {
	pub id: String,

	#[validate(length(min = 1, message = "Name must not be empty"))]
	pub name: String,

	pub questions: Vec<QuestionsUpdateRequestDto>,

	pub banner: Option<String>,

	pub category: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct SubTestsItemDto {
	pub id: String,
	pub name: String,
	pub banner: Option<String>,
	pub category: String,
	pub questions: Vec<QuestionsItemDto>,
	pub created_at: String,
	pub updated_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct TestsCreateRequestDto {
	#[validate(length(min = 1, message = "Name must not be empty"))]
	pub name: String,

	pub questions: Option<Vec<QuestionsCreateRequestDto>>,

	pub sub_tests: Option<Vec<SubTestsCreateRequestDto>>,

	pub banner: Option<String>,

	pub category: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct TestsUpdateRequestDto {
	#[validate(length(min = 1, message = "Name must not be empty"))]
	pub name: String,

	pub questions: Option<Vec<QuestionsUpdateRequestDto>>,

	pub sub_tests: Option<Vec<SubTestsUpdateRequestDto>>,

	pub banner: Option<String>,

	pub category: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct TestsItemDto {
	pub id: String,
	pub name: String,
	pub banner: Option<String>,
	pub category: String,
	pub questions: Option<Vec<QuestionsItemDto>>,
	pub sub_tests: Option<Vec<SubTestsItemDto>>,
	pub created_at: String,
	pub updated_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct TestsResponseListDto {
	pub id: String,
	pub name: String,
	pub category: String,
	pub banner: Option<String>,
	pub question_count: Option<u32>,
	pub sub_test_count: u32,
	pub created_at: String,
	pub updated_at: String,
}

impl From<TestsSchema> for TestsResponseListDto {
	fn from(value: TestsSchema) -> Self {
		let id = &value.id.id.to_raw();
		TestsResponseListDto {
			id: id.to_string(),
			name: value.name,
			banner: value.banner,
			category: value.category,
			question_count: if value.questions.is_empty() {
				Some(0)
			} else {
				Some(value.questions.len() as u32)
			},
			sub_test_count: if value.sub_tests.is_empty() {
				0
			} else {
				value.sub_tests.len() as u32
			},
			created_at: value.created_at,
			updated_at: value.updated_at,
		}
	}
}

impl TestsItemDto {
	pub fn from_with_questions(
		value: TestsSchema,
		questions: Option<Vec<QuestionsItemDto>>,
	) -> Self {
		let id = &value.id.id.to_raw();
		TestsItemDto {
			id: id.to_string(),
			name: value.name,
			banner: value.banner,
			category: value.category,
			questions,
			sub_tests: None,
			created_at: value.created_at,
			updated_at: value.updated_at,
		}
	}

	pub fn from_with_questions_and_sub_tests(
		value: TestsSchema,
		questions: Option<Vec<QuestionsItemDto>>,
		sub_tests: Option<Vec<SubTestsItemDto>>,
	) -> Self {
		let id = &value.id.id.to_raw();
		TestsItemDto {
			id: id.to_string(),
			name: value.name,
			banner: value.banner,
			category: value.category,
			questions,
			sub_tests,
			created_at: value.created_at,
			updated_at: value.updated_at,
		}
	}
}
