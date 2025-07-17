use najm_iam::{
admin_stats, auth, permissions, roles, users, AdminDashboardStatsResponseDto, AuthLoginRequestDto, AuthLoginResponsetDto, AuthNewPasswordRequestDto, AuthRefreshTokenRequestDto, AuthResendOtpRequestDto, AuthVerifyEmailRequestDto, MessageResponseDto, MetaRequestDto, MetaResponseDto, PermissionsItemDto, PermissionsRequestDto, ResponseListSuccessDto, ResponseSuccessDto, RolesDetailItemDto, RolesListItemDto, RolesRequestCreateDto, RolesRequestUpdateDto, TokenDto, UsersCreateRequestDto, UsersDetailItemDto, UsersListItemDto, UsersUpdateRequestDto
};
use utoipa::{
	openapi::security::{Http, HttpAuthScheme, SecurityScheme},
	Modify, OpenApi,
};
use najm_cms::{events_controller, events_dto::{EventsDetailItemDto, EventsListItemDto}};
use najm_exam::{
	options, questions, sessions, student_stats, tests,
	OptionsCreateRequestDto, OptionsUpdateRequestDto, OptionsItemDto,
	QuestionsCreateRequestDto, QuestionsUpdateRequestDto, QuestionsItemDto,
	SessionsCreateRequestDto, SessionsUpdateRequestDto, SessionsDetailResponseDto, SessionsResponseDto,
	StudentDashboardResponseDto, TestsCreateRequestDto, TestsUpdateRequestDto, TestsItemDto, TestsResponseListDto,
	TestSessionsDto, TestSessionsItemDto
};


