use najm_exam::AnswersSchema;
use najm_util::{get_iso_date, make_thing};
use serde_json;
use surrealdb::Uuid;

#[tokio::test]
async fn test_answers_schema_creation() {
	let now = get_iso_date();
	let user_id = Uuid::new_v4().to_string();
	let test_id = Uuid::new_v4().to_string();
	let session_id = Uuid::new_v4().to_string();
	let question_id = Uuid::new_v4().to_string();
	let option_id = Uuid::new_v4().to_string();

	let schema = AnswersSchema {
		id: make_thing("app_answers", &Uuid::new_v4().to_string()),
		user: make_thing("app_users", &user_id),
		test: make_thing("app_tests", &test_id),
		sub_test: None,
		session: make_thing("app_sessions", &session_id),
		question: make_thing("app_questions", &question_id),
		option: make_thing("app_options", &option_id),
		is_correct: true,
		is_deleted: false,
		created_at: now.clone(),
		updated_at: now.clone(),
	};

	// Verify all fields are set correctly
	assert_eq!(schema.user.id.to_raw(), user_id);
	assert_eq!(schema.test.id.to_raw(), test_id);
	assert_eq!(schema.session.id.to_raw(), session_id);
	assert_eq!(schema.question.id.to_raw(), question_id);
	assert_eq!(schema.option.id.to_raw(), option_id);
	assert!(schema.is_correct);
	assert!(!schema.is_deleted);
	assert!(schema.sub_test.is_none());
}

#[tokio::test]
async fn test_answers_schema_with_sub_test() {
	let now = get_iso_date();
	let sub_test_id = Uuid::new_v4().to_string();

	let schema = AnswersSchema {
		id: make_thing("app_answers", &Uuid::new_v4().to_string()),
		user: make_thing("app_users", &Uuid::new_v4().to_string()),
		test: make_thing("app_tests", &Uuid::new_v4().to_string()),
		sub_test: Some(make_thing("app_sub_tests", &sub_test_id)),
		session: make_thing("app_sessions", &Uuid::new_v4().to_string()),
		question: make_thing("app_questions", &Uuid::new_v4().to_string()),
		option: make_thing("app_options", &Uuid::new_v4().to_string()),
		is_correct: false,
		is_deleted: false,
		created_at: now.clone(),
		updated_at: now.clone(),
	};

	// Verify sub_test is properly set
	assert!(schema.sub_test.is_some());
	let sub_test = schema.sub_test.unwrap();
	assert_eq!(sub_test.id.to_raw(), sub_test_id);
	assert!(!schema.is_correct);
}

#[tokio::test]
async fn test_answers_schema_serialization() {
	let now = get_iso_date();
	let answer_id = Uuid::new_v4().to_string();
	let user_id = Uuid::new_v4().to_string();

	let schema = AnswersSchema {
		id: make_thing("app_answers", &answer_id),
		user: make_thing("app_users", &user_id),
		test: make_thing("app_tests", &Uuid::new_v4().to_string()),
		sub_test: None,
		session: make_thing("app_sessions", &Uuid::new_v4().to_string()),
		question: make_thing("app_questions", &Uuid::new_v4().to_string()),
		option: make_thing("app_options", &Uuid::new_v4().to_string()),
		is_correct: true,
		is_deleted: false,
		created_at: now.clone(),
		updated_at: now.clone(),
	};

	let json_result = serde_json::to_string(&schema);
	assert!(json_result.is_ok(), "Schema should serialize to JSON");
	
	let json = json_result.unwrap();
	assert!(json.contains(&answer_id));
	assert!(json.contains(&user_id));
	assert!(json.contains("is_correct"));
	assert!(json.contains("is_deleted"));
	assert!(json.contains(&now));
}

#[tokio::test]
async fn test_answers_schema_deserialization() {
	let answer_id = "answer-123";
	let user_id = "user-456";
	let test_id = "test-789";
	let session_id = "session-101";
	let question_id = "question-202";
	let option_id = "option-303";
	let timestamp = "2025-01-01T12:00:00Z";

	let json = format!(r#"{{
		"id": {{
			"tb": "app_answers",
			"id": {{
				"String": "{}"
			}}
		}},
		"user": {{
			"tb": "app_users",
			"id": {{
				"String": "{}"
			}}
		}},
		"test": {{
			"tb": "app_tests",
			"id": {{
				"String": "{}"
			}}
		}},
		"sub_test": null,
		"session": {{
			"tb": "app_sessions",
			"id": {{
				"String": "{}"
			}}
		}},
		"question": {{
			"tb": "app_questions",
			"id": {{
				"String": "{}"
			}}
		}},
		"option": {{
			"tb": "app_options",
			"id": {{
				"String": "{}"
			}}
		}},
		"is_correct": false,
		"is_deleted": true,
		"created_at": "{}",
		"updated_at": "{}"
	}}"#, answer_id, user_id, test_id, session_id, question_id, option_id, timestamp, timestamp);

	let schema_result: Result<AnswersSchema, _> = serde_json::from_str(&json);
	assert!(schema_result.is_ok(), "Valid JSON should deserialize to schema");
	
	let schema = schema_result.unwrap();
	assert_eq!(schema.id.id.to_raw(), answer_id);
	assert_eq!(schema.user.id.to_raw(), user_id);
	assert_eq!(schema.test.id.to_raw(), test_id);
	assert_eq!(schema.session.id.to_raw(), session_id);
	assert_eq!(schema.question.id.to_raw(), question_id);
	assert_eq!(schema.option.id.to_raw(), option_id);
	assert!(!schema.is_correct);
	assert!(schema.is_deleted);
	assert!(schema.sub_test.is_none());
	assert_eq!(schema.created_at, timestamp);
	assert_eq!(schema.updated_at, timestamp);
}

