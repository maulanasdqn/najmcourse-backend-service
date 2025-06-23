use super::SessionsRepository;
use super::{SessionsCreateRequestDto, SessionsUpdateRequestDto, TestSessionsDto};
use crate::TestsSchema;
use anyhow::Result;
use najm_course_utils::{create_mock_app_state, get_iso_date, make_thing};
use surrealdb::Uuid;

#[tokio::test]
async fn test_create_and_get_session() -> Result<()> {
	let state = create_mock_app_state().await;
	let db = &state.surrealdb_ws;
	let session_name = format!("{} Mock Test 1", Uuid::new_v4());
	let test_id = format!("{} mock_test_1", Uuid::new_v4())
		.as_str()
		.to_string();
	let test_thing = make_thing("app_tests", &test_id.clone());
	let dummy_test = TestsSchema {
		id: test_thing.clone(),
		name: session_name.clone(),
		banner: None,
		category: "Test".into(),
		questions: Some(vec![]),
		sub_tests: None,
		is_deleted: false,
		created_at: get_iso_date(),
		updated_at: get_iso_date(),
	};
	let _: Option<TestsSchema> = db
		.create(("app_tests", &test_id.clone()))
		.content(dummy_test)
		.await?;
	let repo = SessionsRepository::new(&state);
	let payload = SessionsCreateRequestDto {
		name: "Tryout Test".to_string(),
		banner: None,
		category: "Saintek".to_string(),
		description: "Mock tryout".to_string(),
		is_active: true,
		student_type: "SMA".to_string(),
		tests: vec![TestSessionsDto {
			test_id: test_id.to_string(),
			weight: "25%".to_string(),
			shuffle: true,
			multiplier: 1.2,
			timer: 120,
			start_date: "2025-01-01T00:00:00Z".to_string(),
			end_date: "2025-01-10T00:00:00Z".to_string(),
		}],
	};
	let session_id = repo.query_create_session(payload).await?;
	let detail = repo.query_session_by_id(&session_id).await?;
	assert_eq!(detail.name, "Tryout Test");
	assert_eq!(detail.tests.len(), 1);
	assert_eq!(detail.tests[0].test.name, session_name);
	Ok(())
}

#[tokio::test]
async fn test_update_session() -> Result<()> {
	let state = create_mock_app_state().await;
	let repo = SessionsRepository::new(&state);
	let payload = SessionsCreateRequestDto {
		name: "Tryout Update".to_string(),
		category: "Soshum".to_string(),
		banner: None,
		description: "Update test".to_string(),
		is_active: true,

		student_type: "SMA".to_string(),
		tests: vec![TestSessionsDto {
			test_id: "mock_test_2".to_string(),
			weight: "25%".to_string(),
			shuffle: true,
			multiplier: 1.0,
			timer: 90,
			start_date: "2025-02-01T00:00:00Z".to_string(),
			end_date: "2025-02-10T00:00:00Z".to_string(),
		}],
	};
	let session_id = repo.query_create_session(payload).await?;
	let update_payload = SessionsUpdateRequestDto {
		name: "Updated Name".to_string(),
		category: "Saintek".to_string(),
		banner: None,
		description: "Updated description".to_string(),
		student_type: "SMA".to_string(),
		tests: vec![TestSessionsDto {
			test_id: "mock_test_2".to_string(),
			weight: "25%".to_string(),
			shuffle: true,
			multiplier: 1.5,
			timer: 150,
			start_date: "2025-02-05T00:00:00Z".to_string(),
			end_date: "2025-02-15T00:00:00Z".to_string(),
		}],
		is_active: true,
	};
	let result = repo
		.query_update_session(session_id.clone(), update_payload)
		.await?;
	assert_eq!(result, "Success update session");
	Ok(())
}

#[tokio::test]
async fn test_delete_session() -> Result<()> {
	let state = create_mock_app_state().await;
	let repo = SessionsRepository::new(&state);
	let payload = SessionsCreateRequestDto {
		banner: None,
		name: "To Be Deleted".to_string(),
		category: "Campuran".to_string(),
		is_active: true,
		description: "For deletion test".to_string(),
		student_type: "SMA".to_string(),
		tests: vec![TestSessionsDto {
			test_id: "mock_test_3".to_string(),
			weight: "25%".to_string(),
			multiplier: 0.9,
			shuffle: true,
			timer: 180,
			start_date: "2025-03-01T00:00:00Z".to_string(),
			end_date: "2025-03-10T00:00:00Z".to_string(),
		}],
	};
	let session_id = repo.query_create_session(payload).await?;
	let delete_result = repo.query_delete_session(session_id.clone()).await?;
	assert_eq!(delete_result, "Success delete session");
	Ok(())
}

#[tokio::test]
async fn test_create_session_with_empty_tests_should_fail() {
	let state = create_mock_app_state().await;
	let repo = SessionsRepository::new(&state);
	let payload = SessionsCreateRequestDto {
		banner: None,
		name: "Empty Tests".to_string(),
		category: "Kategori".to_string(),
		description: "No tests".to_string(),
		is_active: true,
		student_type: "SMA".to_string(),
		tests: vec![], // ❌
	};
	let result = repo.query_create_session(payload).await;
	assert!(result.is_err());
	assert!(format!("{:?}", result).contains("Tests must not be empty"));
}

