use actix_web::{web, HttpResponse};
use serde_json::json;

use crate::db::{CategoryRepository, PageRepository, PostRepository, UploadRepository};
use crate::mcp::models::{McpOperation, McpRequest, McpResource, McpResponse};
use crate::models::{CreateCategory, CreatePage, CreatePost, UpdateCategory, UpdatePage, UpdatePost};
use crate::services::StorageService;

pub async fn handle_mcp_request(
    request: web::Json<McpRequest>,
    post_repo: web::Data<PostRepository>,
    page_repo: web::Data<PageRepository>,
    category_repo: web::Data<CategoryRepository>,
    upload_repo: web::Data<UploadRepository>,
    storage: web::Data<StorageService>,
) -> Result<HttpResponse, actix_web::Error> {
    let response = match (&request.resource, &request.operation) {
        // 处理工具列表
        (McpResource::Tools, McpOperation::List) => handle_tools_list().await,

        // 处理文章操作
        (McpResource::Post, McpOperation::Create) => {
            handle_post_create(&post_repo, &request.data).await
        }
        (McpResource::Post, McpOperation::Read) => {
            handle_post_read(&post_repo, &request.id, &request.params).await
        }
        (McpResource::Post, McpOperation::Update) => {
            handle_post_update(&post_repo, &request.id, &request.data).await
        }
        (McpResource::Post, McpOperation::Delete) => {
            handle_post_delete(&post_repo, &request.id).await
        }
        (McpResource::Post, McpOperation::List) => handle_post_list(&post_repo, &request.params).await,

        // 处理页面操作
        (McpResource::Page, McpOperation::Create) => {
            handle_page_create(&page_repo, &request.data).await
        }
        (McpResource::Page, McpOperation::Read) => {
            handle_page_read(&page_repo, &request.id, &request.params).await
        }
        (McpResource::Page, McpOperation::Update) => {
            handle_page_update(&page_repo, &request.id, &request.data).await
        }
        (McpResource::Page, McpOperation::Delete) => {
            handle_page_delete(&page_repo, &request.id).await
        }
        (McpResource::Page, McpOperation::List) => handle_page_list(&page_repo, &request.params).await,

        // 处理分类操作
        (McpResource::Category, McpOperation::Create) => {
            handle_category_create(&category_repo, &request.data).await
        }
        (McpResource::Category, McpOperation::Read) => {
            handle_category_read(&category_repo, &request.id, &request.params).await
        }
        (McpResource::Category, McpOperation::Update) => {
            handle_category_update(&category_repo, &request.id, &request.data).await
        }
        (McpResource::Category, McpOperation::Delete) => {
            handle_category_delete(&category_repo, &request.id).await
        }
        (McpResource::Category, McpOperation::List) => {
            handle_category_list(&category_repo, &request.params).await
        }

        // 处理文件上传操作
        (McpResource::Upload, McpOperation::Read) => {
            handle_upload_read(&upload_repo, &request.id).await
        }
        (McpResource::Upload, McpOperation::Delete) => {
            handle_upload_delete(&upload_repo, &storage, &request.id).await
        }
        (McpResource::Upload, McpOperation::List) => {
            handle_upload_list(&upload_repo).await
        }

        // 处理其他情况
        _ => McpResponse::error("不支持的操作".to_string()),
    };

    Ok(HttpResponse::Ok().json(response))
}

