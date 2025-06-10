use crate::RolesEnum;

use super::{UsersCreateRequestDto, UsersDetailQueryDto, UsersUpdateRequestDto};
use najm_lib::{ResourceEnum, hash_password};
use najm_util::{get_iso_date, make_thing};
use serde::{Deserialize, Serialize};
use surrealdb::{Uuid, sql::Thing};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UsersSchema {
	pub id: Thing,
	pub fullname: String,
	pub email: String,
	pub phone_number: String,
	pub password: String,
	pub avatar: Option<String>,
	pub referral_code: Option<String>,
	pub refered_by: Option<String>,
	pub identity_number: Option<String>,
	pub is_active: bool,
	pub is_deleted: bool,
	pub is_profile_completed: bool,
	pub student_type: Option<String>,
	pub religion: Option<String>,
	pub gender: Option<String>,
	pub birthdate: Option<String>,
	pub role: Thing,
	pub created_at: String,
	pub updated_at: String,
}

impl Default for UsersSchema {
	fn default() -> Self {
		let user_uuid = &Uuid::new_v4().to_string();
		let user_table = &ResourceEnum::Users.to_string();
		let user_thing = make_thing(user_table, user_uuid);
		let role_uuid = &RolesEnum::Admin.id();
		let role_table = &ResourceEnum::Roles.to_string();
		let role_thing = make_thing(role_table, role_uuid);
		let password = hash_password("password").unwrap();

		Self {
			id: user_thing,
			fullname: String::new(),
			email: String::new(),
			phone_number: String::new(),
			password,
			avatar: None,
			referral_code: None,
			refered_by: None,
			identity_number: None,
			is_active: true,
			is_deleted: false,
			is_profile_completed: false,
			student_type: None,
			religion: None,
			gender: None,
			birthdate: None,
			role: role_thing,
			created_at: get_iso_date(),
			updated_at: get_iso_date(),
		}
	}
}

impl UsersSchema {
	pub fn from(dto: UsersDetailQueryDto) -> Self {
		Self {
			id: dto.id,
			fullname: dto.fullname,
			email: dto.email,
			avatar: dto.avatar,
			phone_number: dto.phone_number,
			is_active: dto.is_active,
			is_deleted: dto.is_deleted,
			gender: dto.gender,
			birthdate: dto.birthdate,
			password: dto.password,
			student_type: dto.student_type,
			referral_code: dto.referral_code,
			refered_by: dto.refered_by,
			role: dto.role.id,
			created_at: dto.created_at,
			updated_at: dto.updated_at,
			..Default::default()
		}
	}

	pub fn update(user: UsersUpdateRequestDto, id: String) -> Self {
		Self {
			id: make_thing(&ResourceEnum::Users.to_string(), &id),
			fullname: user.fullname,
			email: user.email,
			phone_number: user.phone_number,
			is_active: user.is_active,
			gender: user.gender,
			birthdate: user.birthdate,
			avatar: user.avatar,
			is_deleted: false,
			role: make_thing(&ResourceEnum::Roles.to_string(), &user.role_id),
			updated_at: get_iso_date(),
			..Default::default()
		}
	}

	pub fn create(user: UsersCreateRequestDto) -> Self {
		let password = hash_password(&user.password).unwrap();
		Self {
			id: make_thing(
				&ResourceEnum::Users.to_string(),
				&Uuid::new_v4().to_string(),
			),
			fullname: user.fullname,
			email: user.email,
			password,
			phone_number: user.phone_number,
			gender: None,
			birthdate: None,
			avatar: None,
			referral_code: None,
			refered_by: None,
			student_type: user.student_type,
			role: make_thing(&ResourceEnum::Roles.to_string(), &user.role_id),
			created_at: get_iso_date(),
			updated_at: get_iso_date(),
			..Default::default()
		}
	}

	pub fn patch_password(dto: UsersDetailQueryDto, password: String) -> Self {
		Self {
			password,
			id: dto.id.clone(),
			..Self::from(dto)
		}
	}
}
