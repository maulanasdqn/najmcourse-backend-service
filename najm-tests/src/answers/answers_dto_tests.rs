use najm_exam::{
	AnswerEntryDto, AnswersCreateAkademikRequestDto, AnswersCreatePsikologiRequestDto,
	OptionsItemAnswersDto, QuestionsItemAnswersDto, TestsItemAnswersDto,
};
use serde_json;
use validator::Validate;

#[tokio::test]
async fn test_answers_create_akademik_request_dto_validation_success() {
	let dto = AnswersCreateAkademikRequestDto {
		user_id: "user-123".to_string(),
		test_id: "test-456".to_string(),
		session_id: "session-789".to_string(),
		answers: vec![AnswerEntryDto {
			question_id: "question-001".to_string(),
			option_id: "option-001".to_string(),
		}],
	};

	let validation_result = dto.validate();
	assert!(validation_result.is_ok(), "Valid DTO should pass validation");
}

#[tokio::test]
async fn test_answers_create_akademik_request_dto_validation_empty_user_id() {
	let dto = AnswersCreateAkademikRequestDto {
		user_id: "".to_string(), // Empty user_id should fail
		test_id: "test-456".to_string(),
		session_id: "session-789".to_string(),
		answers: vec![AnswerEntryDto {
			question_id: "question-001".to_string(),
			option_id: "option-001".to_string(),
		}],
	};

	let validation_result = dto.validate();
	assert!(validation_result.is_err(), "Empty user_id should fail validation");
	
	let errors = validation_result.unwrap_err();
	assert!(errors.field_errors().contains_key("user_id"));
}

#[tokio::test]
async fn test_answers_create_akademik_request_dto_validation_empty_test_id() {
	let dto = AnswersCreateAkademikRequestDto {
		user_id: "user-123".to_string(),
		test_id: "".to_string(), // Empty test_id should fail
		session_id: "session-789".to_string(),
		answers: vec![AnswerEntryDto {
			question_id: "question-001".to_string(),
			option_id: "option-001".to_string(),
		}],
	};

	let validation_result = dto.validate();
	assert!(validation_result.is_err(), "Empty test_id should fail validation");
	
	let errors = validation_result.unwrap_err();
	assert!(errors.field_errors().contains_key("test_id"));
}

#[tokio::test]
async fn test_answers_create_akademik_request_dto_validation_empty_session_id() {
	let dto = AnswersCreateAkademikRequestDto {
		user_id: "user-123".to_string(),
		test_id: "test-456".to_string(),
		session_id: "".to_string(), // Empty session_id should fail
		answers: vec![AnswerEntryDto {
			question_id: "question-001".to_string(),
			option_id: "option-001".to_string(),
		}],
	};

	let validation_result = dto.validate();
	assert!(validation_result.is_err(), "Empty session_id should fail validation");
	
	let errors = validation_result.unwrap_err();
	assert!(errors.field_errors().contains_key("session_id"));
}

#[tokio::test]
async fn test_answers_create_akademik_request_dto_validation_empty_answers() {
	let dto = AnswersCreateAkademikRequestDto {
		user_id: "user-123".to_string(),
		test_id: "test-456".to_string(),
		session_id: "session-789".to_string(),
		answers: vec![], // Empty answers should fail
	};

	let validation_result = dto.validate();
	assert!(validation_result.is_err(), "Empty answers should fail validation");
	
	let errors = validation_result.unwrap_err();
	assert!(errors.field_errors().contains_key("answers"));
}

#[tokio::test]
async fn test_answers_create_psikologi_request_dto_validation_success() {
	let dto = AnswersCreatePsikologiRequestDto {
		user_id: "user-123".to_string(),
		test_id: "test-456".to_string(),
		sub_test_id: "subtest-111".to_string(),
		session_id: "session-789".to_string(),
		answers: vec![AnswerEntryDto {
			question_id: "question-001".to_string(),
			option_id: "option-001".to_string(),
		}],
	};

	let validation_result = dto.validate();
	assert!(validation_result.is_ok(), "Valid psikologi DTO should pass validation");
}