/// 处理工具列表请求
async fn handle_tools_list() -> McpResponse {
    let tools = vec![
        json!({
            "name": "Post Operations",
            "description": "文章相关操作",
            "operations": [
                {
                    "operation": "create",
                    "description": "创建新文章",
                    "params": {
                        "title": "文章标题",
                        "slug": "文章别名",
                        "content": "文章内容",
                        "status": "文章状态（published/draft）",
                        "category_id": "分类ID"
                    }
                },
                {
                    "operation": "read",
                    "description": "读取文章详情",
                    "params": {
                        "id": "文章ID",
                        "slug": "文章别名（可选）"
                    }
                },
                {
                    "operation": "update",
                    "description": "更新文章",
                    "params": {
                        "id": "文章ID",
                        "title": "文章标题（可选）",
                        "slug": "文章别名（可选）",
                        "content": "文章内容（可选）",
                        "status": "文章状态（可选）",
                        "category_id": "分类ID（可选）"
                    }
                },
                {
                    "operation": "delete",
                    "description": "删除文章",
                    "params": {
                        "id": "文章ID"
                    }
                },
                {
                    "operation": "list",
                    "description": "获取文章列表",
                    "params": {
                        "published": "是否仅获取已发布文章（true/false）"
                    }
                }
            ]
        }),
        json!({
            "name": "Page Operations",
            "description": "页面相关操作",
            "operations": [
                {
                    "operation": "create",
                    "description": "创建新页面",
                    "params": {
                        "title": "页面标题",
                        "slug": "页面临别名",
                        "content": "页面内容",
                        "status": "页面状态（published/draft）"
                    }
                },
                {
                    "operation": "read",
                    "description": "读取页面详情",
                    "params": {
                        "id": "页面ID",
                        "slug": "页面临别名（可选）"
                    }
                },
                {
                    "operation": "update",
                    "description": "更新页面",
                    "params": {
                        "id": "页面ID",
                        "title": "页面标题（可选）",
                        "slug": "页面临别名（可选）",
                        "content": "页面内容（可选）",
                        "status": "页面状态（可选）"
                    }
                },
                {
                    "operation": "delete",
                    "description": "删除页面",
                    "params": {
                        "id": "页面ID"
                    }
                },
                {
                    "operation": "list",
                    "description": "获取页面列表",
                    "params": {
                        "published": "是否仅获取已发布页面（true/false）"
                    }
                }
            ]
        }),
        json!({
            "name": "Category Operations",
            "description": "分类相关操作",
            "operations": [
                {
                    "operation": "create",
                    "description": "创建新分类",
                    "params": {
                        "name": "分类名称",
                        "slug": "分类别名",
                        "description": "分类描述（可选）",
                        "parent_id": "父分类ID（可选）"
                    }
                },
                {
                    "operation": "read",
                    "description": "读取分类详情",
                    "params": {
                        "id": "分类ID",
                        "slug": "分类别名（可选）"
                    }
                },
                {
                    "operation": "update",
                    "description": "更新分类",
                    "params": {
                        "id": "分类ID",
                        "name": "分类名称（可选）",
                        "slug": "分类别名（可选）",
                        "description": "分类描述（可选）",
                        "parent_id": "父分类ID（可选）"
                    }
                },
                {
                    "operation": "delete",
                    "description": "删除分类",
                    "params": {
                        "id": "分类ID"
                    }
                },
                {
                    "operation": "list",
                    "description": "获取分类列表",
                    "params": {
                        "tree": "是否获取树形结构（true/false）"
                    }
                }
            ]
        }),
        json!({
            "name": "Tools Operations",
            "description": "工具列表相关操作",
            "operations": [
                {
                    "operation": "list",
                    "description": "获取所有可用工具列表",
                    "params": {}
                }
            ]
        }),
        json!({
            "name": "Upload Operations",
            "description": "文件上传相关操作",
            "operations": [
                {
                    "operation": "read",
                    "description": "读取文件详情",
                    "params": {
                        "id": "文件ID"
                    }
                },
                {
                    "operation": "delete",
                    "description": "删除文件",
                    "params": {
                        "id": "文件ID"
                    }
                },
                {
                    "operation": "list",
                    "description": "获取文件列表",
                    "params": {}
                }
            ],
            "note": "文件上传需要通过 /api/admin/uploads 接口使用 multipart/form-data 格式上传"
        })
    ];

    McpResponse::success(Some(json!(tools)), None)
}

async fn handle_post_create(
    repo: &PostRepository,
    data: &Option<serde_json::Value>,
) -> McpResponse {
    match data {
        Some(json_data) => match serde_json::from_value::<CreatePost>(json_data.clone()) {
            Ok(create_data) => match repo.create(create_data).await {
                Ok(post) => McpResponse::success(Some(json!(post)), Some("文章创建成功".to_string())),
                Err(e) => McpResponse::error(format!("创建文章失败：{}", e)),
            },
            Err(e) => McpResponse::error(format!("无效的文章数据：{}", e)),
        },
        None => McpResponse::error("缺少文章数据".to_string()),
    }
}

