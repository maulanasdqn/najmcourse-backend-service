use super::{OptionsCreateRequestDto, OptionsUpdateRequestDto};
use najm_lib::ResourceEnum;
use najm_util::{get_iso_date, make_thing};
use serde::{Deserialize, Serialize};
use surrealdb::{Uuid, sql::Thing};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OptionsSchema {
	pub id: Thing,
	pub label: Option<String>,
	pub points: Option<f32>,
	pub image_url: Option<String>,
	pub is_correct: bool,
	pub is_deleted: bool,
	pub created_at: String,
	pub updated_at: String,
}

impl Default for OptionsSchema {
	fn default() -> Self {
		Self {
			id: make_thing(
				&ResourceEnum::Options.to_string(),
				&Uuid::new_v4().to_string(),
			),
			label: None,
			points: None,
			image_url: None,
			is_correct: false,
			is_deleted: false,
			created_at: get_iso_date(),
			updated_at: get_iso_date(),
		}
	}
}

impl OptionsSchema {
	pub fn create(option: OptionsCreateRequestDto) -> Self {
		Self {
			id: make_thing(
				&ResourceEnum::Options.to_string(),
				&Uuid::new_v4().to_string(),
			),
			label: option.label,
			points: option.points,
			image_url: option.image_url,
			is_correct: option.is_correct,
			is_deleted: false,
			created_at: get_iso_date(),
			updated_at: get_iso_date(),
		}
	}

	pub fn update(option: OptionsUpdateRequestDto, id: String) -> Self {
		Self {
			id: make_thing(&ResourceEnum::Options.to_string(), &id),
			label: option.label,
			points: option.points,
			image_url: option.image_url,
			is_correct: option.is_correct,
			is_deleted: false,
			updated_at: get_iso_date(),
			..Default::default()
		}
	}
}
