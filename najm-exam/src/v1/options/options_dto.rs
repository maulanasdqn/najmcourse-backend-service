use super::OptionsSchema;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct OptionsCreateRequestDto {
	pub label: Option<String>,
	pub image_url: Option<String>,
	pub is_correct: bool,
	pub points: Option<f32>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct OptionsUpdateRequestDto {
	pub id: String,
	pub label: Option<String>,
	pub image_url: Option<String>,
	pub is_correct: bool,
	pub points: Option<f32>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct OptionsItemDto {
	pub id: String,
	pub label: String,
	pub image_url: Option<String>,
	pub is_correct: Option<bool>,
	pub points: Option<f32>,
	pub created_at: String,
	pub updated_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct OptionsResponseListDto {
	pub id: String,
	pub label: String,
	pub image_url: Option<String>,
	pub created_at: String,
	pub updated_at: String,
}

impl From<OptionsSchema> for OptionsResponseListDto {
	fn from(value: OptionsSchema) -> Self {
		let id = match &value.id.id {
			surrealdb::sql::Id::String(s) => s.clone(),
			_ => "".to_string(),
		};
		OptionsResponseListDto {
			id,
			label: value.label.unwrap_or("".into()),
			image_url: value.image_url,
			created_at: value.created_at,
			updated_at: value.updated_at,
		}
	}
}

impl From<OptionsSchema> for OptionsItemDto {
	fn from(o: OptionsSchema) -> Self {
		Self {
			id: match o.id.id {
				surrealdb::sql::Id::String(s) => s,
				_ => "".to_string(),
			},
			label: o.label.unwrap_or("".into()),
			is_correct: Some(o.is_correct),
			points: o.points,
			image_url: o.image_url,
			created_at: o.created_at,
			updated_at: o.updated_at,
		}
	}
}