#[tokio::test]
async fn test_answers_schema_deserialization_with_sub_test() {
	let sub_test_id = "subtest-777";

	let json = format!(r#"{{
		"id": {{
			"tb": "app_answers",
			"id": {{
				"String": "answer-999"
			}}
		}},
		"user": {{
			"tb": "app_users",
			"id": {{
				"String": "user-888"
			}}
		}},
		"test": {{
			"tb": "app_tests",
			"id": {{
				"String": "test-777"
			}}
		}},
		"sub_test": {{
			"tb": "app_sub_tests",
			"id": {{
				"String": "{}"
			}}
		}},
		"session": {{
			"tb": "app_sessions",
			"id": {{
				"String": "session-666"
			}}
		}},
		"question": {{
			"tb": "app_questions",
			"id": {{
				"String": "question-555"
			}}
		}},
		"option": {{
			"tb": "app_options",
			"id": {{
				"String": "option-444"
			}}
		}},
		"is_correct": true,
		"is_deleted": false,
		"created_at": "2025-01-01T12:00:00Z",
		"updated_at": "2025-01-01T13:00:00Z"
	}}"#, sub_test_id);

	let schema_result: Result<AnswersSchema, _> = serde_json::from_str(&json);
	assert!(schema_result.is_ok(), "Valid JSON with sub_test should deserialize");
	
	let schema = schema_result.unwrap();
	assert!(schema.sub_test.is_some());
	let sub_test = schema.sub_test.unwrap();
	assert_eq!(sub_test.id.to_raw(), sub_test_id);
	assert!(schema.is_correct);
	assert!(!schema.is_deleted);
}

#[tokio::test]
async fn test_answers_schema_clone() {
	let now = get_iso_date();
	let original_id = Uuid::new_v4().to_string();

	let original = AnswersSchema {
		id: make_thing("app_answers", &original_id),
		user: make_thing("app_users", &Uuid::new_v4().to_string()),
		test: make_thing("app_tests", &Uuid::new_v4().to_string()),
		sub_test: None,
		session: make_thing("app_sessions", &Uuid::new_v4().to_string()),
		question: make_thing("app_questions", &Uuid::new_v4().to_string()),
		option: make_thing("app_options", &Uuid::new_v4().to_string()),
		is_correct: true,
		is_deleted: false,
		created_at: now.clone(),
		updated_at: now.clone(),
	};

	let cloned = original.clone();
	assert_eq!(original.id.id.to_raw(), cloned.id.id.to_raw());
	assert_eq!(original.user.id.to_raw(), cloned.user.id.to_raw());
	assert_eq!(original.test.id.to_raw(), cloned.test.id.to_raw());
	assert_eq!(original.session.id.to_raw(), cloned.session.id.to_raw());
	assert_eq!(original.question.id.to_raw(), cloned.question.id.to_raw());
	assert_eq!(original.option.id.to_raw(), cloned.option.id.to_raw());
	assert_eq!(original.is_correct, cloned.is_correct);
	assert_eq!(original.is_deleted, cloned.is_deleted);
	assert_eq!(original.created_at, cloned.created_at);
	assert_eq!(original.updated_at, cloned.updated_at);
}

#[tokio::test]
async fn test_answers_schema_debug() {
	let now = get_iso_date();
	let answer_id = "debug-answer-123";

	let schema = AnswersSchema {
		id: make_thing("app_answers", answer_id),
		user: make_thing("app_users", "debug-user-456"),
		test: make_thing("app_tests", "debug-test-789"),
		sub_test: Some(make_thing("app_sub_tests", "debug-subtest-101")),
		session: make_thing("app_sessions", "debug-session-202"),
		question: make_thing("app_questions", "debug-question-303"),
		option: make_thing("app_options", "debug-option-404"),
		is_correct: false,
		is_deleted: true,
		created_at: now.clone(),
		updated_at: now.clone(),
	};

	let debug_string = format!("{:?}", schema);
	assert!(debug_string.contains("AnswersSchema"));
	assert!(debug_string.contains(answer_id));
	assert!(debug_string.contains("debug-user-456"));
	assert!(debug_string.contains("debug-test-789"));
	assert!(debug_string.contains("debug-subtest-101"));
	assert!(debug_string.contains("is_correct: false"));
	assert!(debug_string.contains("is_deleted: true"));
}

