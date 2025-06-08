use crate::{
	create_mock_app_state,
	v1::{
		options::OptionsCreateRequestDto,
		questions::QuestionsCreateRequestDto,
		tests::{tests_router, TestsCreateRequestDto, TestsRepository},
	},
	AppState, PermissionsEnum, TestsResponseListDto,
};
use axum::{Extension, Router};
use axum_test::TestServer;
use najm_course_entities::ResponseListSuccessDto;
use najm_course_utils::authorized;
use surrealdb::Uuid;

fn create_test_app(state: AppState) -> TestServer {
	let app = Router::new()
		.nest("/v1/tests", tests_router())
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
		discussion: Some("Discuss here".into()),
		question_image_url: None,
		discussion_image_url: None,
		options: vec![generate_option("A", false), generate_option("B", true)],
	}
}

fn generate_test_payload(name: &str) -> TestsCreateRequestDto {
	TestsCreateRequestDto {
		name: name.to_string(),
		category: "Test".into(),
		banner: None,
		questions: Some(vec![generate_question_payload()]),
		sub_tests: None,
	}
}

#[tokio::test]
async fn test_post_create_test_should_return_201() {
	let state = create_mock_app_state().await;
	let server = create_test_app(state);
	let payload = generate_test_payload("Test Create");
	let res = authorized(
		&server,
		"POST",
		"/v1/tests/create",
		vec![&PermissionsEnum::CreateTests.to_string()],
		Some(&payload),
	)
	.await;
	assert_eq!(res.status_code(), 201);
}

#[tokio::test]
async fn test_get_test_list_should_return_200() {
	let state = create_mock_app_state().await;
	let server = create_test_app(state);
	let res = authorized::<()>(
		&server,
		"GET",
		"/v1/tests?page=1&per_page=10",
		vec![&PermissionsEnum::ReadListTests.to_string()],
		None,
	)
	.await;
	assert_eq!(res.status_code(), 200);
}

#[tokio::test]
async fn test_get_test_detail_should_return_200() {
	let state = create_mock_app_state().await;
	let server = create_test_app(state.clone());
	let payload = generate_test_payload("Test Detail");
	let repo = TestsRepository::new(&state);
	let _ = repo.query_create_test(payload.clone()).await.unwrap();
	let mut test = None;
	for i in 0..20 {
		if let Ok(t) = repo.query_test_by_name(&payload.name).await {
			test = Some(t);
			break;
		}
		println!(
			"🔁 Retry [{}] – Belum ketemu test: {}, total_data={}",
			i, &payload.name, 10.0
		);
		tokio::time::sleep(std::time::Duration::from_millis(300)).await;
	}
	let test = test.expect("Test not found before get by ID");
	let res = authorized::<()>(
		&server,
		"GET",
		&format!("/v1/tests/detail/{}", test.id),
		vec![&PermissionsEnum::ReadDetailTests.to_string()],
		None,
	)
	.await;
	assert_eq!(res.status_code(), 200);
}

#[tokio::test]
async fn test_delete_test_should_return_200() {
	let state = create_mock_app_state().await;
	let server = create_test_app(state.clone());
	let payload = generate_test_payload("Test Delete");
	let repo = TestsRepository::new(&state);
	let _ = repo.query_create_test(payload.clone()).await.unwrap();
	let mut test = None;
	for i in 0..20 {
		if let Ok(t) = repo.query_test_by_name(&payload.name).await {
			test = Some(t);
			break;
		}
		println!(
			"🔁 Retry [{}] – Belum ketemu test: {}, total_data={}",
			i, &payload.name, 10.0
		);
		tokio::time::sleep(std::time::Duration::from_millis(300)).await;
	}
	let test = test.expect("Test not found before deletion");
	let res = authorized::<()>(
		&server,
		"DELETE",
		&format!("/v1/tests/delete/{}", test.id),
		vec![&PermissionsEnum::DeleteTests.to_string()],
		None,
	)
	.await;
	assert_eq!(res.status_code(), 200);
}

#[tokio::test]
async fn test_create_test_should_fail_if_no_questions() {
	let state = create_mock_app_state().await;
	let server = create_test_app(state);
	let payload = TestsCreateRequestDto {
		name: "Test Without Questions".into(),
		banner: None,
		category: "Test".into(),
		questions: Some(vec![]),
		sub_tests: None,
	};
	let res = authorized(
		&server,
		"POST",
		"/v1/tests/create",
		vec![&PermissionsEnum::CreateTests.to_string()],
		Some(&payload),
	)
	.await;
	assert_eq!(res.status_code(), 400);
}

