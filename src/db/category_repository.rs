use crate::models::{Category, CategoryWithChildren, CreateCategory, UpdateCategory};
use sqlx::postgres::PgPool;

pub struct CategoryRepository {
    pool: PgPool,
}

impl CategoryRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, data: CreateCategory) -> Result<Category, sqlx::Error> {
        let category = sqlx::query_as::<_, Category>(
            r#"
            INSERT INTO categories (id, name, slug, description, parent_id)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING *
            "#
        )
        .bind(cuid2::create_id())
        .bind(&data.name)
        .bind(&data.slug)
        .bind(&data.description)
        .bind(&data.parent_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(category)
    }

    pub async fn get_all(&self) -> Result<Vec<Category>, sqlx::Error> {
        sqlx::query_as::<_, Category>("SELECT * FROM categories ORDER BY name")
            .fetch_all(&self.pool)
            .await
    }

    pub async fn get_by_id(&self, id: &str) -> Result<Option<Category>, sqlx::Error> {
        sqlx::query_as::<_, Category>("SELECT * FROM categories WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
    }

    pub async fn get_by_slug(&self, slug: &str) -> Result<Option<Category>, sqlx::Error> {
        sqlx::query_as::<_, Category>("SELECT * FROM categories WHERE slug = $1")
            .bind(slug)
            .fetch_optional(&self.pool)
            .await
    }

    pub async fn update(&self, id: &str, data: UpdateCategory) -> Result<Category, sqlx::Error> {
        sqlx::query_as::<_, Category>(
            r#"
            UPDATE categories 
            SET name = COALESCE($2, name),
                slug = COALESCE($3, slug),
                description = COALESCE($4, description),
                parent_id = COALESCE($5, parent_id),
                updated_at = NOW()
            WHERE id = $1
            RETURNING *
            "#
        )
        .bind(id)
        .bind(data.name)
        .bind(data.slug)
        .bind(data.description)
        .bind(data.parent_id)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn delete(&self, id: &str) -> Result<bool, sqlx::Error> {
        let result = sqlx::query("DELETE FROM categories WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn get_tree(&self) -> Result<Vec<CategoryWithChildren>, sqlx::Error> {
        // 获取所有顶级分类
        let root_categories = sqlx::query_as::<_, Category>(
            "SELECT * FROM categories WHERE parent_id IS NULL ORDER BY name"
        )
        .fetch_all(&self.pool)
        .await?;

        let mut tree = Vec::new();
        for category in root_categories {
            let children = self.get_children_recursive(&category.id).await?;
            tree.push(CategoryWithChildren {
                category,
                children,
            });
        }

        Ok(tree)
    }

    async fn get_children_recursive(&self, parent_id: &str) -> Result<Vec<CategoryWithChildren>, sqlx::Error> {
        let children = sqlx::query_as::<_, Category>(
            "SELECT * FROM categories WHERE parent_id = $1 ORDER BY name"
        )
        .bind(parent_id)
        .fetch_all(&self.pool)
        .await?;

        let mut result = Vec::new();
        for child in children {
            let grandchildren = Box::pin(self.get_children_recursive(child.id.as_str())).await?;
            result.push(CategoryWithChildren {
                category: child,
                children: grandchildren,
            });
        }

        Ok(result)
    }
}
