-- 创建文件上传表
CREATE TABLE IF NOT EXISTS upload_files (
    id VARCHAR(26) PRIMARY KEY,
    original_name VARCHAR(500) NOT NULL,
    file_name VARCHAR(500) NOT NULL,
    file_path VARCHAR(1000) NOT NULL,
    file_size BIGINT NOT NULL,
    mime_type VARCHAR(100) NOT NULL,
    file_url VARCHAR(1000) NOT NULL,
    storage_type VARCHAR(50) NOT NULL DEFAULT 'fs',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- 创建索引
CREATE INDEX IF NOT EXISTS idx_upload_files_created_at ON upload_files(created_at);
CREATE INDEX IF NOT EXISTS idx_upload_files_mime_type ON upload_files(mime_type);
