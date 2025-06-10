use crate::{PermissionsRequestDto, ResourceEnum};
use najm_util::make_thing;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use super::PermissionsItemDto;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PermissionsSchema {
	pub id: Thing,
	pub name: String,
	pub is_deleted: bool,
	pub created_at: Option<String>,
	pub updated_at: Option<String>,
}

impl Default for PermissionsSchema {
	fn default() -> Self {
		PermissionsSchema {
			id: ResourceEnum::Permissions.thing(),
			name: String::new(),
			is_deleted: false,
			created_at: None,
			updated_at: None,
		}
	}
}

impl PermissionsSchema {
	pub fn list(self) -> PermissionsItemDto {
		PermissionsItemDto {
			id: self.id.id.to_raw(),
			name: self.name.clone(),
			created_at: self.created_at.clone(),
			updated_at: self.updated_at.clone(),
		}
	}

	pub fn create(dto: PermissionsRequestDto) -> Self {
		Self {
			name: dto.name,
			..Default::default()
		}
	}

	pub fn update(dto: PermissionsRequestDto, id: String) -> Self {
		Self {
			name: dto.name,
			id: make_thing(&ResourceEnum::Permissions.to_string(), &id),
			..Default::default()
		}
	}
}