#[tokio::test]
async fn test_answers_create_psikologi_request_dto_validation_empty_sub_test_id() {
	let dto = AnswersCreatePsikologiRequestDto {
		user_id: "user-123".to_string(),
		test_id: "test-456".to_string(),
		sub_test_id: "".to_string(), // Empty sub_test_id should fail
		session_id: "session-789".to_string(),
		answers: vec![AnswerEntryDto {
			question_id: "question-001".to_string(),
			option_id: "option-001".to_string(),
		}],
	};

	let validation_result = dto.validate();
	assert!(validation_result.is_err(), "Empty sub_test_id should fail validation");
	
	let errors = validation_result.unwrap_err();
	assert!(errors.field_errors().contains_key("sub_test_id"));
}

#[tokio::test]
async fn test_answer_entry_dto_validation_success() {
	let dto = AnswerEntryDto {
		question_id: "question-001".to_string(),
		option_id: "option-001".to_string(),
	};

	let validation_result = dto.validate();
	assert!(validation_result.is_ok(), "Valid answer entry should pass validation");
}

#[tokio::test]
async fn test_answer_entry_dto_validation_empty_question_id() {
	let dto = AnswerEntryDto {
		question_id: "".to_string(), // Empty question_id should fail
		option_id: "option-001".to_string(),
	};

	let validation_result = dto.validate();
	assert!(validation_result.is_err(), "Empty question_id should fail validation");
	
	let errors = validation_result.unwrap_err();
	assert!(errors.field_errors().contains_key("question_id"));
}

#[tokio::test]
async fn test_answer_entry_dto_validation_empty_option_id() {
	let dto = AnswerEntryDto {
		question_id: "question-001".to_string(),
		option_id: "".to_string(), // Empty option_id should fail
	};

	let validation_result = dto.validate();
	assert!(validation_result.is_err(), "Empty option_id should fail validation");
	
	let errors = validation_result.unwrap_err();
	assert!(errors.field_errors().contains_key("option_id"));
}

#[tokio::test]
async fn test_answers_create_akademik_request_dto_serialization() {
	let dto = AnswersCreateAkademikRequestDto {
		user_id: "user-123".to_string(),
		test_id: "test-456".to_string(),
		session_id: "session-789".to_string(),
		answers: vec![
			AnswerEntryDto {
				question_id: "question-001".to_string(),
				option_id: "option-001".to_string(),
			},
			AnswerEntryDto {
				question_id: "question-002".to_string(),
				option_id: "option-002".to_string(),
			},
		],
	};

	let json_result = serde_json::to_string(&dto);
	assert!(json_result.is_ok(), "DTO should serialize to JSON");
	
	let json = json_result.unwrap();
	assert!(json.contains("user-123"));
	assert!(json.contains("test-456"));
	assert!(json.contains("session-789"));
	assert!(json.contains("question-001"));
	assert!(json.contains("option-001"));
}

#[tokio::test]
async fn test_answers_create_akademik_request_dto_deserialization() {
	let json = r#"{
		"user_id": "user-123",
		"test_id": "test-456", 
		"session_id": "session-789",
		"answers": [
			{
				"question_id": "question-001",
				"option_id": "option-001"
			}
		]
	}"#;

	let dto_result: Result<AnswersCreateAkademikRequestDto, _> = serde_json::from_str(json);
	assert!(dto_result.is_ok(), "Valid JSON should deserialize to DTO");
	
	let dto = dto_result.unwrap();
	assert_eq!(dto.user_id, "user-123");
	assert_eq!(dto.test_id, "test-456");
	assert_eq!(dto.session_id, "session-789");
	assert_eq!(dto.answers.len(), 1);
	assert_eq!(dto.answers[0].question_id, "question-001");
	assert_eq!(dto.answers[0].option_id, "option-001");
}