#[derive(OpenApi)]
#[openapi(
    paths(
     auth::auth_controller::post_login,
     auth::auth_controller::post_register,
     auth::auth_controller::post_verify_email,
     auth::auth_controller::post_resend_otp,
     auth::auth_controller::post_refresh_token,
     auth::auth_controller::post_forgot_password,
     auth::auth_controller::post_new_password,
     users::users_controller::post_create_user,
     users::users_controller::put_update_user,
     users::users_controller::put_update_user_me,
     users::users_controller::patch_user_active_status,
     users::users_controller::delete_user,
     users::users_controller::get_user_by_id,
     users::users_controller::get_user_me,
     users::users_controller::get_user_list,
     roles::roles_controller::get_role_list,
     roles::roles_controller::get_role_by_id,
     roles::roles_controller::post_create_role,
     roles::roles_controller::put_update_role,
     roles::roles_controller::delete_role,
     permissions::permissions_controller::get_permission_list,
     permissions::permissions_controller::get_permission_by_id,
     permissions::permissions_controller::post_create_permission,
     permissions::permissions_controller::put_update_permission,
     permissions::permissions_controller::delete_permission,
	 events_controller::get_event_list,
	 events_controller::get_event_by_id,
	 events_controller::post_create_event,
	 events_controller::patch_update_event,
	 events_controller::delete_event,
	 options::options_controller::get_option_list,
	 options::options_controller::get_option_by_id,
	 options::options_controller::post_create_option,
	 options::options_controller::put_update_option,
	 options::options_controller::delete_option,
	 questions::questions_controller::get_question_list,
	 questions::questions_controller::get_question_by_id,
	 questions::questions_controller::post_create_question,
	 questions::questions_controller::put_update_question,
	 questions::questions_controller::delete_question,
	 tests::tests_controller::get_test_list,
	 tests::tests_controller::get_test_by_id,
	 tests::tests_controller::post_create_test,
	 tests::tests_controller::put_update_test,
	 tests::tests_controller::delete_test,
	 sessions::sessions_controller::get_session_list,
	 sessions::sessions_controller::get_session_by_id,
	 sessions::sessions_controller::post_create_session,
	 sessions::sessions_controller::put_update_session,
	 sessions::sessions_controller::delete_session,
	 sessions::sessions_controller::get_student_stats,
	 student_stats::student_stats_controller::get_student_dashboard,
	 admin_stats::admin_stats_controller::get_admin_dashboard_stats,
    ),
    components(
        schemas(
           MetaRequestDto,
           MetaResponseDto,
           MessageResponseDto,
           AuthLoginRequestDto,
           AuthLoginResponsetDto,
           AuthVerifyEmailRequestDto,
           AuthResendOtpRequestDto,
           AuthNewPasswordRequestDto,
           AuthRefreshTokenRequestDto,
           ResponseSuccessDto<TokenDto>,
           RolesListItemDto,
           RolesRequestCreateDto, 
           RolesRequestUpdateDto,
           PermissionsRequestDto,
           PermissionsItemDto,
           UsersDetailItemDto,
           UsersListItemDto,
           UsersUpdateRequestDto,
           UsersCreateRequestDto,
           ResponseSuccessDto<AuthLoginResponsetDto>,
           ResponseListSuccessDto<Vec<RolesListItemDto>>,
           ResponseSuccessDto<RolesDetailItemDto>,
           ResponseListSuccessDto<Vec<UsersListItemDto>>,
           ResponseSuccessDto<UsersDetailItemDto>,
           ResponseListSuccessDto<Vec<PermissionsItemDto>>,
           ResponseSuccessDto<PermissionsItemDto>,
		   ResponseListSuccessDto<Vec<EventsListItemDto>>,
		   ResponseSuccessDto<EventsDetailItemDto>,
		   OptionsCreateRequestDto,
		   OptionsUpdateRequestDto,
		   OptionsItemDto,
		   ResponseSuccessDto<OptionsItemDto>,
		   ResponseListSuccessDto<Vec<OptionsItemDto>>,
		   QuestionsCreateRequestDto,
		   QuestionsUpdateRequestDto,
		   QuestionsItemDto,
		   ResponseSuccessDto<QuestionsItemDto>,
		   ResponseListSuccessDto<Vec<QuestionsItemDto>>,
		   TestsCreateRequestDto,
		   TestsUpdateRequestDto,
		   TestsItemDto,
		   TestsResponseListDto,
		   ResponseSuccessDto<TestsItemDto>,
		   ResponseListSuccessDto<Vec<TestsResponseListDto>>,
		   SessionsCreateRequestDto,
		   SessionsUpdateRequestDto,
		   SessionsDetailResponseDto,
		   SessionsResponseDto,
		   TestSessionsDto,
		   TestSessionsItemDto,
		   ResponseSuccessDto<SessionsDetailResponseDto>,
		   ResponseListSuccessDto<Vec<SessionsResponseDto>>,
		   StudentDashboardResponseDto,
		   ResponseSuccessDto<StudentDashboardResponseDto>,
		   AdminDashboardStatsResponseDto,
		   ResponseSuccessDto<AdminDashboardStatsResponseDto>,
		   MessageResponseDto,
        )
    ),
    info(
        title = "NAJM Course Backend Service",
        description = "NAJM Course Backend Service Provide API For Computer Assisted Test and Backoffice Web App",
        version = "0.1.0",
        contact(
            name = "Maulana Sodiqin",
            url = ""
        ),
        license(
            name = "MIT",
            url = "https://opensource.org/licenses/MIT"
        )
    ),
    modifiers(&SecurityAddon),
    tags(
        (name = "Authentication", description = "List of Authentication Endpoints"),
        (name = "Users", description = "User Management Endpoints"),
        (name = "Roles", description = "Role Management Endpoints"),
        (name = "Permissions", description = "Permission Management Endpoints"),
        (name = "Events", description = "Event Management Endpoints"),
        (name = "Options", description = "Option Management Endpoints"),
        (name = "Questions", description = "Question Management Endpoints"),
        (name = "Tests", description = "Test Management Endpoints"),
        (name = "Sessions", description = "Session Management Endpoints"),
        (name = "Student Stats", description = "Student Dashboard Statistics Endpoints"),
        (name = "Admin Stats", description = "Admin Dashboard Statistics Endpoints"),
    )
)]

pub struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
	fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
		if let Some(components) = openapi.components.as_mut() {
			components.add_security_scheme(
				"Bearer",
				SecurityScheme::Http(Http::new(HttpAuthScheme::Bearer)),
			);
		}
	}
}

pub fn docs_router() -> utoipa::openapi::OpenApi {
	ApiDoc::openapi()
}
