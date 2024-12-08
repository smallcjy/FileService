use book::{Book, Cover};
use epub::Epub;

use super::file::file::FileType;

pub mod book;
pub mod epub;
#[allow(unused)]
async fn parse_epub(epub_buffer: Vec<u8>) -> Option<(Book, Cover)> {
    let res = match Epub::new(epub_buffer) {
        Some(mut epub) => epub.parse_book(),
        None => return None,
    };
    res
}

pub async fn parse(file_buffer: Vec<u8>, file_type: FileType) -> Option<(Book, Cover)> {
    match file_type {
        FileType::Epub => parse_epub(file_buffer).await,
        FileType::Pdf => None,
        FileType::Unknown => None,
    }
}

/// 文件类型解析
pub fn parse_filetype(fname: &str) -> FileType {
    let ext = fname.rsplit('.').next().unwrap_or_default();
    FileType::from_str(ext)
}