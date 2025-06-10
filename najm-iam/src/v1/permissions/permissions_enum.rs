use najm_lib::ResourceEnum;
use najm_util::make_thing;
use std::fmt;
use surrealdb::sql::Thing;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PermissionsEnum {
	ReadDashboard,
	ReadListUsers,
	ReadDetailUsers,
	CreateUsers,
	DeleteUsers,
	UpdateUsers,
	ActivateUsers,
	ReadListRoles,
	ReadDetailRoles,
	CreateRoles,
	DeleteRoles,
	UpdateRoles,
	ReadListPermissions,
	ReadDetailPermissions,
	CreatePermissions,
	DeletePermissions,
	UpdatePermissions,
	ReadListSessions,
	ReadDetailSessions,
	CreateSessions,
	UpdateSessions,
	DeleteSessions,
	ReadListTests,
	ReadDetailTests,
	CreateTests,
	UpdateTests,
	DeleteTests,
	ReadListAnswers,
	ReadDetailAnswers,
	CreateAnswers,
	UpdateAnswers,
	DeleteAnswers,
	ReadListOptions,
	ReadDetailOptions,
	CreateOptions,
	UpdateOptions,
	DeleteOptions,
	ReadListQuestions,
	ReadDetailQuestions,
	CreateQuestions,
	UpdateQuestions,
	DeleteQuestions,
}

impl fmt::Display for PermissionsEnum {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let permission_str = match self {
			PermissionsEnum::ReadDashboard => "Read Dashboard",
			PermissionsEnum::ReadListUsers => "Read List Users",
			PermissionsEnum::ReadDetailUsers => "Read Detail Users",
			PermissionsEnum::CreateUsers => "Create Users",
			PermissionsEnum::DeleteUsers => "Delete Users",
			PermissionsEnum::UpdateUsers => "Update Users",
			PermissionsEnum::ActivateUsers => "Activate Users",
			PermissionsEnum::ReadListRoles => "Read List Roles",
			PermissionsEnum::ReadDetailRoles => "Read Detail Roles",
			PermissionsEnum::CreateRoles => "Create Roles",
			PermissionsEnum::DeleteRoles => "Delete Roles",
			PermissionsEnum::UpdateRoles => "Update Roles",
			PermissionsEnum::ReadListPermissions => "Read List Permissions",
			PermissionsEnum::ReadDetailPermissions => "Read Detail Permissions",
			PermissionsEnum::CreatePermissions => "Create Permissions",
			PermissionsEnum::DeletePermissions => "Delete Permissions",
			PermissionsEnum::UpdatePermissions => "Update Permissions",
			PermissionsEnum::ReadListSessions => "Read List Sessions",
			PermissionsEnum::ReadDetailSessions => "Read Detail Sessions",
			PermissionsEnum::CreateSessions => "Create Sessions",
			PermissionsEnum::UpdateSessions => "Update Sessions",
			PermissionsEnum::DeleteSessions => "Delete Sessions",
			PermissionsEnum::ReadListTests => "Read List Tests",
			PermissionsEnum::ReadDetailTests => "Read Detail Tests",
			PermissionsEnum::CreateTests => "Create Tests",
			PermissionsEnum::UpdateTests => "Update Tests",
			PermissionsEnum::DeleteTests => "Delete Tests",
			PermissionsEnum::ReadListAnswers => "Read List Answers",
			PermissionsEnum::ReadDetailAnswers => "Read Detail Answers",
			PermissionsEnum::CreateAnswers => "Create Answers",
			PermissionsEnum::UpdateAnswers => "Update Answers",
			PermissionsEnum::DeleteAnswers => "Delete Answers",
			PermissionsEnum::ReadListOptions => "Read List Options",
			PermissionsEnum::ReadDetailOptions => "Read Detail Options",
			PermissionsEnum::CreateOptions => "Create Options",
			PermissionsEnum::UpdateOptions => "Update Options",
			PermissionsEnum::DeleteOptions => "Delete Options",
			PermissionsEnum::ReadListQuestions => "Read List Questions",
			PermissionsEnum::ReadDetailQuestions => "Read Detail Questions",
			PermissionsEnum::CreateQuestions => "Create Questions",
			PermissionsEnum::UpdateQuestions => "Update Questions",
			PermissionsEnum::DeleteQuestions => "Delete Questions",
		};
		write!(f, "{}", permission_str)
	}
}

