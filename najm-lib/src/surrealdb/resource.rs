use std::fmt;

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
