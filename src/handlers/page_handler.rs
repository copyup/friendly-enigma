use actix_web::{web, HttpResponse};
use serde_json::json;

use crate::db::PageRepository;
use crate::models::{CreatePage, UpdatePage};
use crate::utils::markdown::markdown_to_html;

pub async fn get_pages(
    repo: web::Data<PageRepository>,
) -> Result<HttpResponse, actix_web::Error> {
    let pages = repo.get_all().await.map_err(|e| {
        actix_web::error::ErrorInternalServerError(format!("Database error: {}", e))
    })?;
    Ok(HttpResponse::Ok().json(pages))
}

pub async fn get_published_pages(
    repo: web::Data<PageRepository>,
) -> Result<HttpResponse, actix_web::Error> {
    let pages = repo.get_published().await.map_err(|e| {
        actix_web::error::ErrorInternalServerError(format!("Database error: {}", e))
    })?;
    Ok(HttpResponse::Ok().json(pages))
}

pub async fn get_page(
    repo: web::Data<PageRepository>,
    path: web::Path<String>,
) -> Result<HttpResponse, actix_web::Error> {
    let id = path.into_inner();
    match repo.get_by_id(&id).await.map_err(|e| {
        actix_web::error::ErrorInternalServerError(format!("Database error: {}", e))
    })? {
        Some(page) => Ok(HttpResponse::Ok().json(page)),
        None => Ok(HttpResponse::NotFound().json(json!({"error": "Page not found"}))),
    }
}

pub async fn get_page_by_slug(
    repo: web::Data<PageRepository>,
    path: web::Path<String>,
) -> Result<HttpResponse, actix_web::Error> {
    let slug = path.into_inner();
    match repo.get_by_slug(&slug).await.map_err(|e| {
        actix_web::error::ErrorInternalServerError(format!("Database error: {}", e))
    })? {
        Some(page) => Ok(HttpResponse::Ok().json(page)),
        None => Ok(HttpResponse::NotFound().json(json!({"error": "Page not found"}))),
    }
}

pub async fn create_page(
    repo: web::Data<PageRepository>,
    data: web::Json<CreatePage>,
) -> Result<HttpResponse, actix_web::Error> {
    let page = repo.create(data.into_inner()).await.map_err(|e| {
        actix_web::error::ErrorInternalServerError(format!("Database error: {}", e))
    })?;
    Ok(HttpResponse::Created().json(page))
}

pub async fn update_page(
    repo: web::Data<PageRepository>,
    path: web::Path<String>,
    data: web::Json<UpdatePage>,
) -> Result<HttpResponse, actix_web::Error> {
    let id = path.into_inner();
    match repo.update(&id, data.into_inner()).await {
        Ok(page) => Ok(HttpResponse::Ok().json(page)),
        Err(_) => Ok(HttpResponse::NotFound().json(json!({"error": "Page not found"}))),
    }
}

pub async fn delete_page(
    repo: web::Data<PageRepository>,
    path: web::Path<String>,
) -> Result<HttpResponse, actix_web::Error> {
    let id = path.into_inner();
    match repo.delete(&id).await {
        Ok(true) => Ok(HttpResponse::NoContent().finish()),
        Ok(false) => Ok(HttpResponse::NotFound().finish()),
        Err(_) => Ok(HttpResponse::InternalServerError().finish()),
    }
}

pub async fn render_page_html(
    repo: web::Data<PageRepository>,
    path: web::Path<String>,
) -> Result<HttpResponse, actix_web::Error> {
    let id = path.into_inner();
    match repo.get_by_id(&id).await.map_err(|e| {
        actix_web::error::ErrorInternalServerError(format!("Database error: {}", e))
    })? {
        Some(page) => {
            let html_content = markdown_to_html(&page.content);
            Ok(HttpResponse::Ok()
                .content_type("text/html")
                .body(html_content))
        }
        None => Ok(HttpResponse::NotFound().json(json!({"error": "Page not found"}))),
    }
}
