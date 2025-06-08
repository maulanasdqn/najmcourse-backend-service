use super::StorageResponseDto;
use crate::{common_response, success_response, ResponseSuccessDto, StorageState};
use axum::{extract::Multipart, http::StatusCode, response::Response};
use log::error;

pub struct StorageRepository<'a> {
	pub state: &'a StorageState,
}

impl<'a> StorageRepository<'a> {
	pub fn new(state: &'a StorageState) -> Self {
		Self { state }
	}

	pub async fn mutation_upload_file(&self, mut multipart: Multipart) -> Response {
		while let Some(field) = match multipart.next_field().await {
			Ok(field) => field,
			Err(e) => {
				error!("Failed to parse multipart field: {}", e);
				return common_response(StatusCode::BAD_REQUEST, "Invalid multipart data");
			}
		} {
			if let Some(file_name) = field.file_name().map(|name| name.to_string()) {
				let data = match field.bytes().await {
					Ok(data) => data,
					Err(e) => {
						error!("Failed to read file bytes: {}", e);
						return common_response(
							StatusCode::BAD_REQUEST,
							"Failed to read file data",
						);
					}
				};
				let minio_client = self.state.minio.lock().await;
				match minio_client.upload_file(&file_name, data.to_vec()).await {
					Ok(file_url) => {
						let response = ResponseSuccessDto {
							data: StorageResponseDto { file_url },
						};
						return success_response(response);
					}
					Err(e) => {
						error!("Failed to upload file to storage: {}", e);
						return common_response(
							StatusCode::INTERNAL_SERVER_ERROR,
							&e.to_string(),
						);
					}
				}
			} else {
				error!("File name missing in the multipart field");
				return common_response(StatusCode::BAD_REQUEST, "File name is required");
			}
		}
		common_response(StatusCode::BAD_REQUEST, "Invalid file upload request")
	}
}