#[tokio::test]
async fn test_update_non_existing_session_should_fail() {
	let state = create_mock_app_state().await;
	let repo = SessionsRepository::new(&state);
	let update_payload = SessionsUpdateRequestDto {
		name: "Should Fail".into(),
		category: "Saintek".into(),
		description: "Update should fail".into(),
		banner: None,
		student_type: "SMA".into(),
		tests: vec![TestSessionsDto {
			test_id: "mock_test_x".into(),
			weight: "25%".into(),
			shuffle: true,
			multiplier: 1.0,
			timer: 60,
			start_date: "2025-01-01T00:00:00Z".into(),
			end_date: "2025-01-10T00:00:00Z".into(),
		}],
		is_active: true,
	};
	let result = repo
		.query_update_session("non_existing_id".into(), update_payload)
		.await;
	assert!(result.is_err());
	assert!(format!("{:?}", result).contains("Session not found"));
}

#[tokio::test]
async fn test_create_session_with_invalid_test_ref_should_fail() {
	let state = create_mock_app_state().await;
	let repo = SessionsRepository::new(&state);
	let payload = SessionsCreateRequestDto {
		name: "Invalid Test Ref".to_string(),
		banner: None,
		category: "Saintek".to_string(),
		is_active: true,
		description: "Non-existing test ref".to_string(),
		student_type: "SMA".to_string(),
		tests: vec![TestSessionsDto {
			test_id: "non_existing_test_id".to_string(),
			weight: "25%".to_string(),
			shuffle: true,
			multiplier: 1.0,
			timer: 100,
			start_date: "2025-01-01T00:00:00Z".to_string(),
			end_date: "2025-01-10T00:00:00Z".to_string(),
		}],
	};
	let session_id = repo.query_create_session(payload).await.unwrap();
	let result = repo.query_session_by_id(&session_id).await;
	assert!(result.is_err());
	assert!(
		result
			.unwrap_err()
			.to_string()
			.contains("failed to deserialize")
	);
}

#[tokio::test]
async fn test_get_session_with_invalid_id_should_fail() {
	let state = create_mock_app_state().await;
	let repo = SessionsRepository::new(&state);
	let result = repo.query_session_by_id("not_exist_123").await;
	assert!(result.is_err());
	assert!(result.unwrap_err().to_string().contains("not found"));
}

#[tokio::test]
async fn test_update_session_with_empty_tests_should_fail() {
	let state = create_mock_app_state().await;
	let repo = SessionsRepository::new(&state);
	let payload = SessionsCreateRequestDto {
		name: "To Update".to_string(),
		category: "Campuran".to_string(),
		banner: None,
		is_active: true,
		description: "To test update fail".to_string(),
		student_type: "SMA".to_string(),
		tests: vec![TestSessionsDto {
			test_id: "mock_test_update".to_string(),
			shuffle: true,
			weight: "25%".to_string(),
			multiplier: 1.0,
			timer: 75,
			start_date: "2025-04-01T00:00:00Z".to_string(),
			end_date: "2025-04-10T00:00:00Z".to_string(),
		}],
	};
	let session_id = repo.query_create_session(payload).await.unwrap();
	let update_payload = SessionsUpdateRequestDto {
		banner: None,
		name: "Failing update".to_string(),
		category: "X".to_string(),
		description: "Should fail".to_string(),
		student_type: "SMA".to_string(),
		tests: vec![], // ❌ kosong
		is_active: true,
	};
	let result = repo
		.query_update_session(session_id.clone(), update_payload)
		.await;
	assert!(result.is_err());
	assert!(
		result
			.unwrap_err()
			.to_string()
			.contains("must not be empty")
	);
}

#[tokio::test]
async fn test_delete_non_existing_session_should_fail() {
	let state = create_mock_app_state().await;
	let repo = SessionsRepository::new(&state);
	let result = repo
		.query_delete_session("invalid_id_123".to_string())
		.await;
	assert!(result.is_err());
	assert!(result.unwrap_err().to_string().contains("not found"));
}

#[tokio::test]
async fn test_delete_session_twice_should_fail() -> Result<()> {
	let state = create_mock_app_state().await;
	let db = &state.surrealdb_ws;
	let test_id = format!("{} mock_test_del_twice", Uuid::new_v4());
	let _: Option<TestsSchema> = db
		.create(("app_tests", test_id.clone()))
		.content(TestsSchema {
			id: make_thing("app_tests", &test_id.clone()),
			name: "Del Twice".to_string(),
			questions: Some(vec![]),
			sub_tests: None,
			category: "Test".to_string(),
			is_deleted: false,
			banner: None,
			created_at: get_iso_date(),
			updated_at: get_iso_date(),
		})
		.await?;
	let repo = SessionsRepository::new(&state);
	let payload = SessionsCreateRequestDto {
		name: "Del Twice".to_string(),
		category: "Test".to_string(),
		banner: None,
		description: "Double delete".to_string(),
		is_active: true,
		student_type: "SMA".to_string(),
		tests: vec![TestSessionsDto {
			test_id: test_id.to_string(),
			shuffle: true,
			weight: "25%".to_string(),
			multiplier: 1.0,
			timer: 75,
			start_date: "2025-05-01T00:00:00Z".to_string(),
			end_date: "2025-05-10T00:00:00Z".to_string(),
		}],
	};
	let session_id = repo.query_create_session(payload).await?;
	let del_1 = repo.query_delete_session(session_id.clone()).await?;
	assert_eq!(del_1, "Success delete session");
	let del_2 = repo.query_delete_session(session_id.clone()).await;
	assert!(del_2.is_err());
	assert!(del_2.unwrap_err().to_string().contains("not found"));

	Ok(())
}