#[tokio::test]
async fn test_answers_schema_table_names() {
	let schema = AnswersSchema {
		id: make_thing("app_answers", &Uuid::new_v4().to_string()),
		user: make_thing("app_users", &Uuid::new_v4().to_string()),
		test: make_thing("app_tests", &Uuid::new_v4().to_string()),
		sub_test: Some(make_thing("app_sub_tests", &Uuid::new_v4().to_string())),
		session: make_thing("app_sessions", &Uuid::new_v4().to_string()),
		question: make_thing("app_questions", &Uuid::new_v4().to_string()),
		option: make_thing("app_options", &Uuid::new_v4().to_string()),
		is_correct: true,
		is_deleted: false,
		created_at: get_iso_date(),
		updated_at: get_iso_date(),
	};

	// Verify all Thing references use correct table names
	assert_eq!(schema.id.tb, "app_answers");
	assert_eq!(schema.user.tb, "app_users");
	assert_eq!(schema.test.tb, "app_tests");
	assert_eq!(schema.session.tb, "app_sessions");
	assert_eq!(schema.question.tb, "app_questions");
	assert_eq!(schema.option.tb, "app_options");
	
	let sub_test = schema.sub_test.unwrap();
	assert_eq!(sub_test.tb, "app_sub_tests");
}

#[tokio::test]
async fn test_answers_schema_boolean_states() {
	let now = get_iso_date();

	// Test correct answer, not deleted
	let correct_active = AnswersSchema {
		id: make_thing("app_answers", &Uuid::new_v4().to_string()),
		user: make_thing("app_users", &Uuid::new_v4().to_string()),
		test: make_thing("app_tests", &Uuid::new_v4().to_string()),
		sub_test: None,
		session: make_thing("app_sessions", &Uuid::new_v4().to_string()),
		question: make_thing("app_questions", &Uuid::new_v4().to_string()),
		option: make_thing("app_options", &Uuid::new_v4().to_string()),
		is_correct: true,
		is_deleted: false,
		created_at: now.clone(),
		updated_at: now.clone(),
	};

	assert!(correct_active.is_correct);
	assert!(!correct_active.is_deleted);

	// Test incorrect answer, deleted
	let incorrect_deleted = AnswersSchema {
		id: make_thing("app_answers", &Uuid::new_v4().to_string()),
		user: make_thing("app_users", &Uuid::new_v4().to_string()),
		test: make_thing("app_tests", &Uuid::new_v4().to_string()),
		sub_test: None,
		session: make_thing("app_sessions", &Uuid::new_v4().to_string()),
		question: make_thing("app_questions", &Uuid::new_v4().to_string()),
		option: make_thing("app_options", &Uuid::new_v4().to_string()),
		is_correct: false,
		is_deleted: true,
		created_at: now.clone(),
		updated_at: now.clone(),
	};

	assert!(!incorrect_deleted.is_correct);
	assert!(incorrect_deleted.is_deleted);
}

#[tokio::test]
async fn test_answers_schema_timestamp_format() {
	let timestamp1 = get_iso_date();
	
	// Small delay to ensure different timestamps
	tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
	
	let timestamp2 = get_iso_date();

	let schema = AnswersSchema {
		id: make_thing("app_answers", &Uuid::new_v4().to_string()),
		user: make_thing("app_users", &Uuid::new_v4().to_string()),
		test: make_thing("app_tests", &Uuid::new_v4().to_string()),
		sub_test: None,
		session: make_thing("app_sessions", &Uuid::new_v4().to_string()),
		question: make_thing("app_questions", &Uuid::new_v4().to_string()),
		option: make_thing("app_options", &Uuid::new_v4().to_string()),
		is_correct: true,
		is_deleted: false,
		created_at: timestamp1,
		updated_at: timestamp2,
	};

	// Verify timestamps are ISO format (contains 'T' and 'Z')
	println!("Debug: created_at = '{}', updated_at = '{}'", schema.created_at, schema.updated_at);
	assert!(schema.created_at.contains('T'));
	assert!(schema.created_at.ends_with('Z') || schema.created_at.contains('+'));
	assert!(schema.updated_at.contains('T'));
	assert!(schema.updated_at.ends_with('Z') || schema.updated_at.contains('+'));
	
	// Verify they can be different
	assert_ne!(schema.created_at, schema.updated_at);
}