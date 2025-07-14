# Student Dashboard Stats Endpoint

## Overview

This endpoint provides comprehensive statistics for the student dashboard, including average scores per month and a list of tests already taken by the student.

## Endpoint

```
GET /v1/sessions/student-stats/{user_id}
```

## Authentication

Requires Bearer token authentication with `ReadDetailSessions` permission.

## Parameters

- `user_id` (path parameter): The ID of the user/student

## Response

```json
{
  "data": {
    "average_score_per_month": [
      {
        "month": "2024-01",
        "average_score": 85.5,
        "tests_count": 5
      },
      {
        "month": "2024-02",
        "average_score": 78.2,
        "tests_count": 3
      }
    ],
    "tests_taken": [
      {
        "test_id": "uuid",
        "test_name": "Tryout Saintek 2025",
        "category": "Saintek",
        "score": 85,
        "taken_at": "2024-01-15T10:30:00Z",
        "session_id": "uuid",
        "session_name": "Tryout Saintek 2025"
      }
    ],
    "total_tests_taken": 8,
    "average_score_overall": 82.1
  }
}
```

## Response Fields

### `average_score_per_month`
Array of monthly score statistics:
- `month`: Month in YYYY-MM format
- `average_score`: Average score for that month
- `tests_count`: Number of tests taken in that month

### `tests_taken`
Array of tests the student has completed:
- `test_id`: Unique identifier for the test
- `test_name`: Name of the test
- `category`: Category of the test (e.g., "Saintek", "Soshum")
- `score`: Student's score on the test
- `taken_at`: Timestamp when the test was taken
- `session_id`: ID of the session the test belongs to
- `session_name`: Name of the session

### `total_tests_taken`
Total number of tests the student has completed.

### `average_score_overall`
Overall average score across all tests taken by the student.

## Implementation Details

### Files Modified

1. **`sessions_dto.rs`**: Added new DTOs for the response
   - `StudentStatsResponseDto`
   - `MonthlyScoreDto`
   - `TestTakenDto`

2. **`sessions_controller.rs`**: Added new controller function
   - `get_student_stats()`: Handles the HTTP request

3. **`sessions_service.rs`**: Added new service function
   - `get_student_stats()`: Business logic for processing the request

4. **`sessions_repository.rs`**: Added new repository function
   - `query_student_stats()`: Database query to fetch student statistics

5. **`sessions/mod.rs`**: Added new route
   - `/student-stats/{user_id}`: Route definition

### Database Query

The endpoint queries the `app_answers` table to get all answers submitted by the student, then:

1. Groups answers by test to calculate individual test scores
2. Groups answers by month to calculate monthly averages
3. Calculates overall statistics

### Scoring Logic

The current implementation uses a simplified scoring system:
- Correct answers: 1 point
- Incorrect answers: 0 points

This can be enhanced to use the actual scoring logic from the test configuration (weights, multipliers, etc.).

## Usage Example

```bash
curl -X GET "http://localhost:8000/v1/sessions/student-stats/user123" \
  -H "Authorization: Bearer your-token-here"
```

## Error Handling

- **404 Not Found**: If the user doesn't exist or has no test data
- **401 Unauthorized**: If authentication fails
- **403 Forbidden**: If the user doesn't have the required permissions
- **500 Internal Server Error**: For database or other server errors

## Future Enhancements

1. **Enhanced Scoring**: Use actual test scoring logic with weights and multipliers
2. **Performance Optimization**: Add caching for frequently accessed data
3. **Filtering**: Add date range filters for the statistics
4. **Pagination**: Add pagination for large datasets
5. **Real-time Updates**: Consider WebSocket updates for real-time dashboard updates