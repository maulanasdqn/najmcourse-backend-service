use super::{
	QuestionsDetailSchema, QuestionsItemDto, QuestionsResponseListDto, QuestionsSchema,
};
use anyhow::{Result, bail};
use najm_lib::{AppState, MetaRequestDto, ResourceEnum, ResponseListSuccessDto};
use najm_util::{DetailQueryBuilder, QueryListBuilder, get_id, get_iso_date};

pub struct QuestionsRepository<'a> {
	state: &'a AppState,
}

impl<'a> QuestionsRepository<'a> {
	pub fn new(state: &'a AppState) -> Self {
		Self { state }
	}

	pub async fn query_raw_question_by_id(&self, id: &str) -> Result<QuestionsSchema> {
		let db = &self.state.surrealdb_ws;
		let question: Option<QuestionsSchema> =
			db.select((ResourceEnum::Questions.to_string(), id)).await?;
		match question {
			Some(q) if !q.is_deleted => Ok(q),
			_ => bail!("Question not found"),
		}
	}

	pub async fn query_question_list(
		&self,
		meta: MetaRequestDto,
	) -> Result<ResponseListSuccessDto<Vec<QuestionsResponseListDto>>> {
		let result: ResponseListSuccessDto<Vec<QuestionsSchema>> =
			QueryListBuilder::new(
				&self.state.surrealdb_ws,
				&ResourceEnum::Questions.to_string(),
				&meta,
			)
			.with_condition("is_deleted = false")
			.search_field("question")
			.select_fields(vec!["*"])
			.build()
			.await?;
		let data = result
			.data
			.into_iter()
			.map(QuestionsResponseListDto::from)
			.collect();
		Ok(ResponseListSuccessDto {
			data,
			meta: result.meta,
		})
	}

	pub async fn query_question_by_id(&self, id: &str) -> Result<QuestionsItemDto> {
		let db = &self.state.surrealdb_ws;
		let builder = DetailQueryBuilder::new(ResourceEnum::Questions.to_string())
			.with_id(id)
			.with_select_fields(vec!["*"])
			.with_fetch("options");
		let sql = builder.build();
		let result: Option<QuestionsDetailSchema> =
			builder.apply_bindings(db.query(sql)).await?.take(0)?;
		let Some(question) = result else {
			bail!("Question not found");
		};
		if question.is_deleted {
			bail!("Question not found");
		}
		let options = question.clone().options;
		Ok(QuestionsItemDto::from_with_options(question, options))
	}

	pub async fn query_create_question(
		&self,
		data: QuestionsSchema,
	) -> Result<String> {
		let db = &self.state.surrealdb_ws;
		let record: Option<QuestionsSchema> = db
			.create(ResourceEnum::Questions.to_string())
			.content(data)
			.await?;
		match record {
			Some(_) => Ok("Success create question".into()),
			None => bail!("Failed to create question"),
		}
	}

	pub async fn query_update_question(
		&self,
		data: QuestionsSchema,
	) -> Result<String> {
		let db = &self.state.surrealdb_ws;
		let record_key = get_id(&data.id)?;
		let existing = self.query_raw_question_by_id(&data.id.id.to_raw()).await?;
		if existing.is_deleted {
			bail!("Question already deleted");
		}
		let merged = QuestionsSchema {
			created_at: existing.created_at,
			updated_at: get_iso_date(),
			..data.clone()
		};
		let record: Option<QuestionsSchema> =
			db.update(record_key).merge(merged).await?;
		match record {
			Some(_) => Ok("Success update question".into()),
			None => bail!("Failed to update question"),
		}
	}

	pub async fn query_delete_question(&self, id: String) -> Result<String> {
		let db = &self.state.surrealdb_ws;
		let question = self.query_raw_question_by_id(&id).await?;
		if question.is_deleted {
			bail!("Question not found");
		}
		let record_key = get_id(&question.id)?;
		let record: Option<QuestionsSchema> = db
			.update(record_key)
			.merge(serde_json::json!({ "is_deleted": true }))
			.await?;
		match record {
			Some(_) => Ok("Success delete question".into()),
			None => bail!("Failed to delete question"),
		}
	}
}