async fn handle_post_read(
    repo: &PostRepository,
    id: &Option<String>,
    params: &Option<serde_json::Value>,
) -> McpResponse {
    if let Some(id_str) = id {
            match repo.get_by_id(id_str).await {
                Ok(Some(post)) => McpResponse::success(Some(json!(post)), None),
                Ok(None) => McpResponse::error("文章不存在".to_string()),
                Err(e) => McpResponse::error(format!("获取文章失败：{}", e)),
            }
    } else if let Some(params_data) = params {
        // 支持通过 slug 查询
        if let Some(slug) = params_data.get("slug").and_then(|s| s.as_str()) {
            match repo.get_by_slug(slug).await {
                Ok(Some(post)) => McpResponse::success(Some(json!(post)), None),
                Ok(None) => McpResponse::error("文章不存在".to_string()),
                Err(e) => McpResponse::error(format!("获取文章失败：{}", e)),
            }
        } else {
            McpResponse::error("缺少 slug 参数".to_string())
        }
    } else {
        McpResponse::error("需要提供 ID 或 slug 参数".to_string())
    }
}

async fn handle_post_update(
    repo: &PostRepository,
    id: &Option<String>,
    data: &Option<serde_json::Value>,
) -> McpResponse {
    if let Some(id_str) = id {

            match serde_json::from_value::<UpdatePost>(data.clone().unwrap_or_default()) {
                Ok(update_data) => match repo.update(id_str, update_data).await {
                    Ok(post) => McpResponse::success(Some(json!(post)), Some("文章更新成功".to_string())),
                    Err(_) => McpResponse::error("更新文章失败".to_string()),
                },
                Err(e) => McpResponse::error(format!("无效的文章数据：{}", e)),
            }

    } else {
        McpResponse::error("缺少文章 ID".to_string())
    }
}

async fn handle_post_delete(repo: &PostRepository, id: &Option<String>) -> McpResponse {
    if let Some(id_str) = id {
            match repo.delete(id_str).await {
                Ok(true) => McpResponse::success(None, Some("文章删除成功".to_string())),
                Ok(false) => McpResponse::error("文章不存在".to_string()),
                Err(_) => McpResponse::error("删除文章失败".to_string()),
            }
    } else {
        McpResponse::error("缺少文章 ID".to_string())
    }
}

async fn handle_post_list(
    repo: &PostRepository,
    params: &Option<serde_json::Value>,
) -> McpResponse {
    let result = if let Some(p) = params {
        if p.get("published").and_then(|v| v.as_bool()).unwrap_or(false) {
            repo.get_published().await
        } else {
            repo.get_all().await
        }
    } else {
        repo.get_all().await
    };

    match result {
        Ok(posts) => McpResponse::success(Some(json!(posts)), None),
        Err(e) => McpResponse::error(format!("获取文章列表失败：{}", e)),
    }
}

// 页面处理函数
async fn handle_page_create(
    repo: &PageRepository,
    data: &Option<serde_json::Value>,
) -> McpResponse {
    match data {
        Some(json_data) => match serde_json::from_value::<CreatePage>(json_data.clone()) {
            Ok(create_data) => match repo.create(create_data).await {
                Ok(page) => McpResponse::success(Some(json!(page)), Some("页面创建成功".to_string())),
                Err(e) => McpResponse::error(format!("创建页面失败：{}", e)),
            },
            Err(e) => McpResponse::error(format!("无效的页面数据：{}", e)),
        },
        None => McpResponse::error("缺少页面数据".to_string()),
    }
}

