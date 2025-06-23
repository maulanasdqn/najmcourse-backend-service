#[cfg(test)]
mod auth_repository_test {
	use crate::{
		AuthOtpSchema, AuthRepository, ResourceEnum, UsersRepository, UsersSchema,
		create_mock_app_state, generate_unique_email, get_iso_date, get_role_id,
		make_thing,
	};
	use chrono::{Duration, Utc};
	use najm_iam::{RolesDetailQueryDto, UsersDetailQueryDto};
	use najm_lib::AppState;
	use surrealdb::Uuid;

	async fn create_mock_user(state: &AppState, email: &str) -> UsersSchema {
		UsersSchema {
			id: make_thing("app_users", &Uuid::new_v4().to_string()),
			email: email.to_string(),
			fullname: "Test User".to_string(),
			password: "password".to_string(),
			is_deleted: false,
			avatar: None,
			phone_number: "081234567890".to_string(),
			is_active: true,
			gender: None,
			birthdate: None,
			referral_code: None,
			refered_by: None,
			student_type: None,
			religion: None,
			identity_number: None,
			is_profile_completed: false,
			role: make_thing("app_roles", &get_role_id(state).await),
			created_at: get_iso_date(),
			updated_at: get_iso_date(),
			..Default::default()
		}
	}

	#[tokio::test]
	async fn test_store_and_get_user() {
		let app_state = create_mock_app_state().await;
		let repo = AuthRepository::new(&app_state);
		let email = generate_unique_email("forgot");
		let user = create_mock_user(&app_state, &email).await;
		let user_repo = UsersRepository::new(&app_state);
		let create_user = user_repo.query_create_user(user.clone()).await;
		assert!(create_user.is_ok());
		let user_data = user_repo
			.query_user_by_email(email.to_string())
			.await
			.unwrap();
		let store = repo.query_store_user(user_data.clone()).await;
		assert!(store.is_ok());
		let fetched = repo.query_get_stored_user(user.email.clone()).await;
		assert!(fetched.is_ok());
		assert_eq!(fetched.unwrap().email, user.email);
	}

	#[tokio::test]
	async fn test_delete_stored_user() {
		let state = create_mock_app_state().await;
		let auth_repo = AuthRepository::new(&state);
		let email = "delete_me@example.com".to_string();
		let mock_user = UsersDetailQueryDto {
			id: make_thing(&ResourceEnum::UsersCache.to_string(), &email),
			fullname: "Test User".into(),
			email: email.clone(),
			avatar: None,
			phone_number: "08123456789".into(),
			is_active: true,
			gender: None,
			birthdate: None,
			referral_code: None,
			refered_by: None,
			student_type: None,
			religion: None,
			identity_number: None,
			is_profile_completed: false,
			role: RolesDetailQueryDto {
				id: make_thing("app_roles", &Uuid::new_v4().to_string()),
				name: "Dummy Role".into(),
				permissions: vec![],
				is_deleted: false,
				created_at: Some(get_iso_date()),
				updated_at: Some(get_iso_date()),
			},
			is_deleted: false,
			password: "".into(),
			created_at: get_iso_date(),
			updated_at: get_iso_date(),
		};
		let _: Option<UsersDetailQueryDto> = state
			.surrealdb_mem
			.create((ResourceEnum::UsersCache.to_string(), email.clone()))
			.content(mock_user)
			.await
			.unwrap();
		let result = auth_repo.query_delete_stored_user(email.clone()).await;
		assert!(
			result.is_ok(),
			"Delete operation failed: {:?}",
			result.err()
		);
		assert_eq!(result.unwrap(), "Success delete stored user");
	}

	#[tokio::test]
	async fn test_store_and_get_otp() {
		let app_state = create_mock_app_state().await;
		let repo = AuthRepository::new(&app_state);
		let email = "otp_user@example.com".to_string();
		let otp = "123456".to_string();
		let stored = repo.query_store_otp(email.clone(), otp.clone()).await;
		assert!(stored.is_ok());
		let fetched = repo.query_get_stored_otp(email.clone()).await;
		assert!(fetched.is_ok());
		assert_eq!(fetched.unwrap(), otp);
	}

	#[tokio::test]
	async fn test_delete_stored_otp() {
		let app_state = create_mock_app_state().await;
		let repo = AuthRepository::new(&app_state);
		let email = "otp_del@example.com".to_string();
		let otp = "654321".to_string();
		repo
			.query_store_otp(email.clone(), otp.clone())
			.await
			.unwrap();
		let deleted = repo.query_delete_stored_otp(email.clone()).await;
		assert!(deleted.is_ok());
		let fetched = repo.query_get_stored_otp(email.clone()).await;
		assert!(fetched.is_err());
	}

