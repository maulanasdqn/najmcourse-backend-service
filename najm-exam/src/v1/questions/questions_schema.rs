use super::{QuestionsCreateRequestDto, QuestionsUpdateRequestDto};
use crate::OptionsSchema;
use najm_lib::ResourceEnum;
use najm_util::{get_iso_date, make_thing};
use serde::{Deserialize, Serialize};
use surrealdb::{Uuid, sql::Thing};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QuestionsSchema {
	pub id: Thing,
	pub question: Option<String>,
	pub discussion: Option<String>,
	pub question_image_url: Option<String>,
	pub discussion_image_url: Option<String>,
	pub options: Vec<Thing>,
	pub is_deleted: bool,
	pub created_at: String,
	pub updated_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QuestionsDetailSchema {
	pub id: Thing,
	pub question: Option<String>,
	pub discussion: Option<String>,
	pub question_image_url: Option<String>,
	pub discussion_image_url: Option<String>,
	pub options: Vec<Option<OptionsSchema>>,
	pub is_deleted: bool,
	pub created_at: String,
	pub updated_at: String,
}

impl Default for QuestionsSchema {
	fn default() -> Self {
		Self {
			id: make_thing(
				&ResourceEnum::Questions.to_string(),
				&Uuid::new_v4().to_string(),
			),
			question: None,
			discussion: None,
			question_image_url: None,
			discussion_image_url: None,
			options: Vec::new(),
			is_deleted: false,
			created_at: get_iso_date(),
			updated_at: get_iso_date(),
		}
	}
}

impl QuestionsSchema {
	pub fn create(
		question: QuestionsCreateRequestDto,
		options_ids: Vec<Thing>,
	) -> Self {
		Self {
			id: make_thing(
				&ResourceEnum::Questions.to_string(),
				&Uuid::new_v4().to_string(),
			),
			question: question.question,
			discussion: question.discussion,
			question_image_url: question.question_image_url,
			discussion_image_url: question.discussion_image_url,
			options: options_ids,
			is_deleted: false,
			created_at: get_iso_date(),
			updated_at: get_iso_date(),
		}
	}

	pub fn update(question: QuestionsUpdateRequestDto, id: String) -> Self {
		Self {
			id: make_thing(&ResourceEnum::Questions.to_string(), &id),
			question: question.question,
			discussion: question.discussion,
			question_image_url: question.question_image_url,
			discussion_image_url: question.discussion_image_url,
			options: Vec::new(), // Options will be handled separately
			is_deleted: false,
			updated_at: get_iso_date(),
			..Default::default()
		}
	}
}
