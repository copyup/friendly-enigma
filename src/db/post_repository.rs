use crate::models::{CreatePost, Post, UpdatePost};
use sqlx::postgres::PgPool;

pub struct PostRepository {
    pool: PgPool,
}

impl PostRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, data: CreatePost) -> Result<Post, sqlx::Error> {
        let published_at = if data.status.as_deref() == Some("published") {
            Some(chrono::Utc::now())
        } else {
            None
        };

        sqlx::query_as::<_, Post>(
            r#"
            INSERT INTO posts (id, title, slug, content, excerpt, category_id, status, published_at)
            VALUES ($1, $2, $3, $4, $5, $6, COALESCE($7, 'draft'), $8)
            RETURNING *
            "#
        )
        .bind(cuid2::create_id())
        .bind(&data.title)
        .bind(&data.slug)
        .bind(&data.content)
        .bind(&data.excerpt)
        .bind(&data.category_id)
        .bind(&data.status)
        .bind(published_at)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn get_all(&self) -> Result<Vec<Post>, sqlx::Error> {
        sqlx::query_as::<_, Post>("SELECT * FROM posts ORDER BY created_at DESC")
            .fetch_all(&self.pool)
            .await
    }

    pub async fn get_published(&self) -> Result<Vec<Post>, sqlx::Error> {
        sqlx::query_as::<_, Post>(
            "SELECT * FROM posts WHERE status = 'published' ORDER BY published_at DESC"
        )
        .fetch_all(&self.pool)
        .await
    }

    pub async fn get_by_id(&self, id: &str) -> Result<Option<Post>, sqlx::Error> {
        sqlx::query_as::<_, Post>("SELECT * FROM posts WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
    }

    pub async fn get_by_slug(&self, slug: &str) -> Result<Option<Post>, sqlx::Error> {
        sqlx::query_as::<_, Post>("SELECT * FROM posts WHERE slug = $1")
            .bind(slug)
            .fetch_optional(&self.pool)
            .await
    }

    pub async fn update(&self, id: &str, data: UpdatePost) -> Result<Post, sqlx::Error> {
        sqlx::query_as::<_, Post>(
            r#"
            UPDATE posts 
            SET title = COALESCE($2, title),
                slug = COALESCE($3, slug),
                content = COALESCE($4, content),
                excerpt = COALESCE($5, excerpt),
                category_id = COALESCE($6, category_id),
                status = COALESCE($7, status),
                published_at = CASE 
                    WHEN COALESCE($7, status) = 'published' AND published_at IS NULL THEN NOW()
                    ELSE published_at
                END,
                updated_at = NOW()
            WHERE id = $1
            RETURNING *
            "#
        )
        .bind(id)
        .bind(data.title)
        .bind(data.slug)
        .bind(data.content)
        .bind(data.excerpt)
        .bind(data.category_id)
        .bind(data.status)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn delete(&self, id: &str) -> Result<bool, sqlx::Error> {
        let result = sqlx::query("DELETE FROM posts WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn get_by_category(&self, category_id: &str) -> Result<Vec<Post>, sqlx::Error> {
        sqlx::query_as::<_, Post>(
            "SELECT * FROM posts WHERE category_id = $1 ORDER BY created_at DESC"
        )
        .bind(category_id)
        .fetch_all(&self.pool)
        .await
    }

    /// 获取已发布文章数量
    pub async fn get_published_count(&self) -> Result<i64, sqlx::Error> {
        let count: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM posts WHERE status = 'published'"
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(count.0)
    }

    /// 分页获取已发布文章
    pub async fn get_published_paginated(&self, limit: i64, offset: i64) -> Result<Vec<Post>, sqlx::Error> {
        sqlx::query_as::<_, Post>(
            "SELECT * FROM posts WHERE status = 'published' ORDER BY published_at DESC LIMIT $1 OFFSET $2"
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
    }

    /// 获取最新的N篇已发布文章
    pub async fn get_published_recent(&self, limit: i64) -> Result<Vec<Post>, sqlx::Error> {
        sqlx::query_as::<_, Post>(
            "SELECT * FROM posts WHERE status = 'published' ORDER BY published_at DESC LIMIT $1"
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await
    }
}
