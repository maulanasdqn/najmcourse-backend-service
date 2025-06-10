use crate::ResourceEnum;
use najm_util::make_thing;
use std::fmt;
use surrealdb::sql::Thing;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RolesEnum {
	Admin,
	Student,
	Staff,
}

impl fmt::Display for RolesEnum {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let roles_str = match self {
			RolesEnum::Admin => "Admin",
			RolesEnum::Student => "Student",
			RolesEnum::Staff => "Staff",
		};
		write!(f, "{}", roles_str)
	}
}

impl RolesEnum {
	pub fn id(&self) -> &'static str {
		match self {
			RolesEnum::Admin => "f6b03f25-e416-4893-ac88-caaa690afb07",
			RolesEnum::Student => "5713cb37-dc02-4e87-8048-d7a41d352059",
			RolesEnum::Staff => "aec758fc-3d54-4c6f-8bcb-44368dd81c07",
		}
	}

	pub fn thing(&self) -> Thing {
		match self {
			RolesEnum::Admin => {
				make_thing(&ResourceEnum::Roles.to_string(), &RolesEnum::Admin.id())
			}
			RolesEnum::Student => {
				make_thing(&ResourceEnum::Roles.to_string(), &RolesEnum::Student.id())
			}
			RolesEnum::Staff => {
				make_thing(&ResourceEnum::Roles.to_string(), &RolesEnum::Staff.id())
			}
		}
	}
}
