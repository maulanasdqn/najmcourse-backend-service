use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Clone, Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct AnswersCreateAkademikRequestDto {
	#[validate(length(min = 1))]
	#[schema(example = "uuid")]
	pub user_id: String,

	#[validate(length(min = 1))]
	#[schema(example = "uuid")]
	pub test_id: String,

	#[validate(length(min = 1))]
	#[schema(example = "uuid")]
	pub session_id: String,

	#[validate(length(min = 1))]
	pub answers: Vec<AnswerEntryDto>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct AnswersCreatePsikologiRequestDto {
	#[validate(length(min = 1))]
	#[schema(example = "uuid")]
	pub user_id: String,

	#[validate(length(min = 1))]
	#[schema(example = "uuid")]
	pub test_id: String,

	#[validate(length(min = 1))]
	#[schema(example = "uuid")]
	pub sub_test_id: String,

	#[validate(length(min = 1))]
	#[schema(example = "uuid")]
	pub session_id: String,

	#[validate(length(min = 1))]
	pub answers: Vec<AnswerEntryDto>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct AnswerEntryDto {
	#[validate(length(min = 1))]
	#[schema(example = "uuid")]
	pub question_id: String,

	#[validate(length(min = 1))]
	#[schema(example = "uuid")]
	pub option_id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct OptionsItemAnswersDto {
	pub id: String,
	pub label: String,
	pub is_correct: bool,
	pub points: Option<f32>,
	pub is_user_selected: bool,
	pub image_url: Option<String>,
	pub created_at: String,
	pub updated_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct QuestionsItemAnswersDto {
	pub id: String,
	pub question: String,
	pub discussion: String,
	pub question_image_url: Option<String>,
	pub discussion_image_url: Option<String>,
	pub options: Vec<OptionsItemAnswersDto>,
	pub created_at: String,
	pub updated_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct TestsItemAnswersDto {
	pub id: String,
	pub name: String,
	pub score: i32,
	pub passing_grade: Option<f32>,
	pub questions: Vec<QuestionsItemAnswersDto>,
	pub created_at: String,
	pub updated_at: String,
}
