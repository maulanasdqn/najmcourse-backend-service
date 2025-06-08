use najm_iam::RolesEnum;
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
