use najm_iam::PermissionsEnum;
use najm_util::{get_iso_date, Env};
use serde_json::json;
use std::error::Error;
use surrealdb::{engine::remote::ws::Ws, opt::auth::Root, Surreal};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
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