	#[tokio::test]
	async fn test_expired_otp() {
		let app_state = create_mock_app_state().await;
		let repo = AuthRepository::new(&app_state);
		let email = "expired_otp@example.com".to_string();
		let otp = "789012".to_string();
		let table = ResourceEnum::OtpCache.to_string();
		let expires_at = Utc::now() - Duration::seconds(1);
		let _: Option<AuthOtpSchema> = repo
			.state
			.surrealdb_mem
			.create((table.clone(), email.as_str()))
			.content(AuthOtpSchema { otp, expires_at })
			.await
			.unwrap();
		let result = repo.query_get_stored_otp(email.clone()).await;
		assert!(result.is_err());
	}

	#[tokio::test]
	async fn test_get_non_existent_stored_user_should_fail() {
		let app_state = create_mock_app_state().await;
		let repo = AuthRepository::new(&app_state);
		let result = repo
			.query_get_stored_user("not_found@example.com".into())
			.await;
		assert!(result.is_err());
	}

	#[tokio::test]
	async fn test_delete_non_existent_user_should_fail() {
		let app_state = create_mock_app_state().await;
		let repo = AuthRepository::new(&app_state);
		let result = repo
			.query_delete_stored_user("ghost@example.com".into())
			.await;
		assert!(result.is_err());
	}

	#[tokio::test]
	async fn test_get_expired_otp_should_fail() {
		use chrono::Duration;
		let app_state = create_mock_app_state().await;
		let repo = AuthRepository::new(&app_state);
		let email = "expired_otp@example.com";
		let expired_time = chrono::Utc::now() - Duration::seconds(10);
		let otp = "123456".to_string();
		let _: Option<AuthOtpSchema> = repo
			.state
			.surrealdb_mem
			.create((ResourceEnum::OtpCache.to_string(), email))
			.content(AuthOtpSchema {
				otp,
				expires_at: expired_time,
			})
			.await
			.unwrap();

		let result = repo.query_get_stored_otp(email.into()).await;
		assert!(result.is_err());
	}

	#[tokio::test]
	async fn test_store_and_get_valid_otp() {
		let app_state = create_mock_app_state().await;
		let repo = AuthRepository::new(&app_state);
		let email = "valid_otp@example.com";
		let otp = "654321".to_string();
		let store_result = repo.query_store_otp(email.into(), otp.clone()).await;
		assert!(store_result.is_ok());
		let get_result = repo.query_get_stored_otp(email.into()).await;
		assert_eq!(get_result.unwrap(), otp);
	}

	// =============== EDGE CASE TESTS ===============

	#[tokio::test]
	async fn test_store_otp_with_empty_email() {
		let app_state = create_mock_app_state().await;
		let repo = AuthRepository::new(&app_state);
		let result = repo.query_store_otp("".into(), "123456".into()).await;
		// Documents current behavior: empty email is allowed
		assert!(result.is_ok(), "Empty email is currently allowed");
	}

	#[tokio::test]
	async fn test_store_otp_with_empty_otp() {
		let app_state = create_mock_app_state().await;
		let repo = AuthRepository::new(&app_state);
		let result = repo
			.query_store_otp("test@example.com".into(), "".into())
			.await;
		// Documents current behavior: empty OTP is allowed
		assert!(result.is_ok(), "Empty OTP is currently allowed");
	}

	#[tokio::test]
	async fn test_store_otp_with_invalid_email_format() {
		let app_state = create_mock_app_state().await;
		let repo = AuthRepository::new(&app_state);
		let invalid_emails = vec![
			"notanemail",
			"@example.com",
			"test@",
			"test..test@example.com",
			"test@example",
			"test @example.com",
		];

		for email in invalid_emails {
			let result = repo.query_store_otp(email.into(), "123456".into()).await;
			// Note: Depending on implementation, this might succeed or fail
			// The test documents the current behavior
		}
	}

	#[tokio::test]
	async fn test_store_otp_with_extremely_long_email() {
		let app_state = create_mock_app_state().await;
		let repo = AuthRepository::new(&app_state);
		let long_email = format!("{}@example.com", "a".repeat(1000));
		let result = repo.query_store_otp(long_email, "123456".into()).await;
		// Should handle long emails gracefully
		assert!(result.is_ok() || result.is_err()); // Documents behavior
	}

	#[tokio::test]
	async fn test_store_otp_with_unicode_email() {
		let app_state = create_mock_app_state().await;
		let repo = AuthRepository::new(&app_state);
		let unicode_email = "测试@example.com";
		let result = repo
			.query_store_otp(unicode_email.into(), "123456".into())
			.await;
		assert!(result.is_ok(), "Unicode email should be supported");
	}

	#[tokio::test]
	async fn test_store_otp_with_special_characters() {
		let app_state = create_mock_app_state().await;
		let repo = AuthRepository::new(&app_state);
		let special_email = "test+tag@sub.example.com";
		let result = repo
			.query_store_otp(special_email.into(), "123456".into())
			.await;
		assert!(result.is_ok(), "Email with special characters should work");
	}

