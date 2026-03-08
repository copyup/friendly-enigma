mod category_repository;
mod page_repository;
mod post_repository;
mod upload_repository;

use sqlx::postgres::{PgPool, PgPoolOptions};
use std::time::Duration;

pub use category_repository::CategoryRepository;
pub use page_repository::PageRepository;
pub use post_repository::PostRepository;
pub use upload_repository::UploadRepository;

pub async fn create_pool(database_url: &str) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .idle_timeout(Duration::from_secs(3))
        .min_connections(1)
        .max_connections(30)
        .max_lifetime(Duration::from_secs(60 * 60))
        .connect_lazy(database_url)
}
