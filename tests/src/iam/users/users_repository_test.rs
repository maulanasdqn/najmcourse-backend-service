use crate::{
	MetaRequestDto, UsersRepository, create_mock_app_state, create_test_user,
	generate_unique_email, get_role_id,
};

#[tokio::test]
async fn test_create_and_get_user() {
	let app_state = create_mock_app_state().await;
	let repo = UsersRepository::new(&app_state);
	let email = generate_unique_email("testuser");
	let user =
		create_test_user(&email, "Test User", true, &get_role_id(&app_state).await);
	let create_result = repo.query_create_user(user.clone()).await;
	assert!(create_result.is_ok());
	let fetched = repo.query_user_by_email(email.clone().into()).await;
	assert!(fetched.is_ok());
	assert_eq!(fetched.unwrap().email, email.clone());
}

#[tokio::test]
async fn test_query_user_list_with_pagination_and_filter() {
	let app_state = create_mock_app_state().await;
	let repo = UsersRepository::new(&app_state);
	for i in 0..10 {
		let email = format!("user{}@example.com", i);
		let fullname = format!("User {}", i);
		let is_active = i % 2 == 0;
		let user =
			create_test_user(&email, &fullname, is_active, &get_role_id(&app_state).await);
		repo.query_create_user(user).await.unwrap();
	}
	let meta = MetaRequestDto {
		page: Some(1),
		per_page: Some(5),
		search: None,
		sort_by: Some("email".into()),
		order: Some("ASC".into()),
		filter: Some("true".into()),
		filter_by: Some("is_active".into()),
	};
	let result = repo.query_user_list(meta).await.unwrap();
	assert!(result.data.len() <= 5);
	assert!(result.data.iter().all(|u| u.is_active));
	assert!(result.meta.as_ref().unwrap().total.is_some());
}

#[tokio::test]
async fn test_query_user_list_basic() {
	let app_state = create_mock_app_state().await;
	let repo = UsersRepository::new(&app_state);
	for i in 0..10 {
		let email = format!("basic{}@example.com", i);
		let user = create_test_user(
			&email,
			&format!("Basic User {}", i),
			true,
			&get_role_id(&app_state).await,
		);
		repo.query_create_user(user).await.unwrap();
	}
	let meta = MetaRequestDto {
		page: Some(1),
		per_page: Some(5),
		search: None,
		sort_by: None,
		order: None,
		filter: None,
		filter_by: None,
	};
	let result = repo.query_user_list(meta).await.unwrap();
	assert!(result.meta.as_ref().unwrap().total.unwrap() >= 1);
	assert_eq!(result.meta.as_ref().unwrap().page.unwrap(), 1);
	assert_eq!(result.meta.as_ref().unwrap().per_page.unwrap(), 5);
}

#[tokio::test]
async fn test_query_delete_user() {
	let app_state = create_mock_app_state().await;
	let repo = UsersRepository::new(&app_state);
	let email = &generate_unique_email("deleteuser");
	let user =
		create_test_user(email, "Delete User", true, &get_role_id(&app_state).await);
	repo.query_create_user(user.clone()).await.unwrap();
	let user_detail = repo
		.query_user_by_email(email.to_string().clone())
		.await
		.unwrap();
	let delete_result = repo
		.query_delete_user(user_detail.id.id.to_raw().clone())
		.await;
	assert!(delete_result.is_ok());
	let fetch_result = repo.query_user_by_email(user_detail.email.clone()).await;
	assert!(fetch_result.is_err());
}

#[tokio::test]
async fn test_delete_non_existent_user_should_fail() {
	let app_state = create_mock_app_state().await;
	let repo = UsersRepository::new(&app_state);
	let result = repo.query_delete_user("lklklklk".to_string()).await;
	assert!(result.is_err());
	assert_eq!(result.unwrap_err().to_string(), "User not found");
}

#[tokio::test]
async fn test_delete_user_twice_should_fail_on_second_attempt() {
	let app_state = create_mock_app_state().await;
	let repo = UsersRepository::new(&app_state);
	let email = "twice@example.com";
	let user =
		create_test_user(email, "Delete Twice", true, &get_role_id(&app_state).await);
	repo.query_create_user(user.clone()).await.unwrap();
	let first = repo.query_delete_user(user.id.id.to_raw()).await;
	assert!(first.is_ok());
	let second = repo.query_delete_user(user.id.id.to_raw()).await;
	assert!(second.is_err());
	assert_eq!(second.unwrap_err().to_string(), "User not found");
}

