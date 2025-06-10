use super::{
	RolesDetailItemDto, RolesDetailQueryDto, RolesListItemDto, RolesRequestCreateDto,
	RolesRequestUpdateDto,
};
use crate::{PermissionsEnum, ResourceEnum, make_thing};
use najm_util::get_iso_date;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use surrealdb::sql::Thing;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RolesSchema {
	pub id: Thing,
	pub name: String,
	pub is_deleted: bool,
	pub permissions: Vec<Thing>,
	pub created_at: Option<String>,
	pub updated_at: Option<String>,
}

impl Default for RolesSchema {
	fn default() -> Self {
		RolesSchema {
			id: ResourceEnum::Roles.thing(),
			permissions: vec![PermissionsEnum::ReadDashboard.thing()],
			name: String::new(),
			is_deleted: false,
			created_at: Some(get_iso_date()),
			updated_at: Some(get_iso_date()),
		}
	}
}

impl RolesSchema {
	pub fn from(dto: RolesDetailQueryDto) -> Self {
		Self {
			id: dto.id,
			name: dto.name,
			permissions: dto
				.permissions
				.into_iter()
				.map(|perm| {
					make_thing(&ResourceEnum::Permissions.to_string(), &perm.id.id.to_raw())
				})
				.collect(),
			is_deleted: dto.is_deleted,
			created_at: dto.created_at,
			updated_at: dto.updated_at,
		}
	}

	pub fn create(dto: RolesRequestCreateDto) -> Self {
		let permissions: Vec<Thing> = dto
			.permissions
			.into_iter()
			.map(|id| make_thing(&ResourceEnum::Permissions.to_string(), &id))
			.collect();
		Self {
			name: dto.name,
			permissions,
			..Default::default()
		}
	}

	pub fn update(
		dto: RolesRequestUpdateDto,
		id: String,
		existing: RolesDetailItemDto,
	) -> Self {
		let role_table = &ResourceEnum::Roles.to_string();
		let role_thing = make_thing(role_table, &id);
		let name = dto.name.unwrap_or(existing.name);
		let permissions: Vec<Thing> =
			match (dto.permissions, dto.overwrite.unwrap_or(false)) {
				(Some(new_ids), true) => new_ids
					.iter()
					.map(|id| make_thing(&ResourceEnum::Permissions.to_string(), id))
					.collect(),
				(Some(new_ids), false) => {
					let mut all_ids: HashSet<String> =
						existing.permissions.iter().map(|p| p.id.clone()).collect();
					for id in new_ids {
						all_ids.insert(id);
					}
					all_ids
						.into_iter()
						.map(|id| make_thing(&ResourceEnum::Permissions.to_string(), &id))
						.collect()
				}
				(None, _) => existing
					.permissions
					.iter()
					.map(|p| make_thing(&ResourceEnum::Permissions.to_string(), &p.id))
					.collect(),
			};
		Self {
			id: role_thing,
			name,
			permissions,
			is_deleted: existing.is_deleted,
			created_at: existing.created_at,
			..Default::default()
		}
	}

	pub fn list(self) -> RolesListItemDto {
		RolesListItemDto {
			id: self.id.id.to_raw(),
			name: self.name.clone(),
			permissions_count: self.permissions.len(),
			created_at: self.created_at.clone(),
			updated_at: self.updated_at.clone(),
		}
	}
}
