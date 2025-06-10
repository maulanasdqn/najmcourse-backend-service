use super::{
	SubTestsItemDto, TestsDetailSchema, TestsItemDto, TestsResponseListDto,
	TestsSchema,
};
use crate::{
	OptionsItemDto, OptionsSchema, QuestionsDetailSchema, QuestionsItemDto,
	QuestionsSchema,
};
use anyhow::{Result, bail};
use najm_lib::{AppState, MetaRequestDto, ResourceEnum, ResponseListSuccessDto};
use najm_util::{QueryListBuilder, get_id, get_iso_date};
use surrealdb::sql::Thing;

pub struct TestsRepository<'a> {
	pub state: &'a AppState,
}

impl<'a> TestsRepository<'a> {
	pub fn new(state: &'a AppState) -> Self {
		Self { state }
	}

	pub async fn query_raw_test_by_id(&self, id: &str) -> Result<TestsSchema> {
		let db = &self.state.surrealdb_ws;
		let test: Option<TestsSchema> =
			db.select((ResourceEnum::Tests.to_string(), id)).await?;
		match test {
			Some(t) if !t.is_deleted => Ok(t),
			Some(_) => bail!("Test already deleted"),
			None => bail!("Test not found"),
		}
	}

	pub async fn query_test_list(
		&self,
		meta: MetaRequestDto,
	) -> Result<ResponseListSuccessDto<Vec<TestsResponseListDto>>> {
		let result: ResponseListSuccessDto<Vec<TestsSchema>> = QueryListBuilder::new(
			&self.state.surrealdb_ws,
			&ResourceEnum::Tests.to_string(),
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
			.map(TestsResponseListDto::from)
			.collect();
		Ok(ResponseListSuccessDto {
			data,
			meta: result.meta,
		})
	}

	pub async fn query_test_by_id(&self, id: &str) -> Result<TestsItemDto> {
		let db = &self.state.surrealdb_ws;

		let query = format!(
			"SELECT * FROM {}:⟨{}⟩ WHERE is_deleted = false FETCH questions, questions.options, sub_tests, sub_tests.questions, sub_tests.questions.options",
			ResourceEnum::Tests,
			id
		);

		let mut result = db.query(query).await?;
		let test: Option<TestsDetailSchema> = result.take(0)?;

		let Some(test) = test else {
			bail!("Test not found");
		};

		if test.is_deleted {
			bail!("Test not found");
		}

		let test_id = match &test.id.id {
			surrealdb::sql::Id::String(s) => s.clone(),
			_ => "".to_string(),
		};

		let mut question_items = Vec::new();
		let mut sub_test_items = None;

		if test.category.to_lowercase() == "akademik" {
			if let Some(questions) = test.questions {
				for question in questions.into_iter().filter(|q| !q.is_deleted) {
					let mut option_items = Vec::new();

					for option in question
						.options
						.into_iter()
						.flatten()
						.filter(|opt| !opt.is_deleted)
					{
						let option_id = match &option.id.id {
							surrealdb::sql::Id::String(s) => s.clone(),
							_ => "".to_string(),
						};

						option_items.push(OptionsItemDto {
							id: option_id,
							label: option.label.unwrap_or_default(),
							image_url: option.image_url,
							is_correct: Some(option.is_correct),
							points: option.points,
							created_at: option.created_at,
							updated_at: option.updated_at,
						});
					}

					let question_id = match &question.id.id {
						surrealdb::sql::Id::String(s) => s.clone(),
						_ => "".to_string(),
					};

					question_items.push(QuestionsItemDto {
						id: question_id,
						question: question.question.unwrap_or_default(),
						discussion: question.discussion.unwrap_or_default(),
						question_image_url: question.question_image_url,
						discussion_image_url: question.discussion_image_url,
						options: option_items,
						created_at: question.created_at,
						updated_at: question.updated_at,
					});
				}
			}
		}

		if test.category.to_lowercase() == "psikologi" {
			if let Some(sub_tests) = test.sub_tests {
				let mut sub_tests_vec = Vec::new();

				for sub_test in sub_tests.into_iter().filter(|st| !st.is_deleted) {
					let mut sub_question_items = Vec::new();

					for question in sub_test.questions.into_iter().filter(|q| !q.is_deleted) {
						let mut option_items = Vec::new();

						for option in question
							.options
							.into_iter()
							.flatten()
							.filter(|opt| !opt.is_deleted)
						{
							let option_id = match &option.id.id {
								surrealdb::sql::Id::String(s) => s.clone(),
								_ => "".to_string(),
							};

							option_items.push(OptionsItemDto {
								id: option_id,
								label: option.label.unwrap_or_default(),
								image_url: option.image_url,
								is_correct: Some(option.is_correct),
								points: option.points,
								created_at: option.created_at,
								updated_at: option.updated_at,
							});
						}

						let question_id = match &question.id.id {
							surrealdb::sql::Id::String(s) => s.clone(),
							_ => "".to_string(),
						};

						sub_question_items.push(QuestionsItemDto {
							id: question_id,
							question: question.question.unwrap_or_default(),
							discussion: question.discussion.unwrap_or_default(),
							question_image_url: question.question_image_url,
							discussion_image_url: question.discussion_image_url,
							options: option_items,
							created_at: question.created_at,
							updated_at: question.updated_at,
						});
					}

					let sub_test_id = match &sub_test.id.id {
						surrealdb::sql::Id::String(s) => s.clone(),
						_ => "".to_string(),
					};

					sub_tests_vec.push(SubTestsItemDto {
						id: sub_test_id,
						name: sub_test.name,
						banner: sub_test.banner,
						category: sub_test.category,
						questions: sub_question_items,
						created_at: sub_test.created_at,
						updated_at: sub_test.updated_at,
					});
				}
				sub_test_items = Some(sub_tests_vec);
			}
		}

		Ok(TestsItemDto {
			id: test_id,
			name: test.name,
			banner: test.banner,
			category: test.category,
			questions: Some(question_items),
			sub_tests: sub_test_items,
			created_at: test.created_at,
			updated_at: test.updated_at,
		})
	}

	pub async fn query_test_by_name(&self, name: &str) -> Result<TestsItemDto> {
		let db = &self.state.surrealdb_ws;
		let query = format!(
			"SELECT * FROM {} WHERE name = $name AND is_deleted = false LIMIT 1 FETCH questions, questions.options",
			ResourceEnum::Tests
		);
		let mut result = db.query(query).bind(("name", name.to_string())).await?;
		let test: Option<TestsDetailSchema> = result.take(0)?;
		let Some(t) = test else {
			bail!("Test not found");
		};
		let TestsDetailSchema {
			id,
			name,
			banner,
			questions,
			sub_tests: _,
			is_deleted: _,
			category,
			created_at,
			updated_at,
		} = t;
		let mut question_items = Vec::new();
		for q in questions.into_iter().flatten().filter(|q| !q.is_deleted) {
			let QuestionsDetailSchema {
				id,
				question,
				discussion,
				question_image_url,
				discussion_image_url,
				options,
				is_deleted: _,
				created_at,
				updated_at,
			} = q;
			let mut option_items = Vec::new();
			for opt in options.into_iter().flatten().filter(|opt| !opt.is_deleted) {
				let id = match &opt.id.id {
					surrealdb::sql::Id::String(s) => s.clone(),
					_ => "".to_string(),
				};
				option_items.push(OptionsItemDto {
					id,
					label: opt.label.unwrap_or("".into()),
					image_url: opt.image_url,
					is_correct: None,
					points: None,
					created_at: opt.created_at,
					updated_at: opt.updated_at,
				});
			}

			question_items.push(QuestionsItemDto {
				id: match &id.id {
					surrealdb::sql::Id::String(s) => s.clone(),
					_ => "".to_string(),
				},
				question: question.unwrap_or("".into()),
				discussion: discussion.unwrap_or("".into()),
				question_image_url,
				discussion_image_url,
				options: option_items,
				created_at,
				updated_at,
			});
		}
		Ok(TestsItemDto {
			id: match &id.id {
				surrealdb::sql::Id::String(s) => s.clone(),
				_ => "".to_string(),
			},
			name,
			banner,
			category,
			questions: Some(question_items),
			sub_tests: None,
			created_at,
			updated_at,
		})
	}

	pub async fn query_create_test(&self, data: TestsSchema) -> Result<String> {
		let db = &self.state.surrealdb_ws;
		let record: Option<TestsSchema> = db
			.create(ResourceEnum::Tests.to_string())
			.content(data)
			.await?;
		match record {
			Some(_) => Ok("Success create test".into()),
			None => bail!("Failed to create test"),
		}
	}

	pub async fn query_update_test(&self, data: TestsSchema) -> Result<String> {
		let db = &self.state.surrealdb_ws;
		let record_key = get_id(&data.id)?;
		let existing = self.query_raw_test_by_id(&data.id.id.to_raw()).await?;
		if existing.is_deleted {
			bail!("Test already deleted");
		}
		let merged = TestsSchema {
			created_at: existing.created_at,
			updated_at: get_iso_date(),
			..data.clone()
		};
		let record: Option<TestsSchema> = db.update(record_key).merge(merged).await?;
		match record {
			Some(_) => Ok("Success update test".into()),
			None => bail!("Failed to update test"),
		}
	}

	pub async fn query_delete_test(&self, id: String) -> Result<String> {
		let db = &self.state.surrealdb_ws;
		let test = self.query_raw_test_by_id(&id).await?;
		if test.is_deleted {
			bail!("Test not found");
		}
		let record_key = get_id(&test.id)?;
		let record: Option<TestsSchema> = db
			.update(record_key)
			.merge(serde_json::json!({ "is_deleted": true }))
			.await?;
		match record {
			Some(_) => Ok("Success delete test".into()),
			None => bail!("Failed to delete test"),
		}
	}

	pub async fn query_create_options(
		&self,
		options: Vec<OptionsSchema>,
	) -> Result<Vec<Thing>> {
		let db = &self.state.surrealdb_ws;
		let mut option_ids = Vec::new();

		for option in options {
			let record: Option<OptionsSchema> = db
				.create(ResourceEnum::Options.to_string())
				.content(option)
				.await?;
			match record {
				Some(opt) => option_ids.push(opt.id),
				None => bail!("Failed to create option"),
			}
		}

		Ok(option_ids)
	}

	pub async fn query_create_questions(
		&self,
		questions: Vec<QuestionsSchema>,
	) -> Result<Vec<Thing>> {
		let db = &self.state.surrealdb_ws;
		let mut question_ids = Vec::new();

		for question in questions {
			let record: Option<QuestionsSchema> = db
				.create(ResourceEnum::Questions.to_string())
				.content(question)
				.await?;
			match record {
				Some(q) => question_ids.push(q.id),
				None => bail!("Failed to create question"),
			}
		}

		Ok(question_ids)
	}

	pub async fn query_create_sub_tests(
		&self,
		sub_tests: Vec<super::SubTestsSchema>,
	) -> Result<Vec<Thing>> {
		let db = &self.state.surrealdb_ws;
		let mut sub_test_ids = Vec::new();

		for sub_test in sub_tests {
			let record: Option<super::SubTestsSchema> = db
				.create(ResourceEnum::SubTests.to_string())
				.content(sub_test)
				.await?;
			match record {
				Some(st) => sub_test_ids.push(st.id),
				None => bail!("Failed to create sub-test"),
			}
		}

		Ok(sub_test_ids)
	}

	pub async fn query_create_test_with_relations(
		&self,
		payload: super::TestsCreateRequestDto,
	) -> Result<String> {
		let db = &self.state.surrealdb_ws;
		let mut test_schema = TestsSchema::create(payload.clone());

		match payload.category.to_lowercase().as_str() {
			"akademik" => {
				if let Some(questions_dto) = payload.questions {
					let mut question_things = Vec::new();
					for q_dto in questions_dto {
						let option_schemas: Vec<OptionsSchema> = q_dto
							.options
							.clone()
							.into_iter()
							.map(OptionsSchema::create)
							.collect();
						let option_ids = self.query_create_options(option_schemas).await?;
						let question_schema = QuestionsSchema::create(q_dto, option_ids);
						let question_ids =
							self.query_create_questions(vec![question_schema]).await?;
						question_things.extend(question_ids);
					}
					test_schema.questions = question_things;
				}
			}
			"psikologi" => {
				if let Some(sub_tests_dto) = payload.sub_tests {
					let mut sub_test_things = Vec::new();
					for st_dto in sub_tests_dto {
						let mut question_things = Vec::new();
						for q_dto in st_dto.questions {
							let option_schemas: Vec<OptionsSchema> = q_dto
								.options
								.clone()
								.into_iter()
								.map(OptionsSchema::create)
								.collect();
							let option_ids = self.query_create_options(option_schemas).await?;
							let question_schema = QuestionsSchema::create(q_dto, option_ids);
							let question_ids =
								self.query_create_questions(vec![question_schema]).await?;
							question_things.extend(question_ids);
						}
						let sub_test_schema = super::SubTestsSchema {
							id: najm_util::make_thing(
								&ResourceEnum::SubTests.to_string(),
								&surrealdb::Uuid::new_v4().to_string(),
							),
							name: st_dto.name,
							questions: question_things,
							banner: st_dto.banner,
							category: st_dto.category,
							is_deleted: false,
							created_at: najm_util::get_iso_date(),
							updated_at: najm_util::get_iso_date(),
						};

						let sub_test_ids =
							self.query_create_sub_tests(vec![sub_test_schema]).await?;
						sub_test_things.extend(sub_test_ids);
					}
					test_schema.sub_tests = sub_test_things;
				}
			}
			_ => {}
		}
		let record: Option<TestsSchema> = db
			.create(ResourceEnum::Tests.to_string())
			.content(test_schema)
			.await?;

		match record {
			Some(_) => Ok("Success create test with separate entities".into()),
			None => bail!("Failed to create test"),
		}
	}

	pub async fn query_update_test_with_relations(
		&self,
		id: String,
		payload: super::TestsUpdateRequestDto,
	) -> Result<String> {
		let db = &self.state.surrealdb_ws;
		let existing = self.query_raw_test_by_id(&id).await?;
		if existing.is_deleted {
			bail!("Test already deleted");
		}
		let create_payload = super::TestsCreateRequestDto {
			name: payload.name,
			questions: payload.questions.map(|questions| {
				questions
					.into_iter()
					.map(|q| crate::QuestionsCreateRequestDto {
						question: q.question,
						discussion: q.discussion,
						question_image_url: q.question_image_url,
						discussion_image_url: q.discussion_image_url,
						options: q
							.options
							.into_iter()
							.map(|opt| crate::OptionsCreateRequestDto {
								label: opt.label,
								image_url: opt.image_url,
								is_correct: opt.is_correct,
								points: opt.points,
							})
							.collect(),
					})
					.collect()
			}),
			sub_tests: payload.sub_tests.map(|sub_tests| {
				sub_tests
					.into_iter()
					.map(|st| super::SubTestsCreateRequestDto {
						name: st.name,
						banner: st.banner,
						category: st.category,
						questions: st
							.questions
							.into_iter()
							.map(|q| crate::QuestionsCreateRequestDto {
								question: q.question,
								discussion: q.discussion,
								question_image_url: q.question_image_url,
								discussion_image_url: q.discussion_image_url,
								options: q
									.options
									.into_iter()
									.map(|opt| crate::OptionsCreateRequestDto {
										label: opt.label,
										image_url: opt.image_url,
										is_correct: opt.is_correct,
										points: opt.points,
									})
									.collect(),
							})
							.collect(),
					})
					.collect()
			}),
			banner: payload.banner,
			category: payload.category.clone(),
		};

		let mut test_schema = TestsSchema::create(create_payload.clone());

		match payload.category.to_lowercase().as_str() {
			"akademik" => {
				if let Some(questions_dto) = create_payload.questions {
					let mut question_things = Vec::new();
					for q_dto in questions_dto {
						let option_schemas: Vec<OptionsSchema> = q_dto
							.options
							.clone()
							.into_iter()
							.map(OptionsSchema::create)
							.collect();
						let option_ids = self.query_create_options(option_schemas).await?;
						let question_schema = QuestionsSchema::create(q_dto, option_ids);
						let question_ids =
							self.query_create_questions(vec![question_schema]).await?;
						question_things.extend(question_ids);
					}
					test_schema.questions = question_things;
				}
			}
			"psikologi" => {
				if let Some(sub_tests_dto) = create_payload.sub_tests {
					let mut sub_test_things = Vec::new();
					for st_dto in sub_tests_dto {
						let mut question_things = Vec::new();
						for q_dto in st_dto.questions {
							let option_schemas: Vec<OptionsSchema> = q_dto
								.options
								.clone()
								.into_iter()
								.map(OptionsSchema::create)
								.collect();
							let option_ids = self.query_create_options(option_schemas).await?;
							let question_schema = QuestionsSchema::create(q_dto, option_ids);
							let question_ids =
								self.query_create_questions(vec![question_schema]).await?;
							question_things.extend(question_ids);
						}

						let sub_test_schema = super::SubTestsSchema {
							id: najm_util::make_thing(
								&ResourceEnum::SubTests.to_string(),
								&surrealdb::Uuid::new_v4().to_string(),
							),
							name: st_dto.name,
							questions: question_things,
							banner: st_dto.banner,
							category: st_dto.category,
							is_deleted: false,
							created_at: najm_util::get_iso_date(),
							updated_at: najm_util::get_iso_date(),
						};

						let sub_test_ids =
							self.query_create_sub_tests(vec![sub_test_schema]).await?;
						sub_test_things.extend(sub_test_ids);
					}
					test_schema.sub_tests = sub_test_things;
				}
			}
			_ => {}
		}

		let updated_schema = TestsSchema {
			id: najm_util::make_thing(&ResourceEnum::Tests.to_string(), &id),
			created_at: existing.created_at,
			updated_at: najm_util::get_iso_date(),
			..test_schema
		};

		let record_key = get_id(&updated_schema.id)?;
		let record: Option<TestsSchema> =
			db.update(record_key).content(updated_schema).await?;

		match record {
			Some(_) => Ok("Success update test with separate entities".into()),
			None => bail!("Failed to update test"),
		}
	}
}
