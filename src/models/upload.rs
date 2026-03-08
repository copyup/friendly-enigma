use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct UploadFile {
    pub id: String,
    pub original_name: String,
    pub file_name: String,
    pub file_path: String,
    pub file_size: i64,
    pub mime_type: String,
    pub file_url: String,
    pub storage_type: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUploadFile {
    pub original_name: String,
    pub file_name: String,
    pub file_path: String,
    pub file_size: i64,
    pub mime_type: String,
    pub file_url: String,
    pub storage_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UploadResponse {
    pub id: String,
    pub original_name: String,
    pub file_url: String,
    pub file_size: i64,
    pub mime_type: String,
}

impl From<UploadFile> for UploadResponse {
    fn from(file: UploadFile) -> Self {
        Self {
            id: file.id,
            original_name: file.original_name,
            file_url: file.file_url,
            file_size: file.file_size,
            mime_type: file.mime_type,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileListResponse {
    pub files: Vec<UploadFile>,
    pub total: i64,
}
