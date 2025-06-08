use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct StorageRequestDto {
	file: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct StorageResponseDto {
	pub file_url: String,
}
