use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SessionWeightEnum {
	ZeroPercent,
	TenPercent,
	FifteenPercent,
	TwentyPercent,
	TwentyFivePercent,
	ThirtyPercent,
	ThirtyFivePercent,
	FortyPercent,
	FortyFivePercent,
	FiftyPercent,
}

impl fmt::Display for SessionWeightEnum {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let session_weight_str = match self {
			SessionWeightEnum::ZeroPercent => "0%",
			SessionWeightEnum::TenPercent => "10%",
			SessionWeightEnum::FifteenPercent => "15%",
			SessionWeightEnum::TwentyPercent => "20%",
			SessionWeightEnum::TwentyFivePercent => "25%",
			SessionWeightEnum::ThirtyPercent => "30%",
			SessionWeightEnum::ThirtyFivePercent => "35%",
			SessionWeightEnum::FortyPercent => "40%",
			SessionWeightEnum::FortyFivePercent => "45%",
			SessionWeightEnum::FiftyPercent => "50%",
		};

		write!(f, "{}", session_weight_str)
	}
}

impl SessionWeightEnum {
	pub fn to_float(value: &str) -> f64 {
		match value {
			"0%" => 0.0,
			"10%" => 0.1,
			"15%" => 0.15,
			"20%" => 0.2,
			"25%" => 0.25,
			"30%" => 0.3,
			"35%" => 0.35,
			"40%" => 0.4,
			"45%" => 0.45,
			"50%" => 0.5,
			_ => 0.0,
		}
	}
}