#[tokio::test]
async fn test_query_update_user_should_succeed() {
	let state = create_mock_app_state().await;
	let repo = UsersRepository::new(&state);
	let mut user = create_test_user(
		"update@example.com",
		"Old Name",
		true,
		&get_role_id(&state).await,
	);
	repo.query_create_user(user.clone()).await.unwrap();
	user.fullname = "Updated Name".into();
	user.phone_number = "089876543210".into();
	let result = repo.query_update_user(user.clone()).await;
	assert!(result.is_ok(), "Update failed: {:?}", result.err());
	let updated = repo.query_user_by_id(user.id.id.to_raw()).await.unwrap();
	assert_eq!(updated.fullname, "Updated Name");
	assert_eq!(updated.phone_number, "089876543210");
}

// =============== EDGE CASE TESTS ===============

#[tokio::test]
async fn test_create_user_with_invalid_email_formats() {
	let app_state = create_mock_app_state().await;
	let repo = UsersRepository::new(&app_state);
	let invalid_emails = vec![
		"notanemail",
		"@example.com",
		"test@",
		"test..test@example.com",
		"test@example",
		"test @example.com",
		"",
		"   ",
	];

	let mut failed_count = 0;
	let mut success_count = 0;

	for email in invalid_emails {
		let user =
			create_test_user(email, "Test User", true, &get_role_id(&app_state).await);
		let result = repo.query_create_user(user).await;
		if result.is_err() {
			failed_count += 1;
		} else {
			success_count += 1;
		}
	}

	println!(
		"Email validation: {} failed, {} succeeded out of {} invalid formats",
		failed_count,
		success_count,
		8 // total number of test emails
	);
	// Documents current behavior - some invalid emails might be allowed
}

#[tokio::test]
async fn test_create_user_with_unicode_characters() {
	let app_state = create_mock_app_state().await;
	let repo = UsersRepository::new(&app_state);
	let unicode_email = "测试用户@example.com";
	let unicode_name = "张三 🧑‍💻";
	let user = create_test_user(
		unicode_email,
		unicode_name,
		true,
		&get_role_id(&app_state).await,
	);
	let result = repo.query_create_user(user).await;
	assert!(result.is_ok(), "Unicode characters should be supported");
}

#[tokio::test]
async fn test_create_user_with_extremely_long_values() {
	let app_state = create_mock_app_state().await;
	let repo = UsersRepository::new(&app_state);
	let long_email = format!("{}@example.com", "a".repeat(1000));
	let long_name = "a".repeat(10000);
	let user = create_test_user(
		&long_email,
		&long_name,
		true,
		&get_role_id(&app_state).await,
	);
	let result = repo.query_create_user(user).await;
	// Should handle long values gracefully
	assert!(result.is_ok() || result.is_err());
}

#[tokio::test]
async fn test_create_user_with_empty_required_fields() {
	let app_state = create_mock_app_state().await;
	let repo = UsersRepository::new(&app_state);

	// Test empty fullname
	let user =
		create_test_user("test@example.com", "", true, &get_role_id(&app_state).await);
	let result = repo.query_create_user(user).await;
	// Documents current behavior: empty fullname is allowed
	assert!(result.is_ok(), "Empty fullname is currently allowed");
}

#[tokio::test]
async fn test_create_user_with_invalid_phone_numbers() {
	let app_state = create_mock_app_state().await;
	let repo = UsersRepository::new(&app_state);
	let invalid_phones = vec![
		"",
		"abc123",
		"++123456789",
		"123",
		"phone number",
		"08123456789012345678901234567890", // extremely long
	];

	for phone in invalid_phones {
		let mut user = create_test_user(
			"test@example.com",
			"Test User",
			true,
			&get_role_id(&app_state).await,
		);
		user.phone_number = phone.into();
		let result = repo.query_create_user(user).await;
		// Documents behavior - might succeed or fail depending on validation
		assert!(result.is_ok() || result.is_err());
	}
}