impl PermissionsEnum {
	pub fn id(&self) -> &'static str {
		match self {
			PermissionsEnum::ReadDashboard => "f47ac10b-58cc-4372-a567-0e02b2c3d479",
			PermissionsEnum::ReadListUsers => "7c15e31d-36e2-49f9-97db-138c03fb0cf6",
			PermissionsEnum::ReadDetailUsers => "319ee593-ff0a-4f29-bbaf-9feb3174a3a6",
			PermissionsEnum::CreateUsers => "023e2dfe-93c3-4008-94a8-b5dff403f73b",
			PermissionsEnum::DeleteUsers => "96df0689-2ae9-4894-bf00-837c19415e5c",
			PermissionsEnum::UpdateUsers => "98b3dc4c-0124-461f-afcd-166637c5e6e8",
			PermissionsEnum::ActivateUsers => "4da8b434-89f9-4d91-85ae-eebd63cdbeda",
			PermissionsEnum::ReadListRoles => "9164ca6e-c7e3-4238-a15f-f36ab9577e7e",
			PermissionsEnum::ReadDetailRoles => "73888d18-b3e9-4f62-95a5-ba2c0d69fccb",
			PermissionsEnum::CreateRoles => "319ee593-ff0a-4f29-bbaf-9feb3174a3a2",
			PermissionsEnum::DeleteRoles => "35b0d992-65c8-4b62-b030-e6e0320e4048",
			PermissionsEnum::UpdateRoles => "a00d5608-4c48-4542-845c-dfe004687022",
			PermissionsEnum::ReadListPermissions => "8195eeb8-e64f-4172-aa57-596492c84a72",
			PermissionsEnum::ReadDetailPermissions => {
				"dad435cf-042c-41bd-a946-cea61ed2ffbc"
			}
			PermissionsEnum::CreatePermissions => "0269ed71-0ae0-4c43-ad29-e3d861d8f9a0",
			PermissionsEnum::DeletePermissions => "b2dc3928-86ba-4c59-a03d-0b57d5183ebc",
			PermissionsEnum::UpdatePermissions => "299cb4d5-6556-4cc9-b6c1-32e6d31e0f9b",
			PermissionsEnum::ReadListSessions => "1a2b3c4d-5e6f-7890-abcd-ef1234567890",
			PermissionsEnum::ReadDetailSessions => "2b3c4d5e-6f78-90ab-cdef-123456789012",
			PermissionsEnum::CreateSessions => "3c4d5e6f-7890-abcd-ef12-345678901234",
			PermissionsEnum::UpdateSessions => "4d5e6f78-90ab-cdef-1234-567890123456",
			PermissionsEnum::DeleteSessions => "5e6f7890-abcd-ef12-3456-789012345678",
			PermissionsEnum::ReadListTests => "6f789012-3456-7890-abcd-ef1234567890",
			PermissionsEnum::ReadDetailTests => "78901234-5678-90ab-cdef-123456789012",
			PermissionsEnum::CreateTests => "89012345-6789-0abc-def1-234567890123",
			PermissionsEnum::UpdateTests => "90123456-789a-bcde-f123-456789012345",
			PermissionsEnum::DeleteTests => "01234567-89ab-cdef-1234-56789012345a",
			PermissionsEnum::ReadListAnswers => "12345678-9abc-def1-2345-6789012345ab",
			PermissionsEnum::ReadDetailAnswers => "23456789-abcd-ef12-3456-789012345abc",
			PermissionsEnum::CreateAnswers => "3456789a-bcde-f123-4567-89012345abcd",
			PermissionsEnum::UpdateAnswers => "456789ab-cdef-1234-5678-9012345abcde",
			PermissionsEnum::DeleteAnswers => "56789abc-def1-2345-6789-012345abcdef",
			PermissionsEnum::ReadListOptions => "6789abcd-ef12-3456-7890-12345abcdef1",
			PermissionsEnum::ReadDetailOptions => "789abcde-f123-4567-8901-2345abcdef12",
			PermissionsEnum::CreateOptions => "89abcdef-1234-5678-9012-345abcdef123",
			PermissionsEnum::UpdateOptions => "9abcdef1-2345-6789-0123-45abcdef1234",
			PermissionsEnum::DeleteOptions => "abcdef12-3456-7890-1234-5abcdef12345",
			PermissionsEnum::ReadListQuestions => "bcdef123-4567-8901-2345-abcdef123456",
			PermissionsEnum::ReadDetailQuestions => "cdef1234-5678-9012-3456-bcdef1234567",
			PermissionsEnum::CreateQuestions => "def12345-6789-0123-4567-cdef12345678",
			PermissionsEnum::UpdateQuestions => "ef123456-789a-1234-5678-def123456789",
			PermissionsEnum::DeleteQuestions => "f1234567-89ab-2345-6789-ef123456789a",
		}
	}

	pub fn thing(&self) -> Thing {
		match self {
			PermissionsEnum::ReadDashboard => make_thing(
				&ResourceEnum::Permissions.to_string(),
				&PermissionsEnum::ReadDashboard.id(),
			),
			PermissionsEnum::ReadListUsers => make_thing(
				&ResourceEnum::Permissions.to_string(),
				&PermissionsEnum::ReadListUsers.id(),
			),
			PermissionsEnum::ReadDetailUsers => make_thing(
				&ResourceEnum::Permissions.to_string(),
				&PermissionsEnum::ReadDetailUsers.id(),
			),
			PermissionsEnum::CreateUsers => make_thing(
				&ResourceEnum::Permissions.to_string(),
				&PermissionsEnum::CreateUsers.id(),
			),
			PermissionsEnum::DeleteUsers => make_thing(
				&ResourceEnum::Permissions.to_string(),
				&PermissionsEnum::DeleteUsers.id(),
			),
			PermissionsEnum::UpdateUsers => make_thing(
				&ResourceEnum::Permissions.to_string(),
				&PermissionsEnum::UpdateUsers.id(),
			),
			PermissionsEnum::ActivateUsers => make_thing(
				&ResourceEnum::Permissions.to_string(),
				&PermissionsEnum::ActivateUsers.id(),
			),
			PermissionsEnum::ReadListRoles => make_thing(
				&ResourceEnum::Permissions.to_string(),
				&PermissionsEnum::ReadListRoles.id(),
			),
			PermissionsEnum::ReadDetailRoles => make_thing(
				&ResourceEnum::Permissions.to_string(),
				&PermissionsEnum::ReadDetailRoles.id(),
			),
			PermissionsEnum::CreateRoles => make_thing(
				&ResourceEnum::Permissions.to_string(),
				&PermissionsEnum::CreateRoles.id(),
			),
			PermissionsEnum::DeleteRoles => make_thing(
				&ResourceEnum::Permissions.to_string(),
				&PermissionsEnum::DeleteRoles.id(),
			),
			PermissionsEnum::UpdateRoles => make_thing(
				&ResourceEnum::Permissions.to_string(),
				&PermissionsEnum::UpdateRoles.id(),
			),
			PermissionsEnum::ReadListPermissions => make_thing(
				&ResourceEnum::Permissions.to_string(),
				&PermissionsEnum::ReadListPermissions.id(),
			),
			PermissionsEnum::ReadDetailPermissions => make_thing(
				&ResourceEnum::Permissions.to_string(),
				&PermissionsEnum::ReadDetailPermissions.id(),
			),
			PermissionsEnum::CreatePermissions => make_thing(
				&ResourceEnum::Permissions.to_string(),
				&PermissionsEnum::CreatePermissions.id(),
			),
			PermissionsEnum::DeletePermissions => make_thing(
				&ResourceEnum::Permissions.to_string(),
				&PermissionsEnum::DeletePermissions.id(),
			),
			PermissionsEnum::UpdatePermissions => make_thing(
				&ResourceEnum::Permissions.to_string(),
				&PermissionsEnum::UpdatePermissions.id(),
			),
			PermissionsEnum::ReadListSessions => make_thing(
				&ResourceEnum::Permissions.to_string(),
				&PermissionsEnum::ReadListSessions.id(),
			),
			PermissionsEnum::ReadDetailSessions => make_thing(
				&ResourceEnum::Permissions.to_string(),
				&PermissionsEnum::ReadDetailSessions.id(),
			),
			PermissionsEnum::CreateSessions => make_thing(
				&ResourceEnum::Permissions.to_string(),
				&PermissionsEnum::CreateSessions.id(),
			),
			PermissionsEnum::UpdateSessions => make_thing(
				&ResourceEnum::Permissions.to_string(),
				&PermissionsEnum::UpdateSessions.id(),
			),
			PermissionsEnum::DeleteSessions => make_thing(
				&ResourceEnum::Permissions.to_string(),
				&PermissionsEnum::DeleteSessions.id(),
			),
			PermissionsEnum::ReadListTests => make_thing(
				&ResourceEnum::Permissions.to_string(),
				&PermissionsEnum::ReadListTests.id(),
			),
			PermissionsEnum::ReadDetailTests => make_thing(
				&ResourceEnum::Permissions.to_string(),
				&PermissionsEnum::ReadDetailTests.id(),
			),
			PermissionsEnum::CreateTests => make_thing(
				&ResourceEnum::Permissions.to_string(),
				&PermissionsEnum::CreateTests.id(),
			),
			PermissionsEnum::UpdateTests => make_thing(
				&ResourceEnum::Permissions.to_string(),
				&PermissionsEnum::UpdateTests.id(),
			),
			PermissionsEnum::DeleteTests => make_thing(
				&ResourceEnum::Permissions.to_string(),
				&PermissionsEnum::DeleteTests.id(),
			),
			PermissionsEnum::ReadListAnswers => make_thing(
				&ResourceEnum::Permissions.to_string(),
				&PermissionsEnum::ReadListAnswers.id(),
			),
			PermissionsEnum::ReadDetailAnswers => make_thing(
				&ResourceEnum::Permissions.to_string(),
				&PermissionsEnum::ReadDetailAnswers.id(),
			),
			PermissionsEnum::CreateAnswers => make_thing(
				&ResourceEnum::Permissions.to_string(),
				&PermissionsEnum::CreateAnswers.id(),
			),
			PermissionsEnum::UpdateAnswers => make_thing(
				&ResourceEnum::Permissions.to_string(),
				&PermissionsEnum::UpdateAnswers.id(),
			),
			PermissionsEnum::DeleteAnswers => make_thing(
				&ResourceEnum::Permissions.to_string(),
				&PermissionsEnum::DeleteAnswers.id(),
			),
			PermissionsEnum::ReadListOptions => make_thing(
				&ResourceEnum::Permissions.to_string(),
				&PermissionsEnum::ReadListOptions.id(),
			),
			PermissionsEnum::ReadDetailOptions => make_thing(
				&ResourceEnum::Permissions.to_string(),
				&PermissionsEnum::ReadDetailOptions.id(),
			),
			PermissionsEnum::CreateOptions => make_thing(
				&ResourceEnum::Permissions.to_string(),
				&PermissionsEnum::CreateOptions.id(),
			),
			PermissionsEnum::UpdateOptions => make_thing(
				&ResourceEnum::Permissions.to_string(),
				&PermissionsEnum::UpdateOptions.id(),
			),
			PermissionsEnum::DeleteOptions => make_thing(
				&ResourceEnum::Permissions.to_string(),
				&PermissionsEnum::DeleteOptions.id(),
			),
			PermissionsEnum::ReadListQuestions => make_thing(
				&ResourceEnum::Permissions.to_string(),
				&PermissionsEnum::ReadListQuestions.id(),
			),
			PermissionsEnum::ReadDetailQuestions => make_thing(
				&ResourceEnum::Permissions.to_string(),
				&PermissionsEnum::ReadDetailQuestions.id(),
			),
			PermissionsEnum::CreateQuestions => make_thing(
				&ResourceEnum::Permissions.to_string(),
				&PermissionsEnum::CreateQuestions.id(),
			),
			PermissionsEnum::UpdateQuestions => make_thing(
				&ResourceEnum::Permissions.to_string(),
				&PermissionsEnum::UpdateQuestions.id(),
			),
			PermissionsEnum::DeleteQuestions => make_thing(
				&ResourceEnum::Permissions.to_string(),
				&PermissionsEnum::DeleteQuestions.id(),
			),
		}
	}
}
