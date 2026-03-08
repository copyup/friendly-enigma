use actix_web::{web, HttpResponse};
use serde_json::json;

use crate::db::CategoryRepository;
use crate::models::{CreateCategory, UpdateCategory};

pub async fn get_categories(
    repo: web::Data<CategoryRepository>,
) -> Result<HttpResponse, actix_web::Error> {
    let categories = repo.get_all().await.map_err(|e| {
        actix_web::error::ErrorInternalServerError(format!("Database error: {}", e))
    })?;
    Ok(HttpResponse::Ok().json(categories))
}

pub async fn get_category_tree(
    repo: web::Data<CategoryRepository>,
) -> Result<HttpResponse, actix_web::Error> {
    let tree = repo.get_tree().await.map_err(|e| {
        actix_web::error::ErrorInternalServerError(format!("Database error: {}", e))
    })?;
    Ok(HttpResponse::Ok().json(tree))
}

pub async fn get_category(
    repo: web::Data<CategoryRepository>,
    path: web::Path<String>,
) -> Result<HttpResponse, actix_web::Error> {
    let id = path.into_inner();
    match repo.get_by_id(&id).await.map_err(|e| {
        actix_web::error::ErrorInternalServerError(format!("Database error: {}", e))
    })? {
        Some(category) => Ok(HttpResponse::Ok().json(category)),
        None => Ok(HttpResponse::NotFound().json(json!({"error": "Category not found"}))),
    }
}

pub async fn create_category(
    repo: web::Data<CategoryRepository>,
    data: web::Json<CreateCategory>,
) -> Result<HttpResponse, actix_web::Error> {
    let category = repo.create(data.into_inner()).await.map_err(|e| {
        actix_web::error::ErrorInternalServerError(format!("Database error: {}", e))
    })?;
    Ok(HttpResponse::Created().json(category))
}

pub async fn update_category(
    repo: web::Data<CategoryRepository>,
    path: web::Path<String>,
    data: web::Json<UpdateCategory>,
) -> Result<HttpResponse, actix_web::Error> {
    let id = path.into_inner();
    match repo.update(&id, data.into_inner()).await {
        Ok(category) => Ok(HttpResponse::Ok().json(category)),
        Err(_) => Ok(HttpResponse::NotFound().json(json!({"error": "Category not found"}))),
    }
}

pub async fn delete_category(
    repo: web::Data<CategoryRepository>,
    path: web::Path<String>,
) -> Result<HttpResponse, actix_web::Error> {
    let id = path.into_inner();
    match repo.delete(&id).await {
        Ok(true) => Ok(HttpResponse::NoContent().finish()),
        Ok(false) => Ok(HttpResponse::NotFound().finish()),
        Err(_) => Ok(HttpResponse::InternalServerError().finish()),
    }
}
