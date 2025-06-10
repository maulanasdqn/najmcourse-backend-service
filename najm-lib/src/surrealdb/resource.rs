use std::fmt;
use surrealdb::{Uuid, sql::Thing};

fn make_thing(table: &str, id: &str) -> Thing {
	Thing::from((table, id))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResourceEnum {
	OtpCache,
	UsersCache,
	Tests,
	SubTests,
	Sessions,
	Options,
	Questions,
	Users,
	Roles,
	Flags,
	Answers,
	Permissions,
	Events,
}

impl fmt::Display for ResourceEnum {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let str = match self {
			ResourceEnum::Events => "app_events",
			ResourceEnum::SubTests => "app_sub_tests",
			ResourceEnum::Users => "app_users",
			ResourceEnum::Flags => "app_flags",
			ResourceEnum::Answers => "app_answers",
			ResourceEnum::UsersCache => "app_users_cache",
			ResourceEnum::OtpCache => "app_otp_cache",
			ResourceEnum::Roles => "app_roles",
			ResourceEnum::Permissions => "app_permissions",
			ResourceEnum::Options => "app_options",
			ResourceEnum::Questions => "app_questions",
			ResourceEnum::Tests => "app_tests",
			ResourceEnum::Sessions => "app_sessions",
		};
		write!(f, "{}", str)
	}
}

impl ResourceEnum {
	pub fn thing(&self) -> Thing {
		match self {
			ResourceEnum::Events => make_thing(
				&ResourceEnum::Events.to_string(),
				&Uuid::new_v4().to_string(),
			),
			ResourceEnum::SubTests => make_thing(
				&ResourceEnum::SubTests.to_string(),
				&Uuid::new_v4().to_string(),
			),
			ResourceEnum::Users => make_thing(
				&ResourceEnum::Users.to_string(),
				&Uuid::new_v4().to_string(),
			),
			ResourceEnum::Flags => make_thing(
				&ResourceEnum::Flags.to_string(),
				&Uuid::new_v4().to_string(),
			),
			ResourceEnum::Answers => make_thing(
				&ResourceEnum::Answers.to_string(),
				&Uuid::new_v4().to_string(),
			),
			ResourceEnum::UsersCache => make_thing(
				&ResourceEnum::UsersCache.to_string(),
				&Uuid::new_v4().to_string(),
			),
			ResourceEnum::OtpCache => make_thing(
				&ResourceEnum::OtpCache.to_string(),
				&Uuid::new_v4().to_string(),
			),
			ResourceEnum::Roles => make_thing(
				&ResourceEnum::Roles.to_string(),
				&Uuid::new_v4().to_string(),
			),
			ResourceEnum::Permissions => make_thing(
				&ResourceEnum::Permissions.to_string(),
				&Uuid::new_v4().to_string(),
			),
			ResourceEnum::Options => make_thing(
				&ResourceEnum::Options.to_string(),
				&Uuid::new_v4().to_string(),
			),
			ResourceEnum::Questions => make_thing(
				&ResourceEnum::Questions.to_string(),
				&Uuid::new_v4().to_string(),
			),
			ResourceEnum::Tests => make_thing(
				&ResourceEnum::Tests.to_string(),
				&Uuid::new_v4().to_string(),
			),
			ResourceEnum::Sessions => make_thing(
				&ResourceEnum::Sessions.to_string(),
				&Uuid::new_v4().to_string(),
			),
		}
	}
}
