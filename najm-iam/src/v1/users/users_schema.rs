use super::{UsersCreateRequestDto, UsersDetailQueryDto, UsersUpdateRequestDto};
use crate::{AuthRegisterRequestDto, RolesEnum};
use najm_lib::{ResourceEnum, hash_password};
use najm_util::{get_iso_date, make_thing};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

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
		let password = hash_password("password").unwrap();

		Self {
			id: ResourceEnum::Users.thing(),
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
			role: RolesEnum::Admin.thing(),
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
		let user_table = &ResourceEnum::Users.to_string();
		let user_thing = make_thing(user_table, &id);
		let role_table = &ResourceEnum::Roles.to_string();
		let role_thing = make_thing(role_table, &user.role_id);

		Self {
			id: user_thing,
			fullname: user.fullname,
			email: user.email,
			phone_number: user.phone_number,
			is_active: user.is_active,
			gender: user.gender,
			birthdate: user.birthdate,
			avatar: user.avatar,
			is_deleted: false,
			role: role_thing,
			updated_at: get_iso_date(),
			..Default::default()
		}
	}

	pub fn register(user: AuthRegisterRequestDto) -> Self {
		let password = hash_password(&user.password).unwrap();
		Self {
			fullname: user.fullname,
			email: user.email,
			phone_number: user.phone_number,
			password,
			referral_code: user.referral_code,
			refered_by: user.refered_by,
			student_type: user.student_type,
			created_at: get_iso_date(),
			updated_at: get_iso_date(),
			..Default::default()
		}
	}

	pub fn create(user: UsersCreateRequestDto) -> Self {
		let role_table = &ResourceEnum::Roles.to_string();
		let role_thing = make_thing(role_table, &user.role_id);
		let password = hash_password(&user.password).unwrap();
		Self {
			fullname: user.fullname,
			email: user.email,
			phone_number: user.phone_number,
			password,
			student_type: user.student_type,
			role: role_thing,
			is_active: user.is_active,
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
