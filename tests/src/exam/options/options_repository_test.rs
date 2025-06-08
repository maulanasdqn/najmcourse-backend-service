use super::*;
use crate::create_mock_app_state;
use najm_course_entities::MetaRequestDto;
use surrealdb::Uuid;

fn generate_payload() -> OptionsCreateRequestDto {
	OptionsCreateRequestDto {
		label: Some(format!("Option {}", Uuid::new_v4())),
		image_url: Some("https://example.com/image.png".into()),
		is_correct: true,
		points: Some(10.0),
	}
}

#[tokio::test]
async fn test_query_create_option_should_succeed() {
	let state = create_mock_app_state().await;
	let repo = OptionsRepository::new(&state);
	let payload = generate_payload();
	let result = repo.query_create_option(payload).await;
	assert!(result.is_ok());
}

#[tokio::test]
async fn test_query_raw_option_by_id_should_return_valid_data() {
	let state = create_mock_app_state().await;
	let repo = OptionsRepository::new(&state);
	let payload = generate_payload();
	let _ = repo.query_create_option(payload.clone()).await.unwrap();
	let item = repo
		.query_option_by_label(payload.label.clone())
		.await
		.unwrap();
	let result = repo.query_raw_option_by_id(&item.id).await;
	assert!(result.is_ok());
}

#[tokio::test]
async fn test_query_raw_option_by_id_should_fail_if_not_found() {
	let state = create_mock_app_state().await;
	let repo = OptionsRepository::new(&state);
	let result = repo.query_raw_option_by_id("nonexistent-id").await;
	assert!(result.is_err());
}

#[tokio::test]
async fn test_query_option_by_label_should_return_data() {
	let state = create_mock_app_state().await;
	let repo = OptionsRepository::new(&state);
	let payload = generate_payload();
	let _ = repo.query_create_option(payload.clone()).await.unwrap();
	let result = repo.query_option_by_label(payload.label.clone()).await;
	assert!(result.is_ok());
}

#[tokio::test]
async fn test_query_option_by_label_should_fail_if_not_found() {
	let state = create_mock_app_state().await;
	let repo = OptionsRepository::new(&state);
	let result = repo
		.query_option_by_label(Some("not-exist-label".into()))
		.await;
	assert!(result.is_err());
}

#[tokio::test]
async fn test_query_option_list_should_return_data_with_meta() {
	let state = create_mock_app_state().await;
	let repo = OptionsRepository::new(&state);
	let result = repo.query_option_list(Default::default()).await;
	assert!(result.is_ok());
}

#[tokio::test]
async fn test_query_option_by_id_should_return_item_dto() {
	let state = create_mock_app_state().await;
	let repo = OptionsRepository::new(&state);
	let payload = generate_payload();
	let _ = repo.query_create_option(payload.clone()).await.unwrap();
	let item = repo
		.query_option_by_label(payload.label.clone())
		.await
		.unwrap();
	let result = repo.query_option_by_id(item.id.clone()).await;
	assert!(result.is_ok());
}

#[tokio::test]
async fn test_query_update_option_should_update_existing_data() {
	let state = create_mock_app_state().await;
	let repo = OptionsRepository::new(&state);
	let payload = generate_payload();
	let _ = repo.query_create_option(payload.clone()).await.unwrap();
	let item = repo
		.query_option_by_label(payload.label.clone())
		.await
		.unwrap();
	let update = OptionsUpdateRequestDto {
		id: "ignored".into(),
		label: Some(format!("Updated {}", payload.label.unwrap_or("".into()))),
		image_url: Some("https://example.com/new.png".into()),
		is_correct: false,
		points: Some(20.2),
	};
	let result = repo.query_update_option(item.id.clone(), update).await;
	assert!(result.is_ok());
}

