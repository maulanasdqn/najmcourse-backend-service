use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuthOtpSchema {
	pub otp: String,
	pub expires_at: DateTime<Utc>,
}
