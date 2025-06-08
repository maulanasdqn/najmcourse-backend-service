use crate::Env;
use bytes::Bytes;
use log::{error, info};
use mime_guess::from_path;
use minio_rsc::client::Minio;
use minio_rsc::error::Error;
use minio_rsc::provider::StaticProvider;
use std::sync::Arc;
use surrealdb::sql::Uuid;
use tokio::sync::Mutex;

pub async fn storage_state() -> Result<StorageState, Box<dyn std::error::Error>> {
	let minio_client = MinioClient::new().await?;
	Ok(StorageState {
		minio: Arc::new(Mutex::new(minio_client)),
	})
}

#[derive(Clone)]
pub struct MinioClient {
	client: Arc<Minio>,
	bucket_name: String,
}

#[derive(Clone)]
pub struct StorageState {
	pub minio: Arc<Mutex<MinioClient>>,
}

const ALLOWED_IMAGE_EXTENSIONS: &[&str] =
	&["jpg", "jpeg", "png", "gif", "bmp", "webp"];
const ALLOWED_IMAGE_MIME_TYPES: &[&str] = &[
	"image/jpeg",
	"image/png",
	"image/gif",
	"image/bmp",
	"image/webp",
];

impl MinioClient {
	pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
		let config = Env::new();
		let bucket_name = config.minio_bucket_name;
		let endpoint = config.minio_endpoint;
		let access_key = config.minio_access_key;
		let secret_key = config.minio_secret_key;
		let provider = StaticProvider::new(&access_key, &secret_key, None);
		let client = Minio::builder()
			.endpoint(&endpoint)
			.provider(provider)
			.secure(true)
			.build()?;
		if !client.bucket_exists(&bucket_name).await? {
			client.make_bucket(&bucket_name, false).await?;
			info!("Bucket `{}` created successfully.", bucket_name);
		}
		Ok(Self {
			client: Arc::new(client),
			bucket_name,
		})
	}

	pub async fn upload_file(
		&self,
		original_filename: &str,
		data: Vec<u8>,
	) -> Result<String, Error> {
		let config = Env::new();
		let sanitized_filename = original_filename
			.chars()
			.filter(|c| c.is_alphanumeric() || *c == '.' || *c == '_' || *c == '-')
			.collect::<String>();
		let file_ext = std::path::Path::new(&sanitized_filename)
			.extension()
			.and_then(|ext| ext.to_str())
			.unwrap_or("");
		if !ALLOWED_IMAGE_EXTENSIONS.contains(&file_ext.to_lowercase().as_str()) {
			error!("Invalid file extension: {}", file_ext);
			return Err(Error::from("Unsupported file extension"));
		}
		let mime_guess = from_path(&sanitized_filename).first_or_octet_stream();
		let mime_type = mime_guess.essence_str();
		if !ALLOWED_IMAGE_MIME_TYPES.contains(&mime_type) {
			error!("Invalid MIME type: {}", mime_type);
			return Err(Error::from("Unsupported file type"));
		}
		let unique_filename = format!(
			"{}-{}.{}",
			Uuid::new_v4(),
			sanitized_filename.replace('.', "_"),
			file_ext
		);
		info!(
			"Uploading file: {} ({} KB)",
			unique_filename,
			data.len() / 1024
		);
		let endpoint = config.minio_endpoint;
		self
			.client
			.put_object(&self.bucket_name, &unique_filename, Bytes::from(data))
			.await?;
		let file_url = format!(
			"https://{}/{}/{}",
			endpoint, self.bucket_name, unique_filename
		);
		Ok(file_url)
	}
}
