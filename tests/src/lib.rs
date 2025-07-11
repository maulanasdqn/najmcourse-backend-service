use ::surrealdb::Uuid;
use axum_test::{TestResponse, TestServer};
pub use najm_entity::*;
pub use najm_iam::{
	AuthOtpSchema, AuthRepository, MetaRequestDto, PermissionsRepository,
	PermissionsSchema, RolesRepository, RolesRequestCreateDto, RolesRequestUpdateDto,
	UsersRepository, UsersSchema,
};
use najm_lib::encode_access_token;
pub use najm_lib::{AppState, ResourceEnum};
pub use najm_util::{get_iso_date, make_thing};
use serde::Serialize;

#[cfg(test)]
pub mod iam;

#[cfg(test)]
pub mod exam;

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

pub async fn seed_user_with_permissions(
	state: &AppState,
	permissions: Vec<najm_iam::v1::permissions::PermissionsEnum>,
) -> (String, String) {
	let perm_repo = PermissionsRepository::new(state);
	let role_repo = RolesRepository::new(state);
	let user_repo = UsersRepository::new(state);

	let mut permission_ids = vec![];

	for p in permissions {
		let perm_id = p.id().to_string();
		let perm = PermissionsSchema {
			id: make_thing(&ResourceEnum::Permissions.to_string(), &perm_id),
			name: p.to_string(),
			is_deleted: false,
			created_at: Some(get_iso_date()),
			updated_at: Some(get_iso_date()),
		};
		if perm_repo
			.query_permission_by_id(perm_id.clone())
			.await
			.is_err()
		{
			perm_repo.query_create_permission(perm).await.unwrap();
		}
		permission_ids.push(perm_id);
	}

	let role_name = format!("Test Role {}", Uuid::new_v4());
	let role = if let Ok(r) = role_repo.query_role_by_name(role_name.to_string()).await
	{
		r
	} else {
		let payload = RolesRequestCreateDto {
			name: role_name.to_string(),
			permissions: permission_ids,
		};
		role_repo.query_create_role(payload).await.unwrap();
		role_repo
			.query_role_by_name(role_name.to_string())
			.await
			.unwrap()
	};

	let user = create_test_user("test@example.com", "Test User", true, &role.id);
	if user_repo
		.query_user_by_email(user.email.clone())
		.await
		.is_err()
	{
		user_repo.query_create_user(user.clone()).await.unwrap();
	}

	(user.id.id.to_raw(), role.id.clone())
}

pub async fn seed_user_with_one_permission(
	state: &AppState,
	permission: najm_iam::v1::permissions::PermissionsEnum,
) -> (String, String) {
	seed_user_with_permissions(state, vec![permission]).await
}

pub async fn authorized<T: Serialize>(
	server: &TestServer,
	method: &str,
	path: &str,
	_permissions: Vec<&str>,
	body: Option<&T>,
) -> TestResponse {
	let token = encode_access_token("test@example.com".to_string()).unwrap();

	let mut request = server.method(method.parse().unwrap(), path);

	if let Some(payload) = body {
		request = request.json(payload);
	}

	request
		.add_header(
			axum::http::header::AUTHORIZATION,
			format!("Bearer {}", token),
		)
		.await
}
