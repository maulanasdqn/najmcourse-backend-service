use super::{OptionsItemDto, OptionsSchema};
use anyhow::{Result, bail};
use najm_lib::{AppState, MetaRequestDto, ResourceEnum, ResponseListSuccessDto};
use najm_util::{
	DetailQueryBuilder, QueryListBuilder, extract_id, get_id, get_iso_date,
};

pub struct OptionsRepository<'a> {
	state: &'a AppState,
}

impl<'a> OptionsRepository<'a> {
	pub fn new(state: &'a AppState) -> Self {
		Self { state }
	}

	pub async fn query_raw_option_by_id(&self, id: &str) -> Result<OptionsSchema> {
		let db = &self.state.surrealdb_ws;
		let role: Option<OptionsSchema> =
			db.select((ResourceEnum::Options.to_string(), id)).await?;
		match role {
			Some(r) if !r.is_deleted => Ok(r),
			_ => bail!("Options not found"),
		}
	}

	pub async fn query_option_list(
		&self,
		meta: MetaRequestDto,
	) -> Result<ResponseListSuccessDto<Vec<OptionsItemDto>>> {
		let result: ResponseListSuccessDto<Vec<OptionsSchema>> = QueryListBuilder::new(
			&self.state.surrealdb_ws,
			&ResourceEnum::Options.to_string(),
			&meta,
		)
		.with_condition("is_deleted = false")
		.search_field("label")
		.select_fields(vec!["*"])
		.build()
		.await?;
		let data = result
			.data
			.into_iter()
			.map(|option| OptionsItemDto {
				id: extract_id(&option.id),
				label: option.label.unwrap_or("".into()),
				image_url: option.image_url,
				is_correct: None,
				points: option.points,
				created_at: option.created_at,
				updated_at: option.updated_at,
			})
			.collect();
		Ok(ResponseListSuccessDto {
			data,
			meta: result.meta,
		})
	}

	pub async fn query_option_by_label(
		&self,
		label: String,
	) -> Result<OptionsItemDto> {
		let db = &self.state.surrealdb_ws;
		let builder = DetailQueryBuilder::new(ResourceEnum::Options.to_string())
			.with_where("label")
			.where_value(label.clone())
			.with_select_fields(vec!["*"]);
		let sql = builder.build();
		let option_opt: Option<OptionsSchema> =
			builder.apply_bindings(db.query(sql)).await?.take(0)?;
		let Some(option) = option_opt else {
			bail!("Option not found");
		};
		if option.is_deleted {
			bail!("Option not found");
		}
		Ok(OptionsItemDto {
			id: extract_id(&option.id),
			label: option.label.unwrap_or("".into()),
			image_url: option.image_url,
			is_correct: None,
			points: option.points,
			created_at: option.created_at,
			updated_at: option.updated_at,
		})
	}

	pub async fn query_option_by_id(&self, id: String) -> Result<OptionsItemDto> {
		let db = &self.state.surrealdb_ws;
		let builder = DetailQueryBuilder::new(ResourceEnum::Options.to_string())
			.with_id(&id)
			.with_select_fields(vec!["*"]);
		let sql = builder.build();
		let result: Option<OptionsSchema> =
			builder.apply_bindings(db.query(sql)).await?.take(0)?;
		let Some(option) = result else {
			bail!("Option not found");
		};
		if option.is_deleted {
			bail!("Option not found");
		}
		Ok(OptionsItemDto {
			id: extract_id(&option.id),
			label: option.label.unwrap_or("".into()),
			image_url: option.image_url,
			is_correct: None,
			points: option.points,
			created_at: option.created_at,
			updated_at: option.updated_at,
		})
	}

	pub async fn query_create_option(&self, data: OptionsSchema) -> Result<String> {
		let db = &self.state.surrealdb_ws;
		let record: Option<OptionsSchema> = db
			.create(ResourceEnum::Options.to_string())
			.content(data)
			.await?;
		match record {
			Some(_) => Ok("Success create option".into()),
			None => bail!("Failed to create option"),
		}
	}

	pub async fn query_update_option(&self, data: OptionsSchema) -> Result<String> {
		let db = &self.state.surrealdb_ws;
		let record_key = get_id(&data.id)?;
		let existing = self.query_raw_option_by_id(&data.id.id.to_raw()).await?;
		if existing.is_deleted {
			bail!("Option already deleted");
		}
		let merged = OptionsSchema {
			created_at: existing.created_at,
			updated_at: get_iso_date(),
			..data.clone()
		};
		let record: Option<OptionsSchema> = db.update(record_key).merge(merged).await?;
		match record {
			Some(_) => Ok("Success update option".into()),
			None => bail!("Failed to update option"),
		}
	}

	pub async fn query_delete_option(&self, id: String) -> Result<String> {
		let db = &self.state.surrealdb_ws;
		let option = self.query_raw_option_by_id(&id).await?;
		if option.is_deleted {
			bail!("Option not found");
		}
		let record_key = get_id(&option.id)?;
		let record: Option<OptionsSchema> = db
			.update(record_key)
			.merge(serde_json::json!({ "is_deleted": true }))
			.await?;
		match record {
			Some(_) => Ok("Success delete option".into()),
			None => bail!("Failed to delete option"),
		}
	}
}
