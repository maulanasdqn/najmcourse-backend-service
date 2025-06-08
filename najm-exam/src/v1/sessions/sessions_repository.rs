use super::{
	SessionsDetailResponseDto, SessionsDetailSchema, SessionsResponseDto,
	SessionsSchema,
};
use anyhow::{Result, bail};
use najm_lib::{AppState, MetaRequestDto, ResourceEnum, ResponseListSuccessDto};
use najm_util::{DetailQueryBuilder, QueryListBuilder, get_id, get_iso_date};
use surrealdb::{Surreal, engine::remote::ws::Client};

pub struct SessionsRepository<'a> {
	state: &'a AppState,
}

pub async fn update_partial_schema(
	db: &Surreal<Client>,
	table: &str,
	id: &str,
	patch: SessionsSchema,
) -> Result<String> {
	let thing = najm_util::make_thing(table, id);
	let record_key = get_id(&thing)?;
	let result: Option<SessionsSchema> = db.update(record_key).merge(patch).await?;
	match result {
		Some(_) => Ok("Success update".into()),
		None => bail!("Failed to update"),
	}
}

impl<'a> SessionsRepository<'a> {
	pub fn new(state: &'a AppState) -> Self {
		Self { state }
	}

	pub async fn query_session_list(
		&self,
		meta: MetaRequestDto,
	) -> Result<ResponseListSuccessDto<Vec<SessionsResponseDto>>> {
		let result: ResponseListSuccessDto<Vec<SessionsSchema>> = QueryListBuilder::new(
			&self.state.surrealdb_ws,
			&ResourceEnum::Sessions.to_string(),
			&meta,
		)
		.with_condition("is_deleted = false")
		.search_field("name")
		.select_fields(vec!["*"])
		.build()
		.await?;
		let data = result
			.data
			.into_iter()
			.map(SessionsResponseDto::from)
			.collect();
		Ok(ResponseListSuccessDto {
			data,
			meta: result.meta,
		})
	}

	pub async fn query_session_by_id(
		&self,
		id: String,
	) -> Result<SessionsDetailResponseDto> {
		let db = &self.state.surrealdb_ws;
		let builder = DetailQueryBuilder::new(ResourceEnum::Sessions.to_string())
			.with_id(&id)
			.with_select_fields(vec!["*"])
			.with_fetch("tests.test")
			.with_fetch("tests.test.questions")
			.with_fetch("tests.test.questions.options")
			.with_fetch("tests.test.sub_tests")
			.with_fetch("tests.test.sub_tests.questions")
			.with_fetch("tests.test.sub_tests.questions.options");
		let sql = builder.build();
		let result: Option<SessionsDetailSchema> =
			builder.apply_bindings(db.query(sql)).await?.take(0)?;
		let Some(session) = result else {
			bail!("Session not found");
		};
		if session.is_deleted {
			bail!("Session not found");
		}
		Ok(SessionsDetailResponseDto::from(session))
	}

	pub async fn query_create_session(&self, data: SessionsSchema) -> Result<String> {
		let db = &self.state.surrealdb_ws;
		let record: Option<SessionsSchema> = db
			.create(ResourceEnum::Sessions.to_string())
			.content(data)
			.await?;
		match record {
			Some(_) => Ok("Success create session".into()),
			None => bail!("Failed to create session"),
		}
	}

	pub async fn query_update_session(&self, data: SessionsSchema) -> Result<String> {
		let db = &self.state.surrealdb_ws;
		let record_key = get_id(&data.id)?;
		let existing = self.query_raw_session_by_id(&data.id.id.to_raw()).await?;
		if existing.is_deleted {
			bail!("Session already deleted");
		}
		let merged = SessionsSchema {
			created_at: existing.created_at,
			updated_at: get_iso_date(),
			..data.clone()
		};
		let record: Option<SessionsSchema> = db.update(record_key).merge(merged).await?;
		match record {
			Some(_) => Ok("Success update session".into()),
			None => bail!("Failed to update session"),
		}
	}

	pub async fn query_delete_session(&self, id: String) -> Result<String> {
		let db = &self.state.surrealdb_ws;
		let session = self.query_raw_session_by_id(&id).await?;
		if session.is_deleted {
			bail!("Session not found");
		}
		let record_key = get_id(&session.id)?;
		let record: Option<SessionsSchema> = db
			.update(record_key)
			.merge(serde_json::json!({ "is_deleted": true }))
			.await?;
		match record {
			Some(_) => Ok("Success delete session".into()),
			None => bail!("Failed to delete session"),
		}
	}

	pub async fn query_raw_session_by_id(&self, id: &str) -> Result<SessionsSchema> {
		let db = &self.state.surrealdb_ws;
		let session: Option<SessionsSchema> =
			db.select((ResourceEnum::Sessions.to_string(), id)).await?;
		match session {
			Some(s) if !s.is_deleted => Ok(s),
			_ => bail!("Session not found"),
		}
	}
}