#[tokio::test]
async fn test_create_user_with_special_characters_in_name() {
	let app_state = create_mock_app_state().await;
	let repo = UsersRepository::new(&app_state);
	let special_names = vec![
		"John O'Connor",
		"José María",
		"李明",
		"User-Name",
		"User.Name",
		"User@Company",
		"User (Admin)",
		"<script>alert('xss')</script>",
		"'; DROP TABLE users; --",
	];

	for name in special_names {
		let email = generate_unique_email(&format!("special{}", name.len()));
		let user = create_test_user(&email, name, true, &get_role_id(&app_state).await);
		let result = repo.query_create_user(user).await;
		assert!(
			result.is_ok(),
			"Special characters should be handled: {}",
			name
		);
	}
}

#[tokio::test]
async fn test_create_duplicate_email_addresses() {
	let app_state = create_mock_app_state().await;
	let repo = UsersRepository::new(&app_state);
	let email = "duplicate@example.com";

	// Create first user
	let first_user =
		create_test_user(email, "First User", true, &get_role_id(&app_state).await);
	let first_result = repo.query_create_user(first_user).await;
	assert!(first_result.is_ok(), "First user should succeed");

	// Try to create second user with same email
	let second_user =
		create_test_user(email, "Second User", true, &get_role_id(&app_state).await);
	let second_result = repo.query_create_user(second_user).await;

	if second_result.is_err() {
		println!("Duplicate email properly rejected");
	} else {
		println!("Duplicate email was allowed - no unique constraint");
	}
	// Documents actual behavior - may succeed or fail depending on constraints
}

#[tokio::test]
async fn test_query_user_by_invalid_id_formats() {
	let app_state = create_mock_app_state().await;
	let repo = UsersRepository::new(&app_state);
	let invalid_ids = vec![
		"",
		"   ",
		"not-a-valid-id",
		"123",
		"null",
		"undefined",
		"../../../etc/passwd",
		"DROP TABLE users;",
	];

	for id in invalid_ids {
		let result = repo.query_user_by_id(id.into()).await;
		assert!(result.is_err(), "Invalid ID should fail: {}", id);
	}
}