#[tokio::test]
async fn test_answers_create_psikologi_request_dto_serialization() {
	let dto = AnswersCreatePsikologiRequestDto {
		user_id: "user-456".to_string(),
		test_id: "test-789".to_string(),
		sub_test_id: "subtest-123".to_string(),
		session_id: "session-456".to_string(),
		answers: vec![AnswerEntryDto {
			question_id: "question-psiko".to_string(),
			option_id: "option-psiko".to_string(),
		}],
	};

	let json_result = serde_json::to_string(&dto);
	assert!(json_result.is_ok(), "Psikologi DTO should serialize to JSON");
	
	let json = json_result.unwrap();
	assert!(json.contains("user-456"));
	assert!(json.contains("subtest-123"));
	assert!(json.contains("question-psiko"));
}

#[tokio::test]
async fn test_answers_create_psikologi_request_dto_deserialization() {
	let json = r#"{
		"user_id": "user-456",
		"test_id": "test-789",
		"sub_test_id": "subtest-123", 
		"session_id": "session-456",
		"answers": [
			{
				"question_id": "question-psiko",
				"option_id": "option-psiko"
			}
		]
	}"#;

	let dto_result: Result<AnswersCreatePsikologiRequestDto, _> = serde_json::from_str(json);
	assert!(dto_result.is_ok(), "Valid psikologi JSON should deserialize to DTO");
	
	let dto = dto_result.unwrap();
	assert_eq!(dto.user_id, "user-456");
	assert_eq!(dto.test_id, "test-789");
	assert_eq!(dto.sub_test_id, "subtest-123");
	assert_eq!(dto.session_id, "session-456");
	assert_eq!(dto.answers.len(), 1);
}

#[tokio::test]
async fn test_options_item_answers_dto_serialization() {
	let dto = OptionsItemAnswersDto {
		id: "option-123".to_string(),
		label: "Option A".to_string(),
		is_correct: true,
		points: Some(15.5),
		is_user_selected: false,
		image_url: Some("https://example.com/option.jpg".to_string()),
		created_at: "2025-01-01T00:00:00Z".to_string(),
		updated_at: "2025-01-01T12:00:00Z".to_string(),
	};

	let json_result = serde_json::to_string(&dto);
	assert!(json_result.is_ok(), "Options item DTO should serialize");
	
	let json = json_result.unwrap();
	assert!(json.contains("option-123"));
	assert!(json.contains("Option A"));
	assert!(json.contains("true")); // is_correct
	assert!(json.contains("15.5")); // points
	assert!(json.contains("false")); // is_user_selected
}

#[tokio::test]
async fn test_options_item_answers_dto_deserialization() {
	let json = r#"{
		"id": "option-456",
		"label": "Option B",
		"is_correct": false,
		"points": null,
		"is_user_selected": true,
		"image_url": null,
		"created_at": "2025-01-02T00:00:00Z",
		"updated_at": "2025-01-02T12:00:00Z"
	}"#;

	let dto_result: Result<OptionsItemAnswersDto, _> = serde_json::from_str(json);
	assert!(dto_result.is_ok(), "Valid options JSON should deserialize");
	
	let dto = dto_result.unwrap();
	assert_eq!(dto.id, "option-456");
	assert_eq!(dto.label, "Option B");
	assert!(!dto.is_correct);
	assert!(dto.points.is_none());
	assert!(dto.is_user_selected);
	assert!(dto.image_url.is_none());
}