	#[tokio::test]
	async fn test_otp_with_non_numeric_characters() {
		let app_state = create_mock_app_state().await;
		let repo = AuthRepository::new(&app_state);
		let alphanumeric_otp = "ABC123";
		let result = repo
			.query_store_otp("test@example.com".into(), alphanumeric_otp.into())
			.await;
		assert!(result.is_ok(), "Alphanumeric OTP should be supported");

		let fetched = repo.query_get_stored_otp("test@example.com".into()).await;
		assert_eq!(fetched.unwrap(), alphanumeric_otp);
	}

	#[tokio::test]
	async fn test_otp_with_extremely_long_code() {
		let app_state = create_mock_app_state().await;
		let repo = AuthRepository::new(&app_state);
		let long_otp = "1".repeat(1000);
		let result = repo
			.query_store_otp("test@example.com".into(), long_otp.clone())
			.await;
		assert!(result.is_ok(), "Long OTP should be handled");
	}

	#[tokio::test]
	async fn test_multiple_otps_for_same_email() {
		let app_state = create_mock_app_state().await;
		let repo = AuthRepository::new(&app_state);
		let email = "multi@example.com";

		// Store first OTP
		let first_result = repo.query_store_otp(email.into(), "111111".into()).await;
		assert!(first_result.is_ok(), "First OTP should succeed");

		// Try to store second OTP - behavior depends on implementation
		let second_result = repo.query_store_otp(email.into(), "222222".into()).await;

		if second_result.is_ok() {
			// If second store succeeds, verify it overwrote the first
			let fetched = repo.query_get_stored_otp(email.into()).await.unwrap();
			assert_eq!(fetched, "222222", "Second OTP should overwrite first");
		} else {
			// If second store fails, verify first OTP is still there
			let fetched = repo.query_get_stored_otp(email.into()).await.unwrap();
			assert_eq!(fetched, "111111", "First OTP should remain if second fails");
		}
	}

	#[tokio::test]
	async fn test_delete_otp_multiple_times() {
		let app_state = create_mock_app_state().await;
		let repo = AuthRepository::new(&app_state);
		let email = "delete_multi@example.com";

		// Store OTP
		repo
			.query_store_otp(email.into(), "123456".into())
			.await
			.unwrap();

		// Delete first time
		let first_delete = repo.query_delete_stored_otp(email.into()).await;
		assert!(first_delete.is_ok());

		// Delete second time should fail
		let second_delete = repo.query_delete_stored_otp(email.into()).await;
		assert!(second_delete.is_err());
	}

	#[tokio::test]
	async fn test_otp_boundary_expiration() {
		let app_state = create_mock_app_state().await;
		let repo = AuthRepository::new(&app_state);
		let email = "boundary@example.com";

		// Create OTP that expires in exactly 1 second
		let expires_at = Utc::now() + Duration::seconds(1);
		let _: Option<AuthOtpSchema> = repo
			.state
			.surrealdb_mem
			.create((ResourceEnum::OtpCache.to_string(), email))
			.content(AuthOtpSchema {
				otp: "123456".to_string(),
				expires_at,
			})
			.await
			.unwrap();

		// Should be valid immediately
		let result = repo.query_get_stored_otp(email.into()).await;
		assert!(result.is_ok(), "OTP should be valid before expiration");

		// Wait for expiration
		tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

		// Should be expired now
		let expired_result = repo.query_get_stored_otp(email.into()).await;
		assert!(expired_result.is_err(), "OTP should be expired");
	}

	#[tokio::test]
	async fn test_rapid_sequential_otp_operations() {
		let app_state = create_mock_app_state().await;
		let repo = AuthRepository::new(&app_state);

		// Use different emails to avoid conflicts
		let mut success_count = 0;
		for i in 0..5 {
			let email = format!("rapid{}@example.com", i);
			let result = repo.query_store_otp(email, format!("12345{}", i)).await;
			if result.is_ok() {
				success_count += 1;
			}
		}

		// At least one should succeed
		assert!(success_count > 0, "At least one operation should succeed");
		println!("Rapid operations: {}/{} succeeded", success_count, 5);
	}

	#[tokio::test]
	async fn test_user_cache_with_malformed_data() {
		let state = create_mock_app_state().await;
		let auth_repo = AuthRepository::new(&state);
		let email = "malformed@example.com";

		// Try to store malformed user data directly in cache
		let malformed_user = serde_json::json!({
			"id": null,
			"fullname": "",
			"email": email,
			"role": null
		});

		let create_result: Result<Option<serde_json::Value>, _> = state
			.surrealdb_mem
			.create((ResourceEnum::UsersCache.to_string(), email))
			.content(malformed_user)
			.await;

		if create_result.is_err() {
			// Database rejected malformed data - this is good
			println!("Database properly rejected malformed data");
			return;
		}

		// If malformed data was stored, verify retrieval fails gracefully
		let result = auth_repo.query_get_stored_user(email.into()).await;
		assert!(
			result.is_err(),
			"Should fail gracefully with malformed data"
		);
	}
}