#[tokio::test]
async fn test_query_update_option_should_fail_if_deleted() {
	let state = create_mock_app_state().await;
	let repo = OptionsRepository::new(&state);
	let payload = generate_payload();
	let _ = repo.query_create_option(payload.clone()).await.unwrap();
	let item = repo
		.query_option_by_label(payload.label.clone())
		.await
		.unwrap();
	let _ = repo.query_delete_option(item.id.clone()).await.unwrap();
	let update = OptionsUpdateRequestDto {
		id: item.id.clone(),
		label: Some(format!("Updated {}", payload.label.unwrap_or("".into()))),
		image_url: Some("https://example.com/new.png".into()),
		is_correct: false,
		points: Some(20.2),
	};
	let result = repo.query_update_option(item.id.clone(), update).await;
	assert!(result.is_err(), "Expected update to fail on deleted option");
}

#[tokio::test]
async fn test_query_delete_option_should_soft_delete() {
	let state = create_mock_app_state().await;
	let repo = OptionsRepository::new(&state);
	let payload = generate_payload();
	let _ = repo.query_create_option(payload.clone()).await.unwrap();
	let item = repo
		.query_option_by_label(payload.label.clone())
		.await
		.unwrap();
	let result = repo.query_delete_option(item.id.clone()).await;
	assert!(result.is_ok());
}

#[tokio::test]
async fn test_query_delete_option_should_fail_if_already_deleted() {
	let state = create_mock_app_state().await;
	let repo = OptionsRepository::new(&state);
	let payload = generate_payload();
	let _ = repo.query_create_option(payload.clone()).await.unwrap();
	let item = repo.query_option_by_label(payload.label.clone()).await;
	assert!(item.is_ok(), "Failed to find option after create");
	let item = item.unwrap();
	let _ = repo.query_delete_option(item.id.clone()).await.unwrap();
	let result = repo.query_delete_option(item.id.clone()).await;
	assert!(result.is_err(), "Should fail deleting already deleted item");
}

#[tokio::test]
async fn test_query_option_by_id_should_fail_with_invalid_format() {
	let state = create_mock_app_state().await;
	let repo = OptionsRepository::new(&state);
	let result = repo.query_option_by_id("!!!invalid-id".into()).await;
	assert!(result.is_err(), "Expected failure on invalid ID format");
}

#[tokio::test]
async fn test_query_create_option_should_allow_empty_image_url() {
	let state = create_mock_app_state().await;
	let repo = OptionsRepository::new(&state);
	let payload = OptionsCreateRequestDto {
		label: Some(format!("Option {}", Uuid::new_v4())),
		image_url: None,
		is_correct: true,
		points: Some(10.0),
	};
	let result = repo.query_create_option(payload).await;
	assert!(result.is_ok(), "Should allow creation without image_url");
}

#[tokio::test]
async fn test_query_update_option_should_fail_if_id_not_found() {
	let state = create_mock_app_state().await;
	let repo = OptionsRepository::new(&state);
	let update = OptionsUpdateRequestDto {
		id: "non-existent-id".into(),
		label: Some("Should fail".into()),
		image_url: None,
		is_correct: false,
		points: Some(20.2),
	};
	let result = repo
		.query_update_option("non-existent-id".into(), update)
		.await;
	assert!(
		result.is_err(),
		"Expected error updating nonexistent option"
	);
}

#[tokio::test]
async fn test_query_option_list_should_respect_search_filter() {
	let state = create_mock_app_state().await;
	let repo = OptionsRepository::new(&state);
	let label = format!("UniqueSearchOption-{}", Uuid::new_v4());
	let _ = repo
		.query_create_option(OptionsCreateRequestDto {
			label: Some(label.clone()),
			image_url: None,
			is_correct: true,
			points: Some(10.2),
		})
		.await
		.unwrap();
	tokio::time::sleep(std::time::Duration::from_millis(100)).await;
	let meta = MetaRequestDto {
		page: Some(1),
		per_page: Some(10),
		search: Some(label.to_lowercase()),
		filter_by: None,
		filter: None,
		sort_by: None,
		order: None,
	};
	let result = repo.query_option_list(meta).await.unwrap();
	if result.data.iter().all(|d| d.label != label) {
		dbg!(&result.data);
	}
	let found = result.data.iter().any(|d| d.label == label);
	assert!(found, "Expected to find option with searched label");
}