#[tokio::test]
async fn test_delete_test_should_fail_if_already_deleted() {
	let state = create_mock_app_state().await;
	let server = create_test_app(state.clone());
	let payload = generate_test_payload("Delete Twice Controller Test");
	let repo = TestsRepository::new(&state);
	let _ = repo.query_create_test(payload.clone()).await.unwrap();
	let mut test = None;
	for _ in 0..20 {
		if let Ok(t) = repo.query_test_by_name(&payload.name).await {
			test = Some(t);
			break;
		}
		tokio::time::sleep(std::time::Duration::from_millis(300)).await;
	}
	let test = test.expect("Test not found before deletion");
	let _ = authorized::<()>(
		&server,
		"DELETE",
		&format!("/v1/tests/delete/{}", test.id),
		vec![&PermissionsEnum::DeleteTests.to_string()],
		None,
	)
	.await;
	tokio::time::sleep(std::time::Duration::from_millis(300)).await;
	let res = authorized::<()>(
		&server,
		"DELETE",
		&format!("/v1/tests/delete/{}", test.id),
		vec![&PermissionsEnum::DeleteTests.to_string()],
		None,
	)
	.await;
	assert_eq!(res.status_code(), 400);
}

#[tokio::test]
async fn test_get_test_detail_should_return_404_if_not_found() {
	let state = create_mock_app_state().await;
	let server = create_test_app(state);
	let res = authorized::<()>(
		&server,
		"GET",
		"/v1/tests/detail/non-existent-id",
		vec![&PermissionsEnum::ReadDetailTests.to_string()],
		None,
	)
	.await;
	assert_eq!(res.status_code(), 404);
}

#[tokio::test]
async fn test_get_test_list_should_respect_pagination() {
	let state = create_mock_app_state().await;
	let server = create_test_app(state.clone());
	let repo = TestsRepository::new(&state);
	for i in 0..15 {
		let name = format!("Paginated Test {}", i);
		let _ = repo.query_create_test(generate_test_payload(&name)).await;
	}
	let res = authorized::<()>(
		&server,
		"GET",
		"/v1/tests?page=2&per_page=5",
		vec![&PermissionsEnum::ReadListTests.to_string()],
		None,
	)
	.await;
	assert_eq!(res.status_code(), 200);
	let json = res.json::<ResponseListSuccessDto<Vec<TestsResponseListDto>>>();
	assert_eq!(json.data.len(), 5);
}

#[tokio::test]
async fn test_update_test_should_return_200() {
	let state = create_mock_app_state().await;
	let server = create_test_app(state.clone());
	let payload = generate_test_payload("Update Me Test");
	let repo = TestsRepository::new(&state);
	let test_id = repo.query_create_test(payload.clone()).await.unwrap();
	let mut test = None;
	for i in 0..50 {
		if let Ok(data) = repo.query_test_by_id(&test_id).await {
			let ready = data.questions.as_ref().map_or(false, |questions| {
				!questions.is_empty() && questions.iter().all(|q| !q.options.is_empty())
			});
			if ready {
				test = Some(data);
				break;
			}
		}
		println!("🔁 Retry [{}] – Questions belum ready dengan options", i);
		tokio::time::sleep(std::time::Duration::from_millis(200)).await;
	}
	let test = test.expect("Test detail not found with questions & options populated");
	let questions = test.questions.as_ref().unwrap();
	let update = serde_json::json!({
		"name": "Updated Test Name",
		"category": "Test",
		"banner": null,
		"questions": [{
			"id": questions[0].id,
			"question": "Updated Question",
			"discussion": "Updated Discussion",
			"question_image_url": null,
			"discussion_image_url": null,
			"options": [
				{
					"id": questions[0].options[0].id,
					"label": "Updated Option A",
					"image_url": null,
					"is_correct": false
				},
				{
					"id": questions[0].options[1].id,
					"label": "Updated Option B",
					"image_url": null,
					"is_correct": true
				}
			]
		}]
	});
	let res = authorized(
		&server,
		"PUT",
		&format!("/v1/tests/update/{}", test_id),
		vec![&PermissionsEnum::UpdateTests.to_string()],
		Some(&update),
	)
	.await;
	assert_eq!(res.status_code(), 200);
}

