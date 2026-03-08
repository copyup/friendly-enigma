use actix_web::{web, HttpResponse};
use serde_json::json;

use crate::db::PostRepository;
use crate::models::{CreatePost, UpdatePost};
use crate::utils::markdown::markdown_to_html;

pub async fn get_posts(
    repo: web::Data<PostRepository>,
) -> Result<HttpResponse, actix_web::Error> {
    let posts = repo.get_all().await.map_err(|e| {
        actix_web::error::ErrorInternalServerError(format!("Database error: {}", e))
    })?;
    Ok(HttpResponse::Ok().json(posts))
}

pub async fn get_published_posts(
    repo: web::Data<PostRepository>,
) -> Result<HttpResponse, actix_web::Error> {
    let posts = repo.get_published().await.map_err(|e| {
        actix_web::error::ErrorInternalServerError(format!("Database error: {}", e))
    })?;
    Ok(HttpResponse::Ok().json(posts))
}

pub async fn get_post(
    repo: web::Data<PostRepository>,
    path: web::Path<String>,
) -> Result<HttpResponse, actix_web::Error> {
    let id = path.into_inner();
    match repo.get_by_id(&id).await.map_err(|e| {
        actix_web::error::ErrorInternalServerError(format!("Database error: {}", e))
    })? {
        Some(post) => Ok(HttpResponse::Ok().json(post)),
        None => Ok(HttpResponse::NotFound().json(json!({"error": "Post not found"}))),
    }
}

pub async fn get_post_by_slug(
    repo: web::Data<PostRepository>,
    path: web::Path<String>,
) -> Result<HttpResponse, actix_web::Error> {
    let slug = path.into_inner();
    match repo.get_by_slug(&slug).await.map_err(|e| {
        actix_web::error::ErrorInternalServerError(format!("Database error: {}", e))
    })? {
        Some(post) => Ok(HttpResponse::Ok().json(post)),
        None => Ok(HttpResponse::NotFound().json(json!({"error": "Post not found"}))),
    }
}

pub async fn create_post(
    repo: web::Data<PostRepository>,
    data: web::Json<CreatePost>,
) -> Result<HttpResponse, actix_web::Error> {
    let post = repo.create(data.into_inner()).await.map_err(|e| {
        actix_web::error::ErrorInternalServerError(format!("Database error: {}", e))
    })?;
    Ok(HttpResponse::Created().json(post))
}

pub async fn update_post(
    repo: web::Data<PostRepository>,
    path: web::Path<String>,
    data: web::Json<UpdatePost>,
) -> Result<HttpResponse, actix_web::Error> {
    let id = path.into_inner();
    match repo.update(&id, data.into_inner()).await {
        Ok(post) => Ok(HttpResponse::Ok().json(post)),
        Err(_) => Ok(HttpResponse::NotFound().json(json!({"error": "Post not found"}))),
    }
}

pub async fn delete_post(
    repo: web::Data<PostRepository>,
    path: web::Path<String>,
) -> Result<HttpResponse, actix_web::Error> {
    let id = path.into_inner();
    match repo.delete(&id).await {
        Ok(true) => Ok(HttpResponse::NoContent().finish()),
        Ok(false) => Ok(HttpResponse::NotFound().finish()),
        Err(_) => Ok(HttpResponse::InternalServerError().finish()),
    }
}

pub async fn render_post_html(
    repo: web::Data<PostRepository>,
    path: web::Path<String>,
) -> Result<HttpResponse, actix_web::Error> {
    let id = path.into_inner();
    match repo.get_by_id(&id).await.map_err(|e| {
        actix_web::error::ErrorInternalServerError(format!("Database error: {}", e))
    })? {
        Some(post) => {
            let html_content = markdown_to_html(&post.content);
            Ok(HttpResponse::Ok()
                .content_type("text/html")
                .body(html_content))
        }
        None => Ok(HttpResponse::NotFound().json(json!({"error": "Post not found"}))),
    }
}