async fn handle_page_read(
    repo: &PageRepository,
    id: &Option<String>,
    params: &Option<serde_json::Value>,
) -> McpResponse {
    if let Some(id_str) = id {
            match repo.get_by_id(id_str).await {
                Ok(Some(page)) => McpResponse::success(Some(json!(page)), None),
                Ok(None) => McpResponse::error("页面不存在".to_string()),
                Err(e) => McpResponse::error(format!("获取页面失败：{}", e)),
            }
        
    } else if let Some(params_data) = params {
        if let Some(slug) = params_data.get("slug").and_then(|s| s.as_str()) {
            match repo.get_by_slug(slug).await {
                Ok(Some(page)) => McpResponse::success(Some(json!(page)), None),
                Ok(None) => McpResponse::error("页面不存在".to_string()),
                Err(e) => McpResponse::error(format!("获取页面失败：{}", e)),
            }
        } else {
            McpResponse::error("缺少 slug 参数".to_string())
        }
    } else {
        McpResponse::error("需要提供 ID 或 slug 参数".to_string())
    }
}

async fn handle_page_update(
    repo: &PageRepository,
    id: &Option<String>,
    data: &Option<serde_json::Value>,
) -> McpResponse {
    if let Some(id_str) = id {
            match serde_json::from_value::<UpdatePage>(data.clone().unwrap_or_default()) {
                Ok(update_data) => match repo.update(id_str, update_data).await {
                    Ok(page) => McpResponse::success(Some(json!(page)), Some("页面更新成功".to_string())),
                    Err(_) => McpResponse::error("更新页面失败".to_string()),
                },
                Err(e) => McpResponse::error(format!("无效的页面数据：{}", e)),
            }
    } else {
        McpResponse::error("缺少页面 ID".to_string())
    }
}

async fn handle_page_delete(repo: &PageRepository, id: &Option<String>) -> McpResponse {
    if let Some(id_str) = id {
            match repo.delete(id_str).await {
                Ok(true) => McpResponse::success(None, Some("页面删除成功".to_string())),
                Ok(false) => McpResponse::error("页面不存在".to_string()),
                Err(_) => McpResponse::error("删除页面失败".to_string()),
            }
    } else {
        McpResponse::error("缺少页面 ID".to_string())
    }
}

async fn handle_page_list(
    repo: &PageRepository,
    params: &Option<serde_json::Value>,
) -> McpResponse {
    let result = if let Some(p) = params {
        if p.get("published").and_then(|v| v.as_bool()).unwrap_or(false) {
            repo.get_published().await
        } else {
            repo.get_all().await
        }
    } else {
        repo.get_all().await
    };

    match result {
        Ok(pages) => McpResponse::success(Some(json!(pages)), None),
        Err(e) => McpResponse::error(format!("获取页面列表失败：{}", e)),
    }
}

// 分类处理函数
async fn handle_category_create(
    repo: &CategoryRepository,
    data: &Option<serde_json::Value>,
) -> McpResponse {
    match data {
        Some(json_data) => match serde_json::from_value::<CreateCategory>(json_data.clone()) {
            Ok(create_data) => match repo.create(create_data).await {
                Ok(category) => McpResponse::success(Some(json!(category)), Some("分类创建成功".to_string())),
                Err(e) => McpResponse::error(format!("创建分类失败：{}", e)),
            },
            Err(e) => McpResponse::error(format!("无效的分类数据：{}", e)),
        },
        None => McpResponse::error("缺少分类数据".to_string()),
    }
}

async fn handle_category_read(
    repo: &CategoryRepository,
    id: &Option<String>,
    params: &Option<serde_json::Value>,
) -> McpResponse {
    if let Some(id_str) = id {
            match repo.get_by_id(id_str).await {
                Ok(Some(category)) => McpResponse::success(Some(json!(category)), None),
                Ok(None) => McpResponse::error("分类不存在".to_string()),
                Err(e) => McpResponse::error(format!("获取分类失败：{}", e)),
            }
    } else if let Some(params_data) = params {
        if let Some(slug) = params_data.get("slug").and_then(|s| s.as_str()) {
            match repo.get_by_slug(slug).await {
                Ok(Some(category)) => McpResponse::success(Some(json!(category)), None),
                Ok(None) => McpResponse::error("分类不存在".to_string()),
                Err(e) => McpResponse::error(format!("获取分类失败：{}", e)),
            }
        } else {
            McpResponse::error("缺少 slug 参数".to_string())
        }
    } else {
        McpResponse::error("需要提供 ID 或 slug 参数".to_string())
    }
}