#[tokio::test]
async fn test_update_test_should_fail_if_test_is_deleted() {
	let state = create_mock_app_state().await;
	let server = create_test_app(state.clone());
	let repo = TestsRepository::new(&state);
	let payload = generate_test_payload("To Be Deleted");
	let test_id = repo.query_create_test(payload.clone()).await.unwrap();
	let _ = repo.query_delete_test(test_id.clone()).await.unwrap();
	let dummy_update = serde_json::json!({
		"name": "Should Fail",
		"category": "Test",
		"banner": null,
		"questions": [],
	});
	let res = authorized(
		&server,
		"PUT",
		&format!("/v1/tests/update/{}", test_id),
		vec![&PermissionsEnum::UpdateTests.to_string()],
		Some(&dummy_update),
	)
	.await;
	assert_eq!(res.status_code(), 404);
}

#[tokio::test]
async fn test_update_test_should_fail_if_missing_question_id() {
	let state = create_mock_app_state().await;
	let server = create_test_app(state.clone());
	let repo = TestsRepository::new(&state);
	let payload = generate_test_payload("Missing Question ID");
	let test_id = repo.query_create_test(payload.clone()).await.unwrap();
	let mut full_test = None;
	for _ in 0..20 {
		if let Ok(t) = repo.query_test_by_id(&test_id).await {
			let ready = t.questions.as_ref().map_or(false, |questions| {
				!questions.is_empty() && questions.iter().all(|q| !q.options.is_empty())
			});
			if ready {
				full_test = Some(t);
				break;
			}
		}
		tokio::time::sleep(std::time::Duration::from_millis(300)).await;
	}
	let test = full_test.expect("Test not found with questions");
	let questions = test.questions.as_ref().unwrap();
	let update = serde_json::json!({
		"name": "Invalid Update",
		"category": "Test",
		"banner": null,
		"questions": [{
			"question": "No ID here",
			"discussion": "Missing ID field",
			"question_image_url": null,
			"discussion_image_url": null,
			"options": [{
				"id": questions[0].options[0].id,
				"label": "Option A",
				"image_url": null,
				"is_correct": false
			}]
		}]
	});
	let res = authorized(
		&server,
		"PUT",
		&format!("/v1/tests/update/{}", test_id),
		vec![&PermissionsEnum::UpdateTests.to_string()],
		Some(&update),
	)
	.await;
	assert_eq!(res.status_code(), 422);
}

#[tokio::test]
async fn test_create_test_should_fail_if_question_has_no_option() {
	let state = create_mock_app_state().await;
	let server = create_test_app(state);
	let mut question = generate_question_payload();
	question.options.clear();
	let payload = TestsCreateRequestDto {
		name: "No Option Question".into(),
		banner: None,
		category: "Test".into(),
		questions: Some(vec![question]),
		sub_tests: None,
	};
	let res = authorized(
		&server,
		"POST",
		"/v1/tests/create",
		vec![&PermissionsEnum::CreateTests.to_string()],
		Some(&payload),
	)
	.await;
	assert_eq!(res.status_code(), 400);
}

#[tokio::test]
async fn test_update_test_should_fail_if_option_missing_id() {
	let state = create_mock_app_state().await;
	let server = create_test_app(state.clone());
	let repo = TestsRepository::new(&state);
	let payload = generate_test_payload("Option Missing ID");
	let test_id = repo.query_create_test(payload.clone()).await.unwrap();
	let mut test = None;
	for _ in 0..20 {
		if let Ok(t) = repo.query_test_by_id(&test_id).await {
			let ready = t.questions.as_ref().map_or(false, |questions| {
				!questions.is_empty() && questions.iter().all(|q| !q.options.is_empty())
			});
			if ready {
				test = Some(t);
				break;
			}
		}
		tokio::time::sleep(std::time::Duration::from_millis(300)).await;
	}
	let test = test.expect("Test not found");
	let questions = test.questions.as_ref().unwrap();
	let update = serde_json::json!({
		"name": "Update With Missing Option ID",
		"category": "Test",
		"banner": null,
		"questions": [{
			"id": questions[0].id,
			"question": "Valid",
			"discussion": "But one option missing id",
			"question_image_url": null,
			"discussion_image_url": null,
			"options": [{
				"label": "Option without ID",
				"image_url": null,
				"is_correct": false
			}]
		}]
	});
	let res = authorized(
		&server,
		"PUT",
		&format!("/v1/tests/update/{}", test_id),
		vec![&PermissionsEnum::UpdateTests.to_string()],
		Some(&update),
	)
	.await;
	assert_eq!(res.status_code(), 422);
}
