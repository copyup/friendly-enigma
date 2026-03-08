use crate::models::{CreatePage, Page, UpdatePage};
use sqlx::postgres::PgPool;

pub struct PageRepository {
    pool: PgPool,
}

impl PageRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, data: CreatePage) -> Result<Page, sqlx::Error> {
        sqlx::query_as::<_, Page>(
            r#"
            INSERT INTO pages (id, title, slug, content, is_published)
            VALUES ($1, $2, $3, $4, COALESCE($5, false))
            RETURNING *
            "#
        )
        .bind(cuid2::create_id())
        .bind(&data.title)
        .bind(&data.slug)
        .bind(&data.content)
        .bind(&data.is_published)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn get_all(&self) -> Result<Vec<Page>, sqlx::Error> {
        sqlx::query_as::<_, Page>("SELECT * FROM pages ORDER BY created_at DESC")
            .fetch_all(&self.pool)
            .await
    }

    pub async fn get_published(&self) -> Result<Vec<Page>, sqlx::Error> {
        sqlx::query_as::<_, Page>(
            "SELECT * FROM pages WHERE is_published = true ORDER BY created_at DESC"
        )
        .fetch_all(&self.pool)
        .await
    }

    pub async fn get_by_id(&self, id: &str) -> Result<Option<Page>, sqlx::Error> {
        sqlx::query_as::<_, Page>("SELECT * FROM pages WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
    }

    pub async fn get_by_slug(&self, slug: &str) -> Result<Option<Page>, sqlx::Error> {
        sqlx::query_as::<_, Page>("SELECT * FROM pages WHERE slug = $1")
            .bind(slug)
            .fetch_optional(&self.pool)
            .await
    }

    pub async fn update(&self, id: &str, data: UpdatePage) -> Result<Page, sqlx::Error> {
        sqlx::query_as::<_, Page>(
            r#"
            UPDATE pages 
            SET title = COALESCE($2, title),
                slug = COALESCE($3, slug),
                content = COALESCE($4, content),
                is_published = COALESCE($5, is_published),
                updated_at = NOW()
            WHERE id = $1
            RETURNING *
            "#
        )
        .bind(id)
        .bind(data.title)
        .bind(data.slug)
        .bind(data.content)
        .bind(data.is_published)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn delete(&self, id: &str) -> Result<bool, sqlx::Error> {
        let result = sqlx::query("DELETE FROM pages WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    /// 获取已发布页面数量
    pub async fn get_published_count(&self) -> Result<i64, sqlx::Error> {
        let count: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM pages WHERE is_published = true"
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(count.0)
    }
}