async fn handle_category_update(
    repo: &CategoryRepository,
    id: &Option<String>,
    data: &Option<serde_json::Value>,
) -> McpResponse {
    if let Some(id_str) = id {
            match serde_json::from_value::<UpdateCategory>(data.clone().unwrap_or_default()) {
                Ok(update_data) => match repo.update(id_str, update_data).await {
                    Ok(category) => McpResponse::success(Some(json!(category)), Some("分类更新成功".to_string())),
                    Err(_) => McpResponse::error("更新分类失败".to_string()),
                },
                Err(e) => McpResponse::error(format!("无效的分类数据：{}", e)),
            }
    } else {
        McpResponse::error("缺少分类 ID".to_string())
    }
}

async fn handle_category_delete(repo: &CategoryRepository, id: &Option<String>) -> McpResponse {
    if let Some(id_str) = id {
          match repo.delete(id_str).await {
                Ok(true) => McpResponse::success(None, Some("分类删除成功".to_string())),
                Ok(false) => McpResponse::error("分类不存在".to_string()),
                Err(_) => McpResponse::error("删除分类失败".to_string()),
            }
    } else {
        McpResponse::error("缺少分类 ID".to_string())
    }
}

async fn handle_category_list(
    repo: &CategoryRepository,
    params: &Option<serde_json::Value>,
) -> McpResponse {
    let result = if let Some(p) = params {
        if p.get("tree").and_then(|v| v.as_bool()).unwrap_or(false) {
            repo.get_tree().await.map(|tree| {
                // 转换为 JSON 值
                serde_json::to_value(tree).unwrap_or_default()
            })
        } else {
            repo.get_all().await.map(|cats| json!(cats))
        }
    } else {
        repo.get_all().await.map(|cats| json!(cats))
    };

    match result {
        Ok(data) => McpResponse::success(Some(data), None),
        Err(e) => McpResponse::error(format!("获取分类列表失败：{}", e)),
    }
}

// 文件上传处理函数
async fn handle_upload_read(
    repo: &UploadRepository,
    id: &Option<String>,
) -> McpResponse {
    if let Some(id_str) = id {
        match repo.get_by_id(id_str).await {
            Ok(Some(file)) => McpResponse::success(Some(json!(file)), None),
            Ok(None) => McpResponse::error("文件不存在".to_string()),
            Err(e) => McpResponse::error(format!("获取文件失败：{}", e)),
        }
    } else {
        McpResponse::error("缺少文件 ID".to_string())
    }
}

async fn handle_upload_delete(
    repo: &UploadRepository,
    storage: &StorageService,
    id: &Option<String>,
) -> McpResponse {
    if let Some(id_str) = id {
        // 先获取文件信息
        match repo.get_by_id(id_str).await {
            Ok(Some(file)) => {
                // 从存储中删除
                if let Err(e) = storage.delete(&file.file_path).await {
                    tracing::error!("从存储删除文件失败: {}", e);
                }

                // 从数据库删除
                match repo.delete(id_str).await {
                    Ok(true) => McpResponse::success(None, Some("文件删除成功".to_string())),
                    Ok(false) => McpResponse::error("文件不存在".to_string()),
                    Err(_) => McpResponse::error("删除文件失败".to_string()),
                }
            }
            Ok(None) => McpResponse::error("文件不存在".to_string()),
            Err(e) => McpResponse::error(format!("获取文件信息失败：{}", e)),
        }
    } else {
        McpResponse::error("缺少文件 ID".to_string())
    }
}

async fn handle_upload_list(repo: &UploadRepository) -> McpResponse {
    match repo.get_all().await {
        Ok(files) => McpResponse::success(Some(json!(files)), None),
        Err(e) => McpResponse::error(format!("获取文件列表失败：{}", e)),
    }
}
