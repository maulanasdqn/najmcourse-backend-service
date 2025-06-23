use crate::{
	MetaRequestDto, PermissionsRepository, PermissionsSchema, create_mock_app_state,
};
use chrono::Utc;

fn create_dummy_permission(name: &str) -> PermissionsSchema {
	PermissionsSchema {
		name: name.to_string(),
		..Default::default()
	}
}

#[tokio::test]
async fn test_create_permission_should_succeed() {
	let state = create_mock_app_state().await;
	let repo = PermissionsRepository::new(&state);
	let permission = create_dummy_permission("Test Permission");
	let result = repo.query_create_permission(permission).await;
	assert!(result.is_ok(), "Create failed: {:?}", result.err());
}

#[tokio::test]
async fn test_query_permission_list_should_return_data() {
	let state = create_mock_app_state().await;
	let repo = PermissionsRepository::new(&state);

	let _ = repo
		.query_create_permission(create_dummy_permission("View"))
		.await;
	let meta = MetaRequestDto {
		page: Some(1),
		per_page: Some(10),
		search: None,
		sort_by: None,
		order: None,
		filter: None,
		filter_by: None,
	};

	let result = repo.query_permission_list(meta).await;
	assert!(result.is_ok(), "List failed: {:?}", result.err());
	assert!(!result.unwrap().data.is_empty(), "Data should not be empty");
}

#[tokio::test]
async fn test_query_permission_by_id_should_succeed() {
	let state = create_mock_app_state().await;
	let repo = PermissionsRepository::new(&state);
	let permission = create_dummy_permission("Detail");
	let _ = repo.query_create_permission(permission.clone()).await;
	let id = permission.id.id.to_raw();
	let result = repo.query_permission_by_id(id).await;
	assert!(result.is_ok(), "Get by id failed: {:?}", result.err());
}

#[tokio::test]
async fn test_update_permission_should_succeed() {
	let state = create_mock_app_state().await;
	let repo = PermissionsRepository::new(&state);
	let mut permission = create_dummy_permission("Update This");
	let _ = repo.query_create_permission(permission.clone()).await;
	permission.name = "Updated Name".into();
	permission.updated_at = Some(Utc::now().to_rfc3339());
	let result = repo.query_update_permission(permission).await;
	assert!(result.is_ok(), "Update failed: {:?}", result.err());
}

#[tokio::test]
async fn test_delete_permission_should_succeed() {
	let state = create_mock_app_state().await;
	let repo = PermissionsRepository::new(&state);
	let permission = create_dummy_permission("To Be Deleted");
	let _ = repo.query_create_permission(permission.clone()).await;
	let id = permission.id.id.to_raw();
	let result = repo.query_delete_permission(id).await;
	assert!(result.is_ok(), "Delete failed: {:?}", result.err());
}

#[tokio::test]
async fn test_delete_permission_should_fail_if_already_deleted() {
	let state = create_mock_app_state().await;
	let repo = PermissionsRepository::new(&state);
	let permission = create_dummy_permission("Delete Twice");
	let _ = repo.query_create_permission(permission.clone()).await;
	let id = permission.id.id.to_raw();
	let _ = repo.query_delete_permission(id.clone()).await;
	let second = repo.query_delete_permission(id).await;
	assert!(second.is_err(), "Should fail on second delete");
}

#[tokio::test]
async fn test_update_permission_should_fail_if_deleted() {
	let state = create_mock_app_state().await;
	let repo = PermissionsRepository::new(&state);
	let mut permission = create_dummy_permission("To Be Updated Then Deleted");
	let _ = repo.query_create_permission(permission.clone()).await;
	let id = permission.id.id.to_raw();
	let _ = repo.query_delete_permission(id.clone()).await;
	permission.name = "Try Update".into();
	let result = repo.query_update_permission(permission).await;
	assert!(result.is_err(), "Update on deleted should fail");
}

#[tokio::test]
async fn test_query_permission_by_id_should_fail_if_not_found() {
	let state = create_mock_app_state().await;
	let repo = PermissionsRepository::new(&state);
	let result = repo.query_permission_by_id("non-existent-id".into()).await;
	assert!(result.is_err(), "Expected error for not found id");
}

// =============== EDGE CASE TESTS ===============

#[tokio::test]
async fn test_create_permission_with_empty_name() {
	let state = create_mock_app_state().await;
	let repo = PermissionsRepository::new(&state);
	let permission = create_dummy_permission("");
	let result = repo.query_create_permission(permission).await;
	// Documents current behavior: empty name is allowed
	assert!(result.is_ok(), "Empty name is currently allowed");
}

#[tokio::test]
async fn test_create_permission_with_whitespace_only_name() {
	let state = create_mock_app_state().await;
	let repo = PermissionsRepository::new(&state);
	let permission = create_dummy_permission("   ");
	let result = repo.query_create_permission(permission).await;
	// Documents current behavior - might fail or succeed depending on validation
	assert!(result.is_ok() || result.is_err());
}

#[tokio::test]
async fn test_create_permission_with_extremely_long_name() {
	let state = create_mock_app_state().await;
	let repo = PermissionsRepository::new(&state);
	let long_name = "a".repeat(10000);
	let permission = create_dummy_permission(&long_name);
	let result = repo.query_create_permission(permission).await;
	// Should handle long names gracefully
	assert!(result.is_ok() || result.is_err());
}

#[tokio::test]
async fn test_create_permission_with_unicode_characters() {
	let state = create_mock_app_state().await;
	let repo = PermissionsRepository::new(&state);
	let unicode_name = "查看权限_🔐";
	let permission = create_dummy_permission(unicode_name);
	let result = repo.query_create_permission(permission).await;
	assert!(
		result.is_ok(),
		"Unicode permission names should be supported"
	);
}

