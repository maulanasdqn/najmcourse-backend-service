use crate::{RolesDetailItemDto, RolesDetailQueryDto};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct UsersActiveInactiveRequestDto {
	pub is_active: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct UsersCompletePaymentRequestDto {
	pub is_payment_completed: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct UsersSetNewPasswordRequestDto {
	#[validate(length(
		min = 8,
		message = "Password must have at least 8 characters"
	))]
	pub password: String,

	#[validate(length(
		min = 8,
		message = "Old Password must have at least 8 characters"
	))]
	pub old_password: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct UsersCreateRequestDto {
	#[validate(length(min = 2, message = "Fullname at least have 2 character"))]
	pub fullname: String,

	#[validate(
		length(min = 1, message = "Email cannot be empty"),
		email(message = "Email not valid")
	)]
	pub email: String,

	#[validate(length(
		min = 10,
		message = "Phone number at least have 10 character"
	))]
	pub phone_number: String,

	#[validate(length(
		min = 8,
		message = "Password must have at least 8 characters"
	))]
	pub password: String,

	#[validate(length(max = 6, message = "Referral code must be 6 characters"))]
	pub referral_code: Option<String>,

	pub refered_by: Option<String>,

	pub is_active: bool,

	#[validate(length(min = 1, message = "Student Type is required"))]
	pub student_type: Option<String>,

	pub role_id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct UsersUpdateRequestDto {
	#[validate(length(min = 2, message = "Fullname at least have 2 character"))]
	pub fullname: String,

	#[validate(
		length(min = 1, message = "Email cannot be empty"),
		email(message = "Email not valid")
	)]
	pub email: String,

	#[validate(length(
		min = 10,
		message = "Phone number at least have 10 character"
	))]
	pub phone_number: String,

	#[validate(length(
		min = 8,
		message = "Password must have at least 8 characters"
	))]
	pub password: String,

	pub avatar: Option<String>,

	#[validate(length(max = 6, message = "Referral code must be 6 characters"))]
	pub referral_code: Option<String>,

	pub refered_by: Option<String>,

	pub identity_number: Option<String>,

	pub is_active: bool,

	#[validate(length(min = 1, message = "Student Type is required"))]
	pub student_type: Option<String>,

	pub religion: Option<String>,

	pub gender: Option<String>,

	pub birthdate: Option<String>,

	pub role_id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct UsersDetailItemDto {
	pub id: String,
	pub fullname: String,
	pub email: String,
	pub phone_number: String,
	pub avatar: Option<String>,
	pub referral_code: Option<String>,
	pub refered_by: Option<String>,
	pub identity_number: Option<String>,
	pub is_active: bool,
	pub is_profile_completed: bool,
	pub is_payment_completed: bool,
	pub student_type: Option<String>,
	pub religion: Option<String>,
	pub gender: Option<String>,
	pub birthdate: Option<String>,
	pub role: RolesDetailItemDto,
	pub created_at: String,
	pub updated_at: String,
}

impl UsersDetailItemDto {
	pub fn from(dto: &UsersDetailQueryDto) -> Self {
		Self {
			id: dto.id.id.to_raw().clone(),
			fullname: dto.fullname.clone(),
			email: dto.email.clone(),
			phone_number: dto.phone_number.clone(),
			avatar: dto.avatar.clone(),
			referral_code: dto.referral_code.clone(),
			refered_by: dto.refered_by.clone(),
			identity_number: dto.identity_number.clone(),
			is_active: dto.is_active.clone(),
			is_profile_completed: dto.is_profile_completed.clone(),
			is_payment_completed: dto.is_payment_completed.clone(),
			student_type: dto.student_type.clone(),
			religion: dto.religion.clone(),
			gender: dto.gender.clone(),
			birthdate: dto.birthdate.clone(),
			role: RolesDetailItemDto::from(&dto.role),
			created_at: dto.created_at.clone(),
			updated_at: dto.updated_at.clone(),
		}
	}
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct UsersListItemDto {
	pub id: String,
	pub email: String,
	pub fullname: String,
	pub phone_number: String,
	pub avatar: Option<String>,
	pub student_type: Option<String>,
	pub role: String,
	pub is_active: bool,
	pub is_payment_completed: bool,
	pub created_at: String,
	pub updated_at: String,
}

impl UsersListItemDto {
	pub fn from(dto: &UsersListQueryDto) -> Self {
		Self {
			id: dto.id.id.to_raw().clone(),
			fullname: dto.fullname.clone(),
			email: dto.email.clone(),
			phone_number: dto.phone_number.clone(),
			avatar: dto.avatar.clone(),
			student_type: dto.student_type.clone(),
			role: dto.role.name.clone(),
			is_active: dto.is_active.clone(),
			is_payment_completed: dto.is_payment_completed.clone(),
			created_at: dto.created_at.clone(),
			updated_at: dto.updated_at.clone(),
		}
	}
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UsersListQueryDto {
	pub id: Thing,
	pub fullname: String,
	pub email: String,
	pub phone_number: String,
	pub avatar: Option<String>,
	pub is_active: bool,
	pub is_payment_completed: bool,
	pub student_type: Option<String>,
	pub role: RolesDetailQueryDto,
	pub created_at: String,
	pub updated_at: String,
}

impl UsersListQueryDto {
	pub fn from(self) -> UsersListItemDto {
		UsersListItemDto {
			id: self.id.id.to_raw(),
			fullname: self.fullname.clone(),
			email: self.email.clone(),
			phone_number: self.phone_number.clone(),
			avatar: self.avatar.clone(),
			is_active: self.is_active,
			is_payment_completed: self.is_payment_completed,
			student_type: self.student_type.clone(),
			role: self.role.name.clone(),
			created_at: self.created_at.clone(),
			updated_at: self.updated_at.clone(),
		}
	}
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UsersDetailQueryDto {
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
	pub is_payment_completed: bool,
	pub student_type: Option<String>,
	pub religion: Option<String>,
	pub gender: Option<String>,
	pub birthdate: Option<String>,
	pub role: RolesDetailQueryDto,
	pub created_at: String,
	pub updated_at: String,
}

impl UsersDetailQueryDto {
	pub fn from(&self) -> Self {
		Self {
			role: RolesDetailQueryDto::from(self.role.clone()),
			..self.clone()
		}
	}
}
