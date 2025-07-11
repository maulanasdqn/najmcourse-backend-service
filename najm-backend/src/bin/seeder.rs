use najm_cms::v1::landing::events::events_schema::EventsSchema;
use najm_iam::{PermissionsEnum, RolesEnum};
use najm_util::{get_iso_date, hash_password, make_thing, Env};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::error::Error;
use surrealdb::{
	engine::remote::ws::{Client, Ws},
	opt::auth::Root,
	sql::Thing,
	Surreal,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UsersSchema {
	pub id: Thing,
	pub fullname: String,
	pub email: String,
	pub password: String,
	pub avatar: Option<String>,
	pub phone_number: String,
	pub referral_code: Option<String>,
	pub referred_by: Option<String>,
	pub identity_number: Option<String>,
	pub is_active: bool,
	pub is_deleted: bool,
	pub student_type: Option<String>,
	pub religion: Option<String>,
	pub gender: Option<String>,
	pub birthdate: Option<String>,
	pub is_profile_completed: bool,
	pub role: Thing,
	pub created_at: String,
	pub updated_at: String,
}

async fn seed_permissions(db: &Surreal<Client>) -> Result<(), Box<dyn Error>> {
	println!("🔧 Seeding: permissions");
	for permission in [
		PermissionsEnum::ReadDashboard,
		PermissionsEnum::ReadListUsers,
		PermissionsEnum::ReadDetailUsers,
		PermissionsEnum::CreateUsers,
		PermissionsEnum::DeleteUsers,
		PermissionsEnum::UpdateUsers,
		PermissionsEnum::ActivateUsers,
		PermissionsEnum::ReadListRoles,
		PermissionsEnum::ReadDetailRoles,
		PermissionsEnum::CreateRoles,
		PermissionsEnum::DeleteRoles,
		PermissionsEnum::UpdateRoles,
		PermissionsEnum::ReadListAnswers,
		PermissionsEnum::ReadDetailAnswers,
		PermissionsEnum::CreateAnswers,
		PermissionsEnum::UpdateAnswers,
		PermissionsEnum::DeleteAnswers,
		PermissionsEnum::ReadListPermissions,
		PermissionsEnum::ReadDetailPermissions,
		PermissionsEnum::CreatePermissions,
		PermissionsEnum::DeletePermissions,
		PermissionsEnum::UpdatePermissions,
		PermissionsEnum::ReadListSessions,
		PermissionsEnum::ReadDetailSessions,
		PermissionsEnum::CreateSessions,
		PermissionsEnum::UpdateSessions,
		PermissionsEnum::DeleteSessions,
		PermissionsEnum::ReadListTests,
		PermissionsEnum::ReadDetailTests,
		PermissionsEnum::CreateTests,
		PermissionsEnum::UpdateTests,
		PermissionsEnum::DeleteTests,
		PermissionsEnum::ReadListOptions,
		PermissionsEnum::ReadDetailOptions,
		PermissionsEnum::CreateOptions,
		PermissionsEnum::UpdateOptions,
		PermissionsEnum::DeleteOptions,
		PermissionsEnum::ReadListQuestions,
		PermissionsEnum::ReadDetailQuestions,
		PermissionsEnum::CreateQuestions,
		PermissionsEnum::UpdateQuestions,
		PermissionsEnum::DeleteQuestions,
	] {
		db.query("CREATE type::thing('app_permissions', $id) CONTENT $data")
			.bind(("id", permission.id()))
			.bind((
				"data",
				json!({
					"name": permission.to_string(),
					"is_deleted": false,
					"created_at": get_iso_date(),
					"updated_at": get_iso_date()
				}),
			))
			.await?;
		println!("✅ Inserted: {}", permission.to_string());
	}

	println!("✅ All Permissions seeded");
	Ok(())
}

async fn seed_roles(db: &Surreal<Client>) -> Result<(), Box<dyn Error>> {
	println!("🔧 Seeding: roles");
	let roles = [RolesEnum::Admin, RolesEnum::Student, RolesEnum::Staff];

	for role in roles {
		db.query("CREATE type::thing('app_roles', $id) CONTENT $data")
			.bind(("id", role.id()))
			.bind((
				"data",
				json!({
						"name": role.to_string(),
						"permissions": [],
						"is_deleted": false,
						"created_at": get_iso_date(),
						"updated_at": get_iso_date(),
				}),
			))
			.await?;
		println!("✅ Inserted role: {}", role);
	}
	println!("✅ All roles successfully seeded");
	Ok(())
}

async fn seed_roles_permissions(db: &Surreal<Client>) -> Result<(), Box<dyn Error>> {
	println!("🔧 Seeding: roles_permissions");
	let permission_refs_admin: Vec<_> = [
		PermissionsEnum::ReadDashboard,
		PermissionsEnum::ReadListUsers,
		PermissionsEnum::ReadDetailUsers,
		PermissionsEnum::CreateUsers,
		PermissionsEnum::DeleteUsers,
		PermissionsEnum::UpdateUsers,
		PermissionsEnum::ActivateUsers,
		PermissionsEnum::ReadListRoles,
		PermissionsEnum::ReadDetailRoles,
		PermissionsEnum::CreateRoles,
		PermissionsEnum::DeleteRoles,
		PermissionsEnum::UpdateRoles,
		PermissionsEnum::ReadListPermissions,
		PermissionsEnum::ReadDetailPermissions,
		PermissionsEnum::CreatePermissions,
		PermissionsEnum::DeletePermissions,
		PermissionsEnum::UpdatePermissions,
		PermissionsEnum::ReadListSessions,
		PermissionsEnum::ReadDetailSessions,
		PermissionsEnum::CreateSessions,
		PermissionsEnum::UpdateSessions,
		PermissionsEnum::DeleteSessions,
		PermissionsEnum::ReadListTests,
		PermissionsEnum::ReadDetailTests,
		PermissionsEnum::CreateTests,
		PermissionsEnum::UpdateTests,
		PermissionsEnum::DeleteTests,
		PermissionsEnum::ReadListAnswers,
		PermissionsEnum::ReadDetailAnswers,
		PermissionsEnum::CreateAnswers,
		PermissionsEnum::UpdateAnswers,
		PermissionsEnum::DeleteAnswers,
		PermissionsEnum::ReadListOptions,
		PermissionsEnum::ReadDetailOptions,
		PermissionsEnum::CreateOptions,
		PermissionsEnum::UpdateOptions,
		PermissionsEnum::DeleteOptions,
		PermissionsEnum::ReadListQuestions,
		PermissionsEnum::ReadDetailQuestions,
		PermissionsEnum::CreateQuestions,
		PermissionsEnum::UpdateQuestions,
		PermissionsEnum::DeleteQuestions,
	]
	.iter()
	.map(|perm| make_thing("app_permissions", perm.id()))
	.collect();

	let permission_refs_student: Vec<_> = [
		PermissionsEnum::ReadDashboard,
		PermissionsEnum::ReadListSessions,
		PermissionsEnum::ReadDetailSessions,
		PermissionsEnum::ReadDetailTests,
		PermissionsEnum::ReadDetailAnswers,
		PermissionsEnum::CreateAnswers,
		PermissionsEnum::ReadDetailUsers,
	]
	.iter()
	.map(|perm| make_thing("app_permissions", perm.id()))
	.collect();

	db.query("UPDATE type::thing('app_roles', $role_id) SET permissions = $permissions, updated_at = $updated_at WHERE is_deleted = false")
		.bind(("role_id", RolesEnum::Admin.id()))
		.bind(("permissions", permission_refs_admin))
		.bind(("updated_at", get_iso_date()))
		.await?;
	println!(
		"✅ All permissions successfully added to {} role",
		RolesEnum::Admin
	);

	db.query("UPDATE type::thing('app_roles', $role_id) SET permissions = $permissions, updated_at = $updated_at WHERE is_deleted = false")
		.bind(("role_id", RolesEnum::Student.id()))
		.bind(("permissions", permission_refs_student))
		.bind(("updated_at", get_iso_date()))
		.await?;
	println!(
		"✅ Limited permissions successfully added to {} role",
		RolesEnum::Student
	);
	Ok(())
}

async fn seed_users(db: &Surreal<Client>) -> Result<(), Box<dyn Error>> {
	println!("🔧 Seeding: users");
	let users = vec![
		(
			"c3b1d6a8-8d4f-4b36-b789-2e532ec7a7b2",
			"admin@example.com",
			"Admin",
			None,
			RolesEnum::Admin.id(),
		),
		(
			"a4d23fb5-9e31-423c-9842-fbd6e75a5298",
			"staff@example.com",
			"Staff",
			None,
			RolesEnum::Staff.id(),
		),
		(
			"d5e89c12-72af-4b1a-abc3-ff1234567890",
			"student@example.com",
			"Student",
			Some("TNI".into()),
			RolesEnum::Student.id(),
		),
	];
	for (id, email, fullname, student_type, role_id) in users {
		let user = UsersSchema {
			id: Thing::from(("app_users", id)),
			fullname: fullname.into(),
			email: email.into(),
			password: hash_password("password").unwrap(),
			avatar: None,
			phone_number: "081234567890".into(),
			referral_code: None,
			referred_by: None,
			identity_number: None,
			is_active: true,
			is_deleted: false,
			student_type,
			religion: None,
			gender: None,
			birthdate: None,
			is_profile_completed: false,
			role: Thing::from(("app_roles", role_id)),
			created_at: get_iso_date(),
			updated_at: get_iso_date(),
		};
		db.create::<Option<UsersSchema>>(("app_users", id))
			.content(user)
			.await?;
		println!("✅ Inserted user: {} ({})", fullname, email);
	}
	println!("✅ All users successfully seeded!");
	Ok(())
}

async fn seed_events(db: &Surreal<Client>) -> Result<(), Box<dyn Error>> {
	println!("🔧 Seeding: events");
	let events = vec![
		(
			"e1a2b3c4-5d6e-7f8g-9h0i-1j2k3l4m5n6o",
			"Tech Conference 2025",
			"Annual technology conference featuring the latest innovations in software development, AI, and cloud computing.",
			"https://techconf2025.example.com",
			150.0,
			Some("Jakarta Convention Center".to_string()),
			false,
			"2025-06-15T09:00:00Z",
			"2025-06-17T18:00:00Z",
		),
		(
			"f2b3c4d5-6e7f-8g9h-0i1j-2k3l4m5n6o7p",
			"Online Web Development Workshop",
			"Comprehensive workshop covering modern web development frameworks including React, Vue, and Angular.",
			"https://webdev-workshop.example.com",
			75.0,
			None,
			true,
			"2025-07-10T14:00:00Z",
			"2025-07-10T17:00:00Z",
		),
		(
			"g3c4d5e6-7f8g-9h0i-1j2k-3l4m5n6o7p8q",
			"Startup Pitch Competition",
			"Exciting competition where emerging startups present their innovative ideas to a panel of expert judges and investors.",
			"https://startup-pitch.example.com",
			25.0,
			Some("Innovation Hub Surabaya".to_string()),
			false,
			"2025-08-05T10:00:00Z",
			"2025-08-05T16:00:00Z",
		),
		(
			"h4d5e6f7-8g9h-0i1j-2k3l-4m5n6o7p8q9r",
			"Digital Marketing Masterclass",
			"Learn advanced digital marketing strategies, social media optimization, and data-driven marketing techniques.",
			"https://digital-marketing.example.com",
			100.0,
			None,
			true,
			"2025-09-20T13:00:00Z",
			"2025-09-22T15:00:00Z",
		),
	];

	for (
		id,
		name,
		description,
		detail_link,
		price,
		location,
		is_online,
		start_date,
		end_date,
	) in events
	{
		let event = EventsSchema {
			id: Thing::from(("app_events", id)),
			name: name.into(),
			description: description.into(),
			detail_link: detail_link.into(),
			price,
			location,
			is_online,
			is_deleted: false,
			start_date: start_date.into(),
			end_date: end_date.into(),
			created_at: get_iso_date(),
			updated_at: get_iso_date(),
		};

		db.create::<Option<EventsSchema>>(("app_events", id))
			.content(event)
			.await?;

		println!(
			"✅ Inserted event: {} ({})",
			name,
			if is_online { "Online" } else { "In-person" }
		);
	}

	println!("✅ All Events seeded");
	Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	println!("🚀 Running all seeders...\n");

	let env = Env::new();
	let db = Surreal::new::<Ws>(env.surrealdb_url).await?;
	db.signin(Root {
		username: &env.surrealdb_username,
		password: &env.surrealdb_password,
	})
	.await?;
	db.use_ns(env.surrealdb_namespace)
		.use_db(env.surrealdb_dbname)
		.await?;

	seed_permissions(&db).await?;
	seed_roles(&db).await?;
	seed_roles_permissions(&db).await?;
	seed_users(&db).await?;
	seed_events(&db).await?;

	println!("\n✅ All seeding completed successfully.");
	Ok(())
}