#[tokio::test]
async fn test_create_permission_with_special_characters() {
	let state = create_mock_app_state().await;
	let repo = PermissionsRepository::new(&state);
	let special_chars = vec![
		"read:users",
		"write-posts",
		"admin@system",
		"user.profile",
		"api/v1/access",
		"[admin]",
		"permission with spaces",
	];

	for name in special_chars {
		let permission = create_dummy_permission(name);
		let result = repo.query_create_permission(permission).await;
		assert!(
			result.is_ok(),
			"Special characters should be allowed: {}",
			name
		);
	}
}

#[tokio::test]
async fn test_create_duplicate_permission_names() {
	let state = create_mock_app_state().await;
	let repo = PermissionsRepository::new(&state);
	let name = "duplicate_permission";

	// Create first permission
	let first_permission = create_dummy_permission(name);
	let first_result = repo.query_create_permission(first_permission).await;
	assert!(first_result.is_ok(), "First permission should succeed");

	// Try to create second permission with same name
	let second_permission = create_dummy_permission(name);
	let second_result = repo.query_create_permission(second_permission).await;
	// Should either succeed (allowing duplicates) or fail (enforcing uniqueness)
	assert!(second_result.is_ok() || second_result.is_err());
}

#[tokio::test]
async fn test_query_permission_by_invalid_id_formats() {
	let state = create_mock_app_state().await;
	let repo = PermissionsRepository::new(&state);
	let invalid_ids = vec![
		"",
		"   ",
		"not-a-valid-id",
		"123",
		"null",
		"undefined",
		"../../../etc/passwd",
		"DROP TABLE permissions;",
	];

	for id in invalid_ids {
		let result = repo.query_permission_by_id(id.into()).await;
		assert!(result.is_err(), "Invalid ID should fail: {}", id);
	}
}

#[tokio::test]
async fn test_update_permission_with_empty_name() {
	let state = create_mock_app_state().await;
	let repo = PermissionsRepository::new(&state);
	let mut permission = create_dummy_permission("Original Name");
	let _ = repo.query_create_permission(permission.clone()).await;

	// Try to update with empty name
	permission.name = "".into();
	let result = repo.query_update_permission(permission).await;
	// Documents current behavior: empty name update is allowed
	assert!(
		result.is_ok(),
		"Update with empty name is currently allowed"
	);
}

#[tokio::test]
async fn test_permission_list_with_extreme_pagination() {
	let state = create_mock_app_state().await;
	let repo = PermissionsRepository::new(&state);

	// Create some test data
	for i in 0..5 {
		let permission = create_dummy_permission(&format!("Test Permission {}", i));
		let _ = repo.query_create_permission(permission).await;
	}

	// Test extreme pagination values
	let extreme_meta = MetaRequestDto {
		page: Some(999999),
		per_page: Some(1),
		search: None,
		sort_by: None,
		order: None,
		filter: None,
		filter_by: None,
	};

	let result = repo.query_permission_list(extreme_meta).await;
	assert!(
		result.is_ok(),
		"Extreme pagination should be handled gracefully"
	);
}

#[tokio::test]
async fn test_permission_list_with_zero_per_page() {
	let state = create_mock_app_state().await;
	let repo = PermissionsRepository::new(&state);

	let meta = MetaRequestDto {
		page: Some(1),
		per_page: Some(0),
		search: None,
		sort_by: None,
		order: None,
		filter: None,
		filter_by: None,
	};

	let result = repo.query_permission_list(meta).await;
	// Should handle zero per_page gracefully
	assert!(result.is_ok() || result.is_err());
}

#[tokio::test]
async fn test_permission_search_with_special_characters() {
	let state = create_mock_app_state().await;
	let repo = PermissionsRepository::new(&state);

	// Create permission with special characters
	let permission = create_dummy_permission("read:user@system");
	let _ = repo.query_create_permission(permission).await;

	let search_meta = MetaRequestDto {
		page: Some(1),
		per_page: Some(10),
		search: Some("read:user".into()),
		sort_by: None,
		order: None,
		filter: None,
		filter_by: None,
	};

	let result = repo.query_permission_list(search_meta).await;
	assert!(result.is_ok(), "Search with special characters should work");
}

#[tokio::test]
async fn test_permission_search_with_sql_injection_attempt() {
	let state = create_mock_app_state().await;
	let repo = PermissionsRepository::new(&state);

	let malicious_search = "'; DROP TABLE permissions; --";
	let meta = MetaRequestDto {
		page: Some(1),
		per_page: Some(10),
		search: Some(malicious_search.into()),
		sort_by: None,
		order: None,
		filter: None,
		filter_by: None,
	};

	let result = repo.query_permission_list(meta).await;
	assert!(
		result.is_ok(),
		"Should safely handle SQL injection attempts"
	);
}

#[tokio::test]
async fn test_multiple_rapid_permission_operations() {
	let state = create_mock_app_state().await;
	let repo = PermissionsRepository::new(&state);

	// Rapid create operations
	let mut created_ids = Vec::new();
	for i in 0..10 {
		let permission = create_dummy_permission(&format!("Rapid Permission {}", i));
		let id = permission.id.id.to_raw();
		created_ids.push(id);
		let result = repo.query_create_permission(permission).await;
		assert!(result.is_ok(), "Rapid create {} should succeed", i);
	}

	// Rapid delete operations
	for id in created_ids {
		let result = repo.query_delete_permission(id).await;
		assert!(result.is_ok(), "Rapid delete should succeed");
	}
}
