use najm_gateway::gateway_service;
use najm_lib::axum_init;

#[tokio::main]
async fn main() {
	axum_init(|surrealdb_ws, surrealdb_mem| async {
		gateway_service(surrealdb_ws, surrealdb_mem).await
	})
	.await;
}
