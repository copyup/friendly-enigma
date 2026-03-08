use actix_multipart::Multipart;
use actix_web::{web, HttpResponse};
use futures_util::TryStreamExt;
use tracing::{error, info};

use crate::db::UploadRepository;
use crate::models::{CreateUploadFile, FileListResponse, UploadResponse};
use crate::services::StorageService;

const MAX_FILE_SIZE: usize = 10 * 1024 * 1024; // 10MB
const ALLOWED_MIME_TYPES: &[&str] = &[
    "image/jpeg",
    "image/png",
    "image/gif",
    "image/webp",
    "application/pdf",
    "text/plain",
    "text/markdown",
];

/// 上传文件
pub async fn upload_file(
    mut payload: Multipart,
    storage: web::Data<StorageService>,
    repo: web::Data<UploadRepository>,
) -> Result<HttpResponse, actix_web::Error> {
    let mut files = Vec::new();

    while let Some(mut field) = payload.try_next().await? {
        let content_disposition = field.content_disposition();
        let original_name: String = content_disposition
            .and_then(|cd| cd.get_filename())
            .map(|s: &str| s.to_string())
            .unwrap_or_else(|| "unknown".to_string());

        // 清理文件名，移除危险字符
        let safe_name = sanitize_filename(&original_name);

        // 生成唯一文件名
        let file_ext = get_file_extension(&safe_name);
        let unique_name = format!("{}.{}", cuid2::create_id(), file_ext);

        // 读取文件内容
        let mut content = Vec::new();
        let mut total_size = 0;

        while let Some(chunk) = field.try_next().await? {
            total_size += chunk.len();

            if total_size > MAX_FILE_SIZE {
                return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                    "success": false,
                    "error": "文件大小超过限制 (最大 10MB)"
                })));
            }

            content.extend_from_slice(&chunk);
        }

        // 检测 MIME 类型
        let mime_type = infer::get(&content)
            .map(|t| t.mime_type().to_string())
            .unwrap_or_else(|| "application/octet-stream".to_string());

        // 验证 MIME 类型
        if !ALLOWED_MIME_TYPES.contains(&mime_type.as_str()) {
            return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "success": false,
                "error": format!("不支持的文件类型: {}", mime_type)
            })));
        }

        // 上传到存储
        match storage.upload(&unique_name, content.clone()).await {
            Ok(file_url) => {
                // 保存到数据库
                let create_data = CreateUploadFile {
                    original_name: original_name.clone(),
                    file_name: unique_name.clone(),
                    file_path: format!("uploads/{}", unique_name),
                    file_size: total_size as i64,
                    mime_type: mime_type.clone(),
                    file_url: file_url.clone(),
                    storage_type: storage.storage_type().to_string(),
                };

                match repo.create(create_data).await {
                    Ok(upload_file) => {
                        info!("文件上传成功: {}", file_url);
                        files.push(UploadResponse::from(upload_file));
                    }
                    Err(e) => {
                        error!("数据库保存失败: {}", e);
                        // 尝试删除已上传的文件
                        let _ = storage.delete(&format!("uploads/{}", unique_name)).await;
                        return Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                            "success": false,
                            "error": "文件保存失败"
                        })));
                    }
                }
            }
            Err(e) => {
                error!("文件上传失败: {}", e);
                return Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                    "success": false,
                    "error": "文件上传失败"
                })));
            }
        }
    }

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "data": files,
        "message": "文件上传成功"
    })))
}

/// 获取文件列表
pub async fn list_files(
    repo: web::Data<UploadRepository>,
) -> Result<HttpResponse, actix_web::Error> {
    match repo.get_all().await {
        Ok(files) => {
            let total = files.len() as i64;
            Ok(HttpResponse::Ok().json(serde_json::json!({
                "success": true,
                "data": FileListResponse { files, total },
                "message": "获取文件列表成功"
            })))
        }
        Err(e) => {
            error!("获取文件列表失败: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "success": false,
                "error": "获取文件列表失败"
            })))
        }
    }
}

/// 获取单个文件信息
pub async fn get_file(
    path: web::Path<String>,
    repo: web::Data<UploadRepository>,
) -> Result<HttpResponse, actix_web::Error> {
    let file_id = path.into_inner();

    match repo.get_by_id(&file_id).await {
        Ok(Some(file)) => Ok(HttpResponse::Ok().json(serde_json::json!({
            "success": true,
            "data": file,
            "message": "获取文件信息成功"
        }))),
        Ok(None) => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "success": false,
            "error": "文件不存在"
        }))),
        Err(e) => {
            error!("获取文件信息失败: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "success": false,
                "error": "获取文件信息失败"
            })))
        }
    }
}

/// 删除文件
pub async fn delete_file(
    path: web::Path<String>,
    storage: web::Data<StorageService>,
    repo: web::Data<UploadRepository>,
) -> Result<HttpResponse, actix_web::Error> {
    let file_id = path.into_inner();

    // 先获取文件信息
    match repo.get_by_id(&file_id).await {
        Ok(Some(file)) => {
            // 从存储中删除
            if let Err(e) = storage.delete(&file.file_path).await {
                error!("从存储删除文件失败: {}", e);
                // 继续删除数据库记录，即使存储删除失败
            }

            // 从数据库删除
            match repo.delete(&file_id).await {
                Ok(true) => Ok(HttpResponse::Ok().json(serde_json::json!({
                    "success": true,
                    "message": "文件删除成功"
                }))),
                Ok(false) => Ok(HttpResponse::NotFound().json(serde_json::json!({
                    "success": false,
                    "error": "文件不存在"
                }))),
                Err(e) => {
                    error!("删除文件记录失败: {}", e);
                    Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                        "success": false,
                        "error": "删除文件失败"
                    })))
                }
            }
        }
        Ok(None) => Ok(HttpResponse::NotFound().json(serde_json::json!({
            "success": false,
            "error": "文件不存在"
        }))),
        Err(e) => {
            error!("获取文件信息失败: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "success": false,
                "error": "删除文件失败"
            })))
        }
    }
}

/// 清理文件名，移除危险字符
fn sanitize_filename(filename: &str) -> String {
    filename
        .replace("..", "_")
        .replace('/', "_")
        .replace('\\', "_")
        .replace('\0', "")
}

/// 获取文件扩展名
fn get_file_extension(filename: &str) -> String {
    filename
        .rsplit('.')
        .next()
        .map(|s| s.to_lowercase())
        .unwrap_or_else(|| "bin".to_string())
}

/// 获取存储配置信息
pub async fn get_storage_info(
    storage: web::Data<StorageService>,
) -> Result<HttpResponse, actix_web::Error> {
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "data": {
            "storage_type": storage.storage_type(),
            "base_url": storage.base_url(),
            "max_file_size": MAX_FILE_SIZE,
            "allowed_types": ALLOWED_MIME_TYPES
        },
        "message": "获取存储配置成功"
    })))
}
