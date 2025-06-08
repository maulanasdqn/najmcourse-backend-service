use crate::{
	create_mock_app_state,
	v1::{
		options::{OptionsCreateRequestDto, OptionsUpdateRequestDto},
		questions::{
			questions_router, QuestionsCreateRequestDto, QuestionsRepository,
			QuestionsUpdateRequestDto,
		},
	},
	AppState, PermissionsEnum,
};
use axum::{Extension, Router};
use axum_test::TestServer;
use najm_course_utils::authorized;
use surrealdb::Uuid;

fn create_test_app(state: AppState) -> TestServer {
	let app = Router::new()
		.nest("/v1/questions", questions_router())
		.layer(Extension(state));
	TestServer::new(app).unwrap()
}

fn generate_option(label: &str, correct: bool) -> OptionsCreateRequestDto {
	OptionsCreateRequestDto {
		label: Some(label.into()),
		image_url: None,
		points: Some(10.0),
		is_correct: correct,
	}
}

fn generate_question_payload() -> QuestionsCreateRequestDto {
	QuestionsCreateRequestDto {
		question: Some(format!("Question {}", Uuid::new_v4())),
		discussion: Some("Discussion here".into()),
		question_image_url: None,
		discussion_image_url: None,
		options: vec![generate_option("A", false), generate_option("B", true)],
	}
}

#[tokio::test]
async fn test_post_create_question_should_return_201() {
	let state = create_mock_app_state().await;
	let server = create_test_app(state);
	let payload = generate_question_payload();
	let res = authorized(
		&server,
		"POST",
		"/v1/questions/create",
		vec![&PermissionsEnum::CreateQuestions.to_string()],
		Some(&payload),
	)
	.await;
	assert_eq!(res.status_code(), 201);
}

#[tokio::test]
async fn test_get_question_list_should_return_200() {
	let state = create_mock_app_state().await;
	let server = create_test_app(state);
	let res = authorized::<()>(
		&server,
		"GET",
		"/v1/questions?page=1&per_page=10",
		vec![&PermissionsEnum::ReadListQuestions.to_string()],
		None,
	)
	.await;
	assert_eq!(res.status_code(), 200);
}

#[tokio::test]
async fn test_get_question_detail_should_return_200() {
	let state = create_mock_app_state().await;
	let server = create_test_app(state.clone());
	let payload = generate_question_payload();
	let repo = QuestionsRepository::new(&state);
	let id = repo.query_create_question(payload.clone()).await.unwrap();

	let res = authorized::<()>(
		&server,
		"GET",
		&format!("/v1/questions/detail/{}", id),
		vec![&PermissionsEnum::ReadDetailQuestions.to_string()],
		None,
	)
	.await;
	assert_eq!(res.status_code(), 200);
}

#[tokio::test]
async fn test_put_update_question_should_return_200() {
	let state = create_mock_app_state().await;
	let server = create_test_app(state.clone());
	let payload = generate_question_payload();
	let repo = QuestionsRepository::new(&state);
	let id = repo.query_create_question(payload.clone()).await.unwrap();

	let update = QuestionsUpdateRequestDto {
		id: id.clone(),
		question: Some(format!("Updated {}", payload.question.unwrap_or("".into()))),
		discussion: Some("Updated discussion".into()),
		question_image_url: None,
		discussion_image_url: None,
		options: vec![
			OptionsUpdateRequestDto {
				id: "".into(),
				label: Some("Updated A".into()),
				image_url: None,
				is_correct: false,
				points: Some(10.0),
			},
			OptionsUpdateRequestDto {
				id: "".into(),
				label: Some("Updated B".into()),
				image_url: None,
				is_correct: true,
				points: Some(20.2),
			},
		],
	};

	let res = authorized(
		&server,
		"PUT",
		&format!("/v1/questions/update/{}", id),
		vec![&PermissionsEnum::UpdateQuestions.to_string()],
		Some(&update),
	)
	.await;
	assert_eq!(res.status_code(), 200);
}