#[tokio::test]
async fn test_questions_item_answers_dto_serialization() {
	let dto = QuestionsItemAnswersDto {
		id: "question-123".to_string(),
		question: "What is the capital of France?".to_string(),
		discussion: "Paris is the capital and largest city of France.".to_string(),
		question_image_url: Some("https://example.com/question.jpg".to_string()),
		discussion_image_url: None,
		options: vec![
			OptionsItemAnswersDto {
				id: "option-1".to_string(),
				label: "Paris".to_string(),
				is_correct: true,
				points: Some(10.0),
				is_user_selected: true,
				image_url: None,
				created_at: "2025-01-01T00:00:00Z".to_string(),
				updated_at: "2025-01-01T00:00:00Z".to_string(),
			},
		],
		created_at: "2025-01-01T00:00:00Z".to_string(),
		updated_at: "2025-01-01T00:00:00Z".to_string(),
	};

	let json_result = serde_json::to_string(&dto);
	assert!(json_result.is_ok(), "Questions item DTO should serialize");
	
	let json = json_result.unwrap();
	assert!(json.contains("question-123"));
	assert!(json.contains("What is the capital of France?"));
	assert!(json.contains("Paris is the capital"));
	assert!(json.contains("Paris"));
}

#[tokio::test]
async fn test_tests_item_answers_dto_serialization() {
	let dto = TestsItemAnswersDto {
		id: "answer-123".to_string(),
		name: "Geography Test".to_string(),
		score: 85,
		passing_grade: Some(70.0),
		questions: vec![
			QuestionsItemAnswersDto {
				id: "question-1".to_string(),
				question: "Test question".to_string(),
				discussion: "Test discussion".to_string(),
				question_image_url: None,
				discussion_image_url: None,
				options: vec![],
				created_at: "2025-01-01T00:00:00Z".to_string(),
				updated_at: "2025-01-01T00:00:00Z".to_string(),
			},
		],
		created_at: "2025-01-01T00:00:00Z".to_string(),
		updated_at: "2025-01-01T12:00:00Z".to_string(),
	};

	let json_result = serde_json::to_string(&dto);
	assert!(json_result.is_ok(), "Tests item DTO should serialize");
	
	let json = json_result.unwrap();
	assert!(json.contains("answer-123"));
	assert!(json.contains("Geography Test"));
	assert!(json.contains("85")); // score
	assert!(json.contains("70")); // passing_grade
}

#[tokio::test]
async fn test_multiple_answer_entries_validation() {
	let dto = AnswersCreateAkademikRequestDto {
		user_id: "user-123".to_string(),
		test_id: "test-456".to_string(),
		session_id: "session-789".to_string(),
		answers: vec![
			AnswerEntryDto {
				question_id: "question-001".to_string(),
				option_id: "option-001".to_string(),
			},
			AnswerEntryDto {
				question_id: "question-002".to_string(),
				option_id: "option-002".to_string(),
			},
			AnswerEntryDto {
				question_id: "question-003".to_string(),
				option_id: "option-003".to_string(),
			},
		],
	};

	let validation_result = dto.validate();
	assert!(validation_result.is_ok(), "Multiple valid answers should pass validation");
	assert_eq!(dto.answers.len(), 3);
}

#[tokio::test]
async fn test_dto_clone_functionality() {
	let original = AnswersCreateAkademikRequestDto {
		user_id: "user-123".to_string(),
		test_id: "test-456".to_string(),
		session_id: "session-789".to_string(),
		answers: vec![AnswerEntryDto {
			question_id: "question-001".to_string(),
			option_id: "option-001".to_string(),
		}],
	};

	let cloned = original.clone();
	assert_eq!(original.user_id, cloned.user_id);
	assert_eq!(original.test_id, cloned.test_id);
	assert_eq!(original.session_id, cloned.session_id);
	assert_eq!(original.answers.len(), cloned.answers.len());
	assert_eq!(original.answers[0].question_id, cloned.answers[0].question_id);
}

#[tokio::test]
async fn test_dto_debug_functionality() {
	let dto = AnswerEntryDto {
		question_id: "question-debug".to_string(),
		option_id: "option-debug".to_string(),
	};

	let debug_string = format!("{:?}", dto);
	assert!(debug_string.contains("question-debug"));
	assert!(debug_string.contains("option-debug"));
	assert!(debug_string.contains("AnswerEntryDto"));
}