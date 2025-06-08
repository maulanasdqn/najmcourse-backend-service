use najm_iam::{get_iso_date, make_thing, Env, PermissionsEnum, RolesEnum};
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
