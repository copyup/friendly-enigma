use actix_web::{web, HttpResponse};
use askama::Template;

use crate::db::{CategoryRepository, PageRepository, PostRepository};
use crate::templates::{CategoriesTemplate, IndexTemplate, PageTemplate, PostTemplate, PostsTemplate};
use crate::utils::markdown::markdown_to_html;

pub async fn index(
    post_repo: web::Data<PostRepository>,
) -> Result<HttpResponse, actix_web::Error> {
    let posts = match post_repo.get_published().await {
        Ok(p) => p,
        Err(e) => return Ok(HttpResponse::InternalServerError().body(format!("Error: {}", e))),
    };

    let template = IndexTemplate { posts };
    match template.render() {
        Ok(body) => Ok(HttpResponse::Ok().content_type("text/html").body(body)),
        Err(e) => Ok(HttpResponse::InternalServerError().body(format!("Template error: {}", e))),
    }
}

pub async fn posts_list(
    post_repo: web::Data<PostRepository>,
) -> Result<HttpResponse, actix_web::Error> {
    let posts = match post_repo.get_published().await {
        Ok(p) => p,
        Err(e) => return Ok(HttpResponse::InternalServerError().body(format!("Error: {}", e))),
    };

    let template = PostsTemplate { posts };
    match template.render() {
        Ok(body) => Ok(HttpResponse::Ok().content_type("text/html").body(body)),
        Err(e) => Ok(HttpResponse::InternalServerError().body(format!("Template error: {}", e))),
    }
}

pub async fn post_detail(
    path: web::Path<String>,
    post_repo: web::Data<PostRepository>,
) -> Result<HttpResponse, actix_web::Error> {
    let slug = path.into_inner();
    let post = match post_repo.get_by_slug(&slug).await {
        Ok(Some(p)) => p,
        Ok(None) => return Ok(HttpResponse::NotFound().body("Post not found")),
        Err(e) => return Ok(HttpResponse::InternalServerError().body(format!("Error: {}", e))),
    };

    let content_html = markdown_to_html(&post.content);
    
    let published_at = post.published_at
        .map(|d| d.format("%Y-%m-%d").to_string())
        .unwrap_or_default();
    let updated_at = post.updated_at.format("%Y-%m-%d").to_string();

    let template = PostTemplate { 
        post, 
        content_html,
        published_at,
        updated_at,
    };
    match template.render() {
        Ok(body) => Ok(HttpResponse::Ok().content_type("text/html").body(body)),
        Err(e) => Ok(HttpResponse::InternalServerError().body(format!("Template error: {}", e))),
    }
}

pub async fn categories_list(
    category_repo: web::Data<CategoryRepository>,
) -> Result<HttpResponse, actix_web::Error> {
    let categories = match category_repo.get_tree().await {
        Ok(c) => c,
        Err(e) => return Ok(HttpResponse::InternalServerError().body(format!("Error: {}", e))),
    };

    let template = CategoriesTemplate { categories };
    match template.render() {
        Ok(body) => Ok(HttpResponse::Ok().content_type("text/html").body(body)),
        Err(e) => Ok(HttpResponse::InternalServerError().body(format!("Template error: {}", e))),
    }
}

pub async fn page_detail(
    path: web::Path<String>,
    page_repo: web::Data<PageRepository>,
) -> Result<HttpResponse, actix_web::Error> {
    let slug = path.into_inner();
    let page = match page_repo.get_by_slug(&slug).await {
        Ok(Some(p)) => p,
        Ok(None) => return Ok(HttpResponse::NotFound().body("Page not found")),
        Err(e) => return Ok(HttpResponse::InternalServerError().body(format!("Error: {}", e))),
    };

    let content_html = markdown_to_html(&page.content);

    let template = PageTemplate { page, content_html };
    match template.render() {
        Ok(body) => Ok(HttpResponse::Ok().content_type("text/html").body(body)),
        Err(e) => Ok(HttpResponse::InternalServerError().body(format!("Template error: {}", e))),
    }
}