#[tokio::test]
async fn test_delete_question_should_return_200() {
	let state = create_mock_app_state().await;
	let server = create_test_app(state.clone());
	let payload = generate_question_payload();
	let repo = QuestionsRepository::new(&state);
	let id = repo.query_create_question(payload).await.unwrap();

	let res = authorized::<()>(
		&server,
		"DELETE",
		&format!("/v1/questions/delete/{}", id),
		vec![&PermissionsEnum::DeleteQuestions.to_string()],
		None,
	)
	.await;
	assert_eq!(res.status_code(), 200);
}

#[tokio::test]
async fn test_create_question_should_fail_if_empty_question() {
	let state = create_mock_app_state().await;
	let server = create_test_app(state);
	let res = authorized(
		&server,
		"POST",
		"/v1/questions/create",
		vec![&PermissionsEnum::CreateQuestions.to_string()],
		Some(&QuestionsCreateRequestDto {
			question: Some("".into()),
			discussion: Some("Valid".into()),
			question_image_url: None,
			discussion_image_url: None,
			options: vec![],
		}),
	)
	.await;
	assert_eq!(res.status_code(), 400);
}

#[tokio::test]
async fn test_create_question_should_fail_if_empty_discussion() {
	let state = create_mock_app_state().await;
	let server = create_test_app(state);
	let res = authorized(
		&server,
		"POST",
		"/v1/questions/create",
		vec![&PermissionsEnum::CreateQuestions.to_string()],
		Some(&QuestionsCreateRequestDto {
			question: Some("Valid".into()),
			discussion: Some("".into()),
			question_image_url: None,
			discussion_image_url: None,
			options: vec![],
		}),
	)
	.await;
	assert_eq!(res.status_code(), 400);
}

#[tokio::test]
async fn test_create_question_should_fail_if_empty_options() {
	let state = create_mock_app_state().await;
	let server = create_test_app(state);
	let res = authorized(
		&server,
		"POST",
		"/v1/questions/create",
		vec![&PermissionsEnum::CreateQuestions.to_string()],
		Some(&QuestionsCreateRequestDto {
			question: Some("Valid".into()),
			discussion: Some("Valid".into()),
			question_image_url: None,
			discussion_image_url: None,
			options: vec![],
		}),
	)
	.await;
	assert_eq!(res.status_code(), 400);
}

#[tokio::test]
async fn test_update_question_should_fail_if_not_found() {
	let state = create_mock_app_state().await;
	let server = create_test_app(state);
	let update = QuestionsUpdateRequestDto {
		id: "non-existent-id".into(),
		question: Some("Updated".into()),
		discussion: Some("Updated".into()),
		question_image_url: None,
		discussion_image_url: None,
		options: vec![OptionsUpdateRequestDto {
			id: "".into(),
			label: Some("Updated A".into()),
			image_url: None,
			is_correct: false,
			points: Some(10.0),
		}],
	};
	let res = authorized(
		&server,
		"PUT",
		"/v1/questions/update/non-existent-id",
		vec![&PermissionsEnum::UpdateQuestions.to_string()],
		Some(&update),
	)
	.await;
	assert_eq!(res.status_code(), 400);
}

#[tokio::test]
async fn test_get_question_detail_should_return_404_if_not_found() {
	let state = create_mock_app_state().await;
	let server = create_test_app(state);
	let res = authorized::<()>(
		&server,
		"GET",
		"/v1/questions/detail/non-existent-id",
		vec![&PermissionsEnum::ReadDetailQuestions.to_string()],
		None,
	)
	.await;
	assert_eq!(res.status_code(), 404);
}

#[tokio::test]
async fn test_delete_question_should_return_404_if_not_found() {
	let state = create_mock_app_state().await;
	let server = create_test_app(state);
	let res = authorized::<()>(
		&server,
		"DELETE",
		"/v1/questions/delete/non-existent-id",
		vec![&PermissionsEnum::DeleteQuestions.to_string()],
		None,
	)
	.await;
	assert_eq!(res.status_code(), 404);
}
