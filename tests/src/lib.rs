use ::surrealdb::Uuid;
pub use najm_entity::*;
pub use najm_iam::{
	AuthOtpSchema, AuthRepository, MetaRequestDto, PermissionsRepository,
	PermissionsSchema, RolesRepository, RolesRequestCreateDto, RolesRequestUpdateDto,
	UsersRepository, UsersSchema,
};
pub use najm_lib::{AppState, ResourceEnum};
pub use najm_util::{get_iso_date, make_thing};
// pub use najm_exam::{
//     SessionsRepository, SessionsCreateRequestDto, TestSessionsDto, SessionsSchema,
//     AnswersRepository, AnswersCreateAkademikRequestDto, AnswerEntryDto,
//     OptionsRepository, OptionsCreateRequestDto, OptionsUpdateRequestDto,
//     QuestionsRepository, QuestionsCreateRequestDto, QuestionsUpdateRequestDto,
//     TestsRepository, TestsCreateRequestDto,
// };

#[cfg(test)]
pub mod iam;

// #[cfg(test)]
// pub mod exam;

pub fn generate_unique_email(prefix: &str) -> String {
	format!("{}_{}@example.com", prefix, Uuid::new_v4())
}

pub async fn create_mock_app_state() -> AppState {
	use najm_lib::{surrealdb_init_mem, surrealdb_init_ws};

	let surrealdb_ws = surrealdb_init_ws()
		.await
		.expect("Failed to create WebSocket database connection");

	let surrealdb_mem = surrealdb_init_mem()
		.await
		.expect("Failed to create in-memory database connection");

	AppState {
		surrealdb_ws,
		surrealdb_mem,
	}
}

pub fn create_test_user(
	email: &str,
	fullname: &str,
	is_active: bool,
	role_id: &str,
) -> UsersSchema {
	UsersSchema {
		id: make_thing("app_users", &Uuid::new_v4().to_string()),
		email: email.to_string(),
		fullname: fullname.to_string(),
		password: "password".to_string(),
		is_deleted: false,
		avatar: None,
		phone_number: "081234567890".to_string(),
		is_active,
		gender: None,
		birthdate: None,
		referral_code: None,
		refered_by: None,
		student_type: None,
		religion: None,
		identity_number: None,
		is_profile_completed: false,
		role: make_thing("app_roles", role_id),
		created_at: get_iso_date(),
		updated_at: get_iso_date(),
		..Default::default()
	}
}

pub async fn get_role_id(state: &AppState) -> String {
	let repo = RolesRepository::new(state);
	if let Ok(existing) = repo.query_role_by_name("User".into()).await {
		return existing.id;
	}
	let _ = repo
		.query_create_role(RolesRequestCreateDto {
			name: "User".into(),
			permissions: vec![],
		})
		.await;
	repo
		.query_role_by_name("User".into())
		.await
		.expect("Role not found after creation")
		.id
}
