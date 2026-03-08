use crate::models::{CreateUploadFile, UploadFile};
use sqlx::postgres::PgPool;

pub struct UploadRepository {
    pool: PgPool,
}

impl UploadRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, data: CreateUploadFile) -> Result<UploadFile, sqlx::Error> {
        let id = cuid2::create_id();

        sqlx::query_as::<_, UploadFile>(
            r#"
            INSERT INTO upload_files (id, original_name, file_name, file_path, file_size, mime_type, file_url, storage_type)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING *
            "#,
        )
        .bind(&id)
        .bind(&data.original_name)
        .bind(&data.file_name)
        .bind(&data.file_path)
        .bind(data.file_size)
        .bind(&data.mime_type)
        .bind(&data.file_url)
        .bind(&data.storage_type)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn get_all(&self) -> Result<Vec<UploadFile>, sqlx::Error> {
        sqlx::query_as::<_, UploadFile>(
            "SELECT * FROM upload_files ORDER BY created_at DESC"
        )
        .fetch_all(&self.pool)
        .await
    }

    pub async fn get_by_id(&self, id: &str) -> Result<Option<UploadFile>, sqlx::Error> {
        sqlx::query_as::<_, UploadFile>("SELECT * FROM upload_files WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
    }

    pub async fn delete(&self, id: &str) -> Result<bool, sqlx::Error> {
        let result = sqlx::query("DELETE FROM upload_files WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn get_total_count(&self) -> Result<i64, sqlx::Error> {
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM upload_files")
            .fetch_one(&self.pool)
            .await?;
        Ok(count)
    }
}
