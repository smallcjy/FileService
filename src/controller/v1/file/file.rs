use actix_multipart::form::{tempfile::TempFile, MultipartForm};

// 临时文件流
#[derive(Debug, MultipartForm)]
pub struct UploadForm {
    pub file: TempFile,
}

