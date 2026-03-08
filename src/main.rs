mod db;
mod handlers;
mod middleware;
mod mcp;
mod models;
mod services;
mod templates;
mod utils;

use actix_web::{web, App, HttpServer, middleware as actix_middleware};
use actix_files as fs;
use dotenvy::dotenv;
use std::env;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::filter::LevelFilter;
use db::{create_pool, CategoryRepository, PageRepository, PostRepository, UploadRepository};
use handlers::*;
use mcp::handler::handle_mcp_request;
use middleware::api_key_auth::ApiKeyAuth;
use services::StorageService;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        EnvFilter::builder()
            .with_default_directive(LevelFilter::DEBUG.into())
            .from_env_lossy()
    });
    tracing_subscriber::fmt().with_env_filter(filter).init();
    dotenv().unwrap_or_default();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    
    // 获取 API Key，如果没有设置则生成一个默认的（仅用于开发环境）
    let api_key = env::var("API_KEY").unwrap_or_else(|_| {
        let default_key = "your-secure-api-key-change-in-production".to_string();
        tracing::warn!("API_KEY not set, using default key. Please set API_KEY in production!");
        default_key
    });

    tracing::info!("正在连接数据库...");
    let pool = create_pool(&database_url).await
        .expect("Failed to create pool");

    tracing::info!("正在初始化存储服务...");
    let storage = web::Data::new(StorageService::new().expect("Failed to create storage service"));

    tracing::info!("启动服务器于 http://0.0.0.0:8080");

    // 创建仓库实例
    let post_repo = web::Data::new(PostRepository::new(pool.clone()));
    let page_repo = web::Data::new(PageRepository::new(pool.clone()));
    let category_repo = web::Data::new(CategoryRepository::new(pool.clone()));
    let upload_repo = web::Data::new(UploadRepository::new(pool.clone()));

    // 创建 API Key 认证中间件
    let api_key_auth = ApiKeyAuth::new(api_key);

    HttpServer::new(move || {
        App::new()
            .wrap(actix_middleware::Logger::default())
            .app_data(post_repo.clone())
            .app_data(page_repo.clone())
            .app_data(category_repo.clone())
            .app_data(upload_repo.clone())
            .app_data(storage.clone())
            .service(fs::Files::new("/static", "./static").show_files_listing())
            .service(fs::Files::new("/uploads", "./uploads").show_files_listing())
            .route("/robots.txt", web::get().to(seo_handler::robots_txt))
            .route("/sitemap.xml", web::get().to(seo_handler::sitemap_index))
            .route("/sitemap-index.xml", web::get().to(seo_handler::sitemap_index))
            .route("/sitemap-static.xml", web::get().to(seo_handler::sitemap_static))
            .route("/sitemap-posts-{page}.xml", web::get().to(seo_handler::sitemap_posts))
            .route("/sitemap-pages.xml", web::get().to(seo_handler::sitemap_pages))
            .route("/rss.xml", web::get().to(seo_handler::rss_xml))
            .route("/atom.xml", web::get().to(seo_handler::atom_xml))
            // 前端展示页面（公开访问）
            .route("/", web::get().to(view_handler::index))
            .route("/posts", web::get().to(view_handler::posts_list))
            .route("/posts/{slug}", web::get().to(view_handler::post_detail))
            .route("/categories", web::get().to(view_handler::categories_list))
            .route("/pages/{slug}", web::get().to(view_handler::page_detail))
            // RESTful API - 公开接口（只读操作，不需要认证）
            .route("/api/posts/published", web::get().to(post_handler::get_published_posts))
            .route("/api/posts/{id}", web::get().to(post_handler::get_post))
            .route("/api/posts/slug/{slug}", web::get().to(post_handler::get_post_by_slug))
            .route("/api/posts/{id}/html", web::get().to(post_handler::render_post_html))
            .route("/api/pages/published", web::get().to(page_handler::get_published_pages))
            .route("/api/pages/{id}", web::get().to(page_handler::get_page))
            .route("/api/pages/slug/{slug}", web::get().to(page_handler::get_page_by_slug))
            .route("/api/pages/{id}/html", web::get().to(page_handler::render_page_html))
            .route("/api/categories", web::get().to(category_handler::get_categories))
            .route("/api/categories/tree", web::get().to(category_handler::get_category_tree))
            .route("/api/categories/{id}", web::get().to(category_handler::get_category))
            // RESTful API - 需要认证的接口（管理操作）
            .service(
                web::scope("/api/admin")
                    .wrap(api_key_auth.clone())
                    // 文章管理
                    .route("/posts", web::get().to(post_handler::get_posts))
                    .route("/posts", web::post().to(post_handler::create_post))
                    .route("/posts/{id}", web::put().to(post_handler::update_post))
                    .route("/posts/{id}", web::delete().to(post_handler::delete_post))
                    // 页面管理
                    .route("/pages", web::get().to(page_handler::get_pages))
                    .route("/pages", web::post().to(page_handler::create_page))
                    .route("/pages/{id}", web::put().to(page_handler::update_page))
                    .route("/pages/{id}", web::delete().to(page_handler::delete_page))
                    // 分类管理
                    .route("/categories", web::post().to(category_handler::create_category))
                    .route("/categories/{id}", web::put().to(category_handler::update_category))
                    .route("/categories/{id}", web::delete().to(category_handler::delete_category))
                    // MCP 接口
                    .route("/mcp", web::post().to(handle_mcp_request))
                    // 文件上传管理
                    .route("/uploads", web::post().to(upload_handler::upload_file))
                    .route("/uploads", web::get().to(upload_handler::list_files))
                    .route("/uploads/{id}", web::get().to(upload_handler::get_file))
                    .route("/uploads/{id}", web::delete().to(upload_handler::delete_file))
                    .route("/storage/info", web::get().to(upload_handler::get_storage_info))
            )
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
