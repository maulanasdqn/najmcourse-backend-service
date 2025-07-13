use crate::UsersDetailItemDto;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::{Validate, ValidationError};

fn validate_password_complexity(password: &str) -> Result<(), ValidationError> {
	let has_uppercase = password.chars().any(|c| c.is_ascii_uppercase());
	let has_lowercase = password.chars().any(|c| c.is_ascii_lowercase());
	let has_digit = password.chars().any(|c| c.is_ascii_digit());
	let has_special = password.chars().any(|c| "@$!%*?&".contains(c));
	if has_uppercase && has_lowercase && has_digit && has_special {
		Ok(())
	} else {
		Err(ValidationError::new("complexity"))
	}
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct AuthLoginRequestDto {
	#[validate(
		length(min = 1, message = "Email cannot be empty"),
		email(message = "Email not valid")
	)]
	pub email: String,
	#[validate(length(min = 1, message = "Password cannot be empty"))]
	pub password: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct AuthLoginResponsetDto {
	pub token: TokenDto,
	pub user: UsersDetailItemDto,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct TokenDto {
	pub access_token: String,
	pub refresh_token: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct AuthRegisterRequestDto {
	#[validate(length(min = 2, message = "Fullname at least have 2 character"))]
	pub fullname: String,

	#[validate(
		length(min = 1, message = "Email cannot be empty"),
		email(message = "Email not valid")
	)]
	pub email: String,

	#[validate(length(
		min = 8,
		message = "Password must have at least 8 characters"
	))]
	#[validate(custom(
		function = "validate_password_complexity",
		message = "Password must include uppercase, lowercase, number, and special character"
	))]
	pub password: String,

	#[validate(length(min = 1, message = "Student type is required"))]
	pub phone_number: String,

	#[validate(length(max = 6, message = "Referral code must be 6 characters"))]
	pub referral_code: Option<String>,

	pub refered_by: Option<String>,

	#[validate(length(min = 1, message = "Student Type is required"))]
	pub student_type: Option<String>,
}

impl AuthRegisterRequestDto {
	pub fn from(self) -> Self {
		Self { ..self }
	}
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct AuthVerifyEmailRequestDto {
	#[validate(
		length(min = 1, message = "Email cannot be empty"),
		email(message = "Email not valid")
	)]
	pub email: String,

	#[validate(length(min = 6, max = 6, message = "OTP must be 6 digits"))]
	pub otp: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct AuthResendOtpRequestDto {
	#[validate(
		length(min = 1, message = "Email cannot be empty"),
		email(message = "Email not valid")
	)]
	pub email: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct AuthRefreshTokenRequestDto {
	#[validate(length(min = 1, message = "Refresh token cannot be empty"))]
	pub refresh_token: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct AuthNewPasswordRequestDto {
	#[validate(length(min = 1, message = "Token cannot be empty"))]
	pub token: String,

	#[validate(length(
		min = 8,
		message = "Password must have at least 8 characters"
	))]
	#[validate(custom(
		function = "validate_password_complexity",
		message = "Password must include uppercase, lowercase, number, and special character"
	))]
	pub password: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct AuthSetNewPasswordRequestDto {
	#[validate(
		length(min = 1, message = "Email cannot be empty"),
		email(message = "Email not valid")
	)]
	pub email: String,

	#[validate(length(
		min = 8,
		message = "Password must have at least 8 characters"
	))]
	#[validate(custom(
		function = "validate_password_complexity",
		message = "Password must include uppercase, lowercase, number, and special character"
	))]
	pub password: String,
}
