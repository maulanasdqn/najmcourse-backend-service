# Score Formula Documentation

## Overview

The score calculation formula is used to determine the final score for exam answers based on points earned, test multipliers, and session weights.

## Formula

```
Final Score = round(weight * (total_points * multiplier))
```

Where:

- **total_points**: Sum of points from all user-selected options
- **multiplier**: Test multiplier from session configuration
- **weight**: Session weight percentage converted to float (e.g., "25%" → 0.25)

## Components

### 1. Total Points

The sum of all points from user-selected options in the test.

### 2. Multiplier

A multiplier value assigned to each test in a session. This allows for adjusting the relative importance or difficulty of different tests.

### 3. Weight

The percentage weight of the test within the session, converted to a decimal value. Available weights:

- 0% → 0.0
- 10% → 0.1
- 15% → 0.15
- 20% → 0.2
- 25% → 0.25
- 30% → 0.3
- 35% → 0.35
- 40% → 0.4
- 45% → 0.45
- 50% → 0.5

## Calculation Steps

1. Calculate intermediate score: `total_points * multiplier`
2. Apply weight: `weight * intermediate_score`
3. Round to nearest integer: `round(weighted_score)`
4. Convert to integer: `as i32`

## Examples

### Example 1: Basic Calculation

- Total Points: 10.0
- Multiplier: 2.0
- Weight: "25%" (0.25)
- Result: round(0.25 _ (10.0 _ 2.0)) = round(0.25 \* 20.0) = round(5.0) = 5

### Example 2: Multiple Questions

- Total Points: 40.0 (15.0 + 25.0)
- Multiplier: 1.5
- Weight: "50%" (0.5)
- Result: round(0.5 _ (40.0 _ 1.5)) = round(0.5 \* 60.0) = round(30.0) = 30

### Example 3: Decimal Rounding

- Total Points: 3.3
- Multiplier: 1.7
- Weight: "35%" (0.35)
- Result: round(0.35 _ (3.3 _ 1.7)) = round(0.35 \* 5.61) = round(1.9635) = 2

## Test Coverage

The score formula is tested with the following scenarios:

- Basic calculations
- Multiple questions
- Zero weight edge cases
- Decimal point handling
- Rounding behavior
- Maximum weight and multiplier values
- Zero points edge cases
- Different weight percentages
- Precision and rounding
- Large numbers
- Fractional multipliers

## Implementation

The formula is implemented in `najm-exam/src/v1/answers/answers_repository.rs` within the following methods:

- `query_test_with_answers`
- `query_test_sub_test_with_answers`
- `query_create_akademik`
- `query_create_psikologi`
