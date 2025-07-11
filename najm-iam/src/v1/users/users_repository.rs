use super::{UsersDetailQueryDto, UsersListItemDto, UsersListQueryDto, UsersSchema};
use crate::{
	AppState, MetaRequestDto, ResourceEnum, ResponseListSuccessDto, get_id,
};
use anyhow::{Result, bail};
use najm_util::{DetailQueryBuilder, QueryListBuilder};

pub struct UsersRepository<'a> {
	state: &'a AppState,
}

impl<'a> UsersRepository<'a> {
	pub fn new(state: &'a AppState) -> Self {
		Self { state }
	}

	pub async fn query_user_list(
		&self,
		meta: MetaRequestDto,
	) -> Result<ResponseListSuccessDto<Vec<UsersListItemDto>>> {
		let result: ResponseListSuccessDto<Vec<UsersListQueryDto>> =
			QueryListBuilder::new(
				&self.state.surrealdb_ws,
				&ResourceEnum::Users.to_string(),
				&meta,
			)
			.with_condition("is_deleted = false")
			.search_field("fullname")
			.select_fields(vec![
				"id",
				"email",
				"fullname",
				"phone_number",
				"avatar",
				"student_type",
				"role",
				"is_active",
				"created_at",
				"updated_at",
			])
			.fetch_fields(vec!["role", "role.permissions"])
			.build()
			.await?;
		let data = result
			.data
			.into_iter()
			.map(UsersListQueryDto::from)
			.collect();
		Ok(ResponseListSuccessDto {
			data,
			meta: result.meta,
		})
	}

	pub async fn query_user_by_email(
		&self,
		email: String,
	) -> Result<UsersDetailQueryDto> {
		let db = &self.state.surrealdb_ws;
		let builder = DetailQueryBuilder::new(ResourceEnum::Users.to_string())
			.with_where("email")
			.where_value(email.clone())
			.with_select_fields(vec!["*"])
			.with_fetch("role")
			.with_fetch("role.permissions");
		let sql = builder.build();
		let user_opt: Option<UsersDetailQueryDto> =
			builder.apply_bindings(db.query(sql)).await?.take(0)?;
		let Some(user) = user_opt else {
			bail!("User not found");
		};
		if user.is_deleted {
			bail!("User not found");
		}
		if user.role.is_deleted {
			bail!("User not found");
		}
		Ok(UsersDetailQueryDto::from(&user))
	}

	pub async fn query_user_by_id(&self, id: String) -> Result<UsersDetailQueryDto> {
		let db = &self.state.surrealdb_ws;
		let builder = DetailQueryBuilder::new(ResourceEnum::Users.to_string())
			.with_id(&id)
			.with_select_fields(vec!["*"])
			.with_fetch("role")
			.with_fetch("role.permissions");
		let sql = builder.build();
		let result: Option<UsersDetailQueryDto> =
			builder.apply_bindings(db.query(sql)).await?.take(0)?;
		let Some(user) = result else {
			bail!("User not found");
		};
		if user.is_deleted {
			bail!("User not found");
		}
		if user.role.is_deleted {
			bail!("User not found");
		}
		Ok(UsersDetailQueryDto::from(&user))
	}

	pub async fn query_create_user(&self, data: UsersSchema) -> Result<String> {
		let db = &self.state.surrealdb_ws;
		let record: Option<UsersSchema> = db
			.create(ResourceEnum::Users.to_string())
			.content(data)
			.await?;
		match record {
			Some(_) => Ok("Success create user".into()),
			None => bail!("Failed to create user"),
		}
	}

	pub async fn query_update_user(&self, data: UsersSchema) -> Result<String> {
		let db = &self.state.surrealdb_ws;
		let record_key = get_id(&data.id)?;
		let existing = self.query_user_by_id(data.id.id.to_raw()).await?;
		if existing.is_deleted {
			bail!("User already deleted");
		}
		let role_thing = if data.role == existing.role.id {
			existing.role.id.clone()
		} else {
			data.role.clone()
		};
		let merged = UsersSchema {
			password: existing.password,
			created_at: existing.created_at,
			role: role_thing,
			..data.clone()
		};
		let record: Option<UsersSchema> = db.update(record_key).merge(merged).await?;
		match record {
			Some(_) => Ok("Success update user".into()),
			None => bail!("Failed to update user"),
		}
	}

	pub async fn query_update_user_me(&self, data: UsersSchema) -> Result<String> {
		let db = &self.state.surrealdb_ws;
		let record_key = get_id(&data.id)?;
		let existing = self.query_user_by_id(data.id.id.to_raw()).await?;
		if existing.is_deleted {
			bail!("User already deleted");
		}
		let role_thing = if data.role == existing.role.id {
			existing.role.id.clone()
		} else {
			data.role.clone()
		};
		let merged = UsersSchema {
			password: existing.password,
			created_at: existing.created_at,
			role: role_thing,
			is_profile_completed: true,
			..data.clone()
		};
		let record: Option<UsersSchema> = db.update(record_key).merge(merged).await?;
		match record {
			Some(_) => Ok("Success update user".into()),
			None => bail!("Failed to update user"),
		}
	}

	pub async fn query_delete_user(&self, id: String) -> Result<String> {
		let db = &self.state.surrealdb_ws;
		let user = self.query_user_by_id(id).await?;
		if user.is_deleted {
			bail!("User not found");
		}
		let record_key = get_id(&user.id)?;
		let record: Option<UsersSchema> = db
			.update(record_key)
			.merge(serde_json::json!({ "is_deleted": true }))
			.await?;
		match record {
			Some(_) => Ok("Success delete user".into()),
			None => bail!("Failed to delete user"),
		}
	}
}