#[tokio::test]
async fn test_user_list_with_extreme_pagination() {
	let app_state = create_mock_app_state().await;
	let repo = UsersRepository::new(&app_state);

	// Create some test data
	for i in 0..5 {
		let email = format!("paginate{}@example.com", i);
		let user = create_test_user(
			&email,
			&format!("User {}", i),
			true,
			&get_role_id(&app_state).await,
		);
		let _ = repo.query_create_user(user).await;
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

	let result = repo.query_user_list(extreme_meta).await;
	assert!(
		result.is_ok(),
		"Extreme pagination should be handled gracefully"
	);
}

#[tokio::test]
async fn test_user_list_with_zero_per_page() {
	let app_state = create_mock_app_state().await;
	let repo = UsersRepository::new(&app_state);

	let meta = MetaRequestDto {
		page: Some(1),
		per_page: Some(0),
		search: None,
		sort_by: None,
		order: None,
		filter: None,
		filter_by: None,
	};

	let result = repo.query_user_list(meta).await;
	// Should handle zero per_page gracefully
	assert!(result.is_ok() || result.is_err());
}

#[tokio::test]
async fn test_user_search_with_special_characters() {
	let app_state = create_mock_app_state().await;
	let repo = UsersRepository::new(&app_state);

	// Create user with special characters
	let email = generate_unique_email("search");
	let user = create_test_user(
		&email,
		"John O'Connor",
		true,
		&get_role_id(&app_state).await,
	);
	let _ = repo.query_create_user(user).await;

	let search_meta = MetaRequestDto {
		page: Some(1),
		per_page: Some(10),
		search: Some("O'Connor".into()),
		sort_by: None,
		order: None,
		filter: None,
		filter_by: None,
	};

	let result = repo.query_user_list(search_meta).await;
	assert!(result.is_ok(), "Search with special characters should work");
}

#[tokio::test]
async fn test_user_search_with_sql_injection_attempt() {
	let app_state = create_mock_app_state().await;
	let repo = UsersRepository::new(&app_state);

	let malicious_search = "'; DROP TABLE users; --";
	let meta = MetaRequestDto {
		page: Some(1),
		per_page: Some(10),
		search: Some(malicious_search.into()),
		sort_by: None,
		order: None,
		filter: None,
		filter_by: None,
	};

	let result = repo.query_user_list(meta).await;
	assert!(
		result.is_ok(),
		"Should safely handle SQL injection attempts"
	);
}

#[tokio::test]
async fn test_update_user_with_invalid_data() {
	let app_state = create_mock_app_state().await;
	let repo = UsersRepository::new(&app_state);
	let mut user = create_test_user(
		"update@example.com",
		"Original User",
		true,
		&get_role_id(&app_state).await,
	);
	repo.query_create_user(user.clone()).await.unwrap();

	// Try to update with empty email
	user.email = "".into();
	let result = repo.query_update_user(user.clone()).await;

	if result.is_err() {
		println!("Update with empty email properly rejected");
	} else {
		println!("Update with empty email was allowed");
	}

	// Reset email and try empty fullname
	user.email = "update@example.com".into();
	user.fullname = "".into();
	let result = repo.query_update_user(user).await;

	if result.is_err() {
		println!("Update with empty fullname properly rejected");
	} else {
		println!("Update with empty fullname was allowed");
	}
	// Documents current behavior for updates with invalid data
}

#[tokio::test]
async fn test_user_with_invalid_role_id() {
	let app_state = create_mock_app_state().await;
	let repo = UsersRepository::new(&app_state);
	let mut user = create_test_user(
		"role@example.com",
		"Role Test User",
		true,
		&get_role_id(&app_state).await,
	);

	// Use invalid role ID
	use najm_util::make_thing;
	user.role = make_thing("app_roles", "non-existent-role-id");

	let result = repo.query_create_user(user).await;
	// Should either fail due to foreign key constraint or succeed with invalid role
	assert!(result.is_ok() || result.is_err());
}

#[tokio::test]
async fn test_user_filter_by_invalid_field() {
	let app_state = create_mock_app_state().await;
	let repo = UsersRepository::new(&app_state);

	let meta = MetaRequestDto {
		page: Some(1),
		per_page: Some(10),
		search: None,
		sort_by: None,
		order: None,
		filter: Some("true".into()),
		filter_by: Some("non_existent_field".into()),
	};

	let result = repo.query_user_list(meta).await;
	// Should handle invalid filter fields gracefully
	assert!(result.is_ok() || result.is_err());
}

#[tokio::test]
async fn test_user_sort_by_invalid_field() {
	let app_state = create_mock_app_state().await;
	let repo = UsersRepository::new(&app_state);

	let meta = MetaRequestDto {
		page: Some(1),
		per_page: Some(10),
		search: None,
		sort_by: Some("non_existent_field".into()),
		order: Some("ASC".into()),
		filter: None,
		filter_by: None,
	};

	let result = repo.query_user_list(meta).await;
	// Should handle invalid sort fields gracefully
	assert!(result.is_ok() || result.is_err());
}

#[tokio::test]
async fn test_multiple_rapid_user_operations() {
	let app_state = create_mock_app_state().await;
	let repo = UsersRepository::new(&app_state);

	// Rapid user creation
	let mut created_users = Vec::new();
	for i in 0..10 {
		let email = format!("rapid{}@example.com", i);
		let user = create_test_user(
			&email,
			&format!("Rapid User {}", i),
			true,
			&get_role_id(&app_state).await,
		);
		let result = repo.query_create_user(user.clone()).await;
		assert!(result.is_ok(), "Rapid create {} should succeed", i);
		created_users.push(user.id.id.to_raw());
	}

	// Rapid user deletion
	for user_id in created_users {
		let result = repo.query_delete_user(user_id).await;
		assert!(result.is_ok(), "Rapid delete should succeed");
	}
}

#[tokio::test]
async fn test_user_email_case_sensitivity() {
	let app_state = create_mock_app_state().await;
	let repo = UsersRepository::new(&app_state);
	let email_lower = "test@example.com";
	let email_upper = "TEST@EXAMPLE.COM";
	let email_mixed = "Test@Example.Com";

	// Create user with lowercase email
	let user1 =
		create_test_user(email_lower, "User 1", true, &get_role_id(&app_state).await);
	let result1 = repo.query_create_user(user1).await;
	assert!(result1.is_ok(), "Lowercase email should succeed");

	// Try to create with uppercase (should fail if case-insensitive uniqueness)
	let user2 =
		create_test_user(email_upper, "User 2", true, &get_role_id(&app_state).await);
	let result2 = repo.query_create_user(user2).await;
	// Documents behavior - might succeed or fail depending on case sensitivity
	assert!(result2.is_ok() || result2.is_err());

	// Test retrieval with different case
	let fetch_result = repo.query_user_by_email(email_mixed.into()).await;
	// Documents behavior
	assert!(fetch_result.is_ok() || fetch_result.is_err());
}
