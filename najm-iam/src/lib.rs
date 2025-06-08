use ::surrealdb::Uuid;
use najm_entity::*;
use najm_lib::*;
use najm_util::*;

pub mod v1;

pub use najm_entity::*;
pub use najm_lib::*;
pub use najm_util::*;
pub use v1::*;

pub fn create_test_user(
	email: &str,
	fullname: &str,
	is_active: bool,
	role_id: &str,
) -> UsersSchema {
	UsersSchema {
		id: make_thing("app_users", &Uuid::new_v4().to_string()),
		email: email.to_string(),
		fullname: format!("Randomize {} {}", fullname, rand::random::<u32>()),
		password: hash_password("secret").unwrap(),
		is_deleted: false,
		avatar: None,
		phone_number: "081234567890".to_string(),
		is_active,
		gender: None,
		birthdate: None,
		referral_code: None,
		refered_by: None,
		student_type: None,
		role: make_thing("app_roles", role_id),
		created_at: get_iso_date(),
		updated_at: get_iso_date(),
		..Default::default()
	}
}
