#[cfg(test)]
mod tests {
	use crate::v1::sessions::SessionWeightEnum;

	fn calculate_score(total_points: f32, multiplier: f64, weight: &str) -> i32 {
		let score = total_points as f64 * multiplier;
		let weight_value = SessionWeightEnum::to_float(weight);
		(weight_value * score).round() as i32
	}

	#[test]
	fn test_score_formula_basic_calculation() {
		let total_points = 10.0;
		let multiplier = 2.0;
		let weight = "25%";
		let result = calculate_score(total_points, multiplier, weight);
		assert_eq!(result, 5);
	}

	#[test]
	fn test_score_formula_with_multiple_questions() {
		let total_points = 40.0; // 15.0 + 25.0
		let multiplier = 1.5;
		let weight = "50%";
		let result = calculate_score(total_points, multiplier, weight);
		assert_eq!(result, 30);
	}

	#[test]
	fn test_score_formula_with_zero_weight() {
		let total_points = 100.0;
		let multiplier = 3.0;
		let weight = "0%";
		let result = calculate_score(total_points, multiplier, weight);
		assert_eq!(result, 0);
	}

	#[test]
	fn test_score_formula_with_decimal_points() {
		let total_points = 7.5;
		let multiplier = 1.2;
		let weight = "30%";
		let result = calculate_score(total_points, multiplier, weight);
		assert_eq!(result, 3);
	}

	#[test]
	fn test_score_formula_rounding_behavior() {
		let total_points = 3.3;
		let multiplier = 1.7;
		let weight = "35%";
		let result = calculate_score(total_points, multiplier, weight);
		assert_eq!(result, 2);
	}

	#[test]
	fn test_score_formula_with_max_weight_and_multiplier() {
		let total_points = 20.0;
		let multiplier = 5.0;
		let weight = "50%";
		let result = calculate_score(total_points, multiplier, weight);
		assert_eq!(result, 50);
	}

	#[test]
	fn test_score_formula_edge_case_zero_points() {
		let total_points = 0.0;
		let multiplier = 10.0;
		let weight = "45%";
		let result = calculate_score(total_points, multiplier, weight);
		assert_eq!(result, 0);
	}

	#[test]
	fn test_different_weight_percentages() {
		let total_points = 100.0;
		let multiplier = 1.0;

		assert_eq!(calculate_score(total_points, multiplier, "10%"), 10);
		assert_eq!(calculate_score(total_points, multiplier, "15%"), 15);
		assert_eq!(calculate_score(total_points, multiplier, "20%"), 20);
		assert_eq!(calculate_score(total_points, multiplier, "25%"), 25);
		assert_eq!(calculate_score(total_points, multiplier, "30%"), 30);
		assert_eq!(calculate_score(total_points, multiplier, "35%"), 35);
		assert_eq!(calculate_score(total_points, multiplier, "40%"), 40);
		assert_eq!(calculate_score(total_points, multiplier, "45%"), 45);
		assert_eq!(calculate_score(total_points, multiplier, "50%"), 50);
	}

	#[test]
	fn test_precision_and_rounding() {
		let total_points = 33.33;
		let multiplier = 1.5;
		let weight = "30%";
		let result = calculate_score(total_points, multiplier, weight);
		assert_eq!(result, 15);
	}

	#[test]
	fn test_large_numbers() {
		let total_points = 1000.0;
		let multiplier = 2.5;
		let weight = "40%";
		let result = calculate_score(total_points, multiplier, weight);
		assert_eq!(result, 1000);
	}

	#[test]
	fn test_fractional_multiplier() {
		let total_points = 10.0;
		let multiplier = 0.5;
		let weight = "20%";
		let result = calculate_score(total_points, multiplier, weight);
		assert_eq!(result, 1);
	}
}
