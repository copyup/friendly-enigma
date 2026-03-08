use opendal::Operator;
use std::env;
use tracing::{error, info};

pub struct StorageService {
    operator: Operator,
    base_url: String,
    storage_type: String,
}

impl StorageService {
    /// 从环境变量创建存储服务
    /// 支持: fs (本地文件系统), s3 (兼容S3的存储如阿里云OSS、MinIO)
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let storage_type = env::var("STORAGE_TYPE").unwrap_or_else(|_| "fs".to_string());

        match storage_type.as_str() {
            "s3" => Self::create_s3_storage(),
            "fs" | _ => Self::create_fs_storage(),
        }
    }

    /// 创建本地文件系统存储
    fn create_fs_storage() -> Result<Self, Box<dyn std::error::Error>> {
        let upload_dir = env::var("UPLOAD_DIR").unwrap_or_else(|_| "./uploads".to_string());
        let base_url = env::var("BASE_URL").unwrap_or_else(|_| "http://localhost:8080".to_string());

        let builder = opendal::services::Fs::default()
            .root(&upload_dir);

        let operator = Operator::new(builder)?.finish();

        info!("创建本地文件系统存储: {}", upload_dir);

        Ok(Self {
            operator,
            base_url,
            storage_type: "fs".to_string(),
        })
    }

    /// 创建 S3 兼容存储（阿里云 OSS、MinIO 等）
    fn create_s3_storage() -> Result<Self, Box<dyn std::error::Error>> {
        let bucket = env::var("S3_BUCKET").map_err(|_| "S3_BUCKET 环境变量未设置")?;
        let endpoint = env::var("S3_ENDPOINT").map_err(|_| "S3_ENDPOINT 环境变量未设置")?;
        let access_key = env::var("S3_ACCESS_KEY").map_err(|_| "S3_ACCESS_KEY 环境变量未设置")?;
        let secret_key = env::var("S3_SECRET_KEY").map_err(|_| "S3_SECRET_KEY 环境变量未设置")?;
        let region = env::var("S3_REGION").unwrap_or_else(|_| "us-east-1".to_string());

        let mut builder = opendal::services::S3::default()
            .bucket(&bucket)
            .endpoint(&endpoint)
            .access_key_id(&access_key)
            .secret_access_key(&secret_key)
            .region(&region);

        // 针对阿里云 OSS 的特殊配置
        if endpoint.contains("aliyuncs.com") {
            builder = builder.enable_virtual_host_style();
        }

        let operator = Operator::new(builder)?.finish();

        let base_url = env::var("CDN_URL")
            .unwrap_or_else(|_| format!("{}/{}", endpoint, bucket));

        info!("创建 S3 存储: bucket={}, endpoint={}", bucket, endpoint);

        Ok(Self {
            operator,
            base_url,
            storage_type: "s3".to_string(),
        })
    }

    /// 上传文件
    pub async fn upload(
        &self,
        file_name: &str,
        content: Vec<u8>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let path = format!("uploads/{}", file_name);

        self.operator.write(&path, content).await.map_err(|e| {
            error!("文件上传失败: {}", e);
            Box::new(e) as Box<dyn std::error::Error>
        })?;

        let file_url = format!("{}/{}", self.base_url, path);
        info!("文件上传成功: {}", file_url);

        Ok(file_url)
    }

    /// 删除文件
    pub async fn delete(&self, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.operator.delete(file_path).await.map_err(|e| {
            error!("文件删除失败: {}", e);
            Box::new(e) as Box<dyn std::error::Error>
        })?;

        info!("文件删除成功: {}", file_path);
        Ok(())
    }

    /// 获取文件内容
    pub async fn get(&self, file_path: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let content: Vec<u8> = self.operator.read(file_path).await.map_err(|e| {
            error!("文件读取失败: {}", e);
            Box::new(e) as Box<dyn std::error::Error>
        })?.to_vec();

        Ok(content)
    }

    /// 检查文件是否存在
    pub async fn exists(&self, file_path: &str) -> Result<bool, Box<dyn std::error::Error>> {
        let exists = self.operator.exists(file_path).await.map_err(|e| {
            error!("文件检查失败: {}", e);
            Box::new(e) as Box<dyn std::error::Error>
        })?;

        Ok(exists)
    }

    /// 获取存储类型
    pub fn storage_type(&self) -> &str {
        &self.storage_type
    }

    /// 获取基础 URL
    pub fn base_url(&self) -> &str {
        &self.base_url
    }
}

impl Clone for StorageService {
    fn clone(&self) -> Self {
        Self {
            operator: self.operator.clone(),
            base_url: self.base_url.clone(),
            storage_type: self.storage_type.clone(),
        }
    }
}
