use super::{QuestionsDetailSchema, QuestionsSchema};
use crate::{
	OptionsCreateRequestDto, OptionsItemDto, OptionsSchema, OptionsUpdateRequestDto,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct QuestionsCreateRequestDto {
	pub question: Option<String>,
	pub discussion: Option<String>,
	pub question_image_url: Option<String>,
	pub discussion_image_url: Option<String>,
	#[validate(length(min = 1, message = "At least one option is required"))]
	pub options: Vec<OptionsCreateRequestDto>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct QuestionsUpdateRequestDto {
	#[validate(length(min = 1, message = "Question ID is required"))]
	pub id: String,
	pub question: Option<String>,
	pub discussion: Option<String>,
	pub question_image_url: Option<String>,
	pub discussion_image_url: Option<String>,
	#[validate(length(min = 1, message = "At least one option is required"))]
	pub options: Vec<OptionsUpdateRequestDto>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct QuestionsItemDto {
	pub id: String,
	pub question: String,
	pub discussion: String,
	pub question_image_url: Option<String>,
	pub discussion_image_url: Option<String>,
	pub options: Vec<OptionsItemDto>,
	pub created_at: String,
	pub updated_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct QuestionsResponseListDto {
	pub id: String,
	pub question: String,
	pub discussion: String,
	pub created_at: String,
	pub updated_at: String,
}

impl From<QuestionsSchema> for QuestionsResponseListDto {
	fn from(value: QuestionsSchema) -> Self {
		let id = &value.id.id.to_raw().to_string();
		QuestionsResponseListDto {
			id: id.to_string(),
			question: value.question.unwrap_or("".into()),
			discussion: value.discussion.unwrap_or("".into()),
			created_at: value.created_at,
			updated_at: value.updated_at,
		}
	}
}

impl QuestionsItemDto {
	pub fn from_with_options(
		value: QuestionsDetailSchema,
		options: Vec<Option<OptionsSchema>>,
	) -> Self {
		let id = &value.id.id.to_raw();
		let mapped_options = options
			.into_iter()
			.flatten()
			.map(OptionsItemDto::from)
			.collect();
		Self {
			id: id.to_string(),
			question: value.question.unwrap_or("".into()),
			discussion: value.discussion.unwrap_or("".into()),
			question_image_url: value.question_image_url,
			discussion_image_url: value.discussion_image_url,
			options: mapped_options,
			created_at: value.created_at,
			updated_at: value.updated_at,
		}
	}
}
