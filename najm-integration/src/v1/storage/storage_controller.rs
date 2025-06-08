use super::StorageState;
use super::{StorageRepository, StorageRequestDto, StorageResponseDto};
use crate::MessageResponseDto;
use axum::{
	extract::{Multipart, State},
	response::Response,
};

#[utoipa::path(
    post,
    path = "/v1/storage/upload",
    request_body(
        content = StorageRequestDto,
        content_type = "multipart/form-data"
    ),
    security(
        ("Bearer" = [])
    ),
    responses(
        (status = 201, description = "File Uploaded", body = StorageResponseDto),
        (status = 400, description = "Failed to upload file", body = MessageResponseDto)
    ),
    tag = "Storage"
)]
pub async fn post_upload(
	State(state): State<StorageState>,
	payload: Multipart,
) -> Response {
	let storage_repo = StorageRepository::new(&state);
	storage_repo.mutation_upload_file(payload).await
}
