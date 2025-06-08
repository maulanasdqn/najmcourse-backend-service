use crate::{
	create_mock_app_state,
	v1::{
		options::{OptionsCreateRequestDto, OptionsUpdateRequestDto},
		questions::{
			QuestionsCreateRequestDto, QuestionsRepository, QuestionsUpdateRequestDto,
		},
	},
};
use surrealdb::Uuid;

fn generate_option(label: &str, correct: bool) -> OptionsCreateRequestDto {
	OptionsCreateRequestDto {
		label: Some(label.into()),
		image_url: None,
		is_correct: correct,
		points: Some(10.0),
	}
}

fn generate_question_payload() -> QuestionsCreateRequestDto {
	QuestionsCreateRequestDto {
		question: Some(format!("Question {}", Uuid::new_v4())),
		discussion: Some("This is a discussion".into()),
		question_image_url: None,
		discussion_image_url: None,
		options: vec![
			generate_option("Option A", false),
			generate_option("Option B", true),
		],
	}
}

#[tokio::test]
async fn test_create_question_should_succeed() {
	let state = create_mock_app_state().await;
	let repo = QuestionsRepository::new(&state);
	let payload = generate_question_payload();
	let res = repo.query_create_question(payload).await;
	assert!(res.is_ok());
}

#[tokio::test]
async fn test_get_question_list_should_return_data() {
	let state = create_mock_app_state().await;
	let repo = QuestionsRepository::new(&state);
	let res = repo.query_question_list(Default::default()).await;
	assert!(res.is_ok());
}

#[tokio::test]
async fn test_get_question_by_id_should_return_data() {
	let state = create_mock_app_state().await;
	let repo = QuestionsRepository::new(&state);
	let payload = generate_question_payload();
	let _ = repo.query_create_question(payload.clone()).await.unwrap();
	let all = repo.query_question_list(Default::default()).await.unwrap();
	let latest = all.data.last().expect("Expected at least one question");
	let res = repo.query_question_by_id(&latest.id.clone()).await;
	assert!(res.is_ok());
}

#[tokio::test]
async fn test_update_question_should_succeed() {
	let state = create_mock_app_state().await;
	let repo = QuestionsRepository::new(&state);
	let payload = generate_question_payload();
	let id = repo.query_create_question(payload.clone()).await.unwrap();
	let mut question = None;
	for i in 0..50 {
		if let Ok(q) = repo.query_question_by_id(&id).await {
			if q.options.len() >= 2 {
				question = Some(q);
				break;
			}
		}
		println!("🔁 Retry [{}] – options belum ready", i);
		tokio::time::sleep(std::time::Duration::from_millis(200)).await;
	}
	let question = question.expect("Question not ready with options");
	let update = QuestionsUpdateRequestDto {
		id: question.id.clone(),
		question: Some(format!("Updated {}", payload.question.unwrap_or("".into()))),
		discussion: Some("Updated discussion".into()),
		question_image_url: None,
		discussion_image_url: None,
		options: vec![
			OptionsUpdateRequestDto {
				id: question.options[0].id.clone(),
				label: Some("Updated A".into()),
				image_url: None,
				is_correct: false,
				points: Some(10.0),
			},
			OptionsUpdateRequestDto {
				id: question.options[1].id.clone(),
				label: Some("Updated B".into()),
				image_url: None,
				is_correct: true,
				points: Some(20.2),
			},
		],
	};
	let res = repo
		.query_update_question(question.id.clone(), update)
		.await;
	assert!(res.is_ok());
}

#[tokio::test]
async fn test_delete_question_should_soft_delete() {
	let state = create_mock_app_state().await;
	let repo = QuestionsRepository::new(&state);
	let payload = generate_question_payload();
	let create_result = repo.query_create_question(payload.clone()).await;
	assert!(create_result.is_ok(), "Failed to create question");
	let id = create_result.unwrap();
	let delete_result = repo.query_delete_question(id.clone()).await;
	assert!(
		delete_result.is_ok(),
		"Failed to delete question: {:?}",
		delete_result.unwrap_err()
	);
}

#[tokio::test]
async fn test_delete_question_should_fail_if_already_deleted() {
	let state = create_mock_app_state().await;
	let repo = QuestionsRepository::new(&state);
	let payload = generate_question_payload();
	let _ = repo.query_create_question(payload.clone()).await.unwrap();
	let all = repo.query_question_list(Default::default()).await.unwrap();
	let latest = all.data.last().expect("Expected at least one question");
	let _ = repo.query_delete_question(latest.id.clone()).await.unwrap();
	let res = repo.query_delete_question(latest.id.clone()).await;
	assert!(res.is_err());
}
