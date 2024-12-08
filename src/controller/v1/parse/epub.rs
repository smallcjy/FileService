use epub::doc::EpubDoc;
use std::io::Cursor;
use uuid::Uuid;

use super::book::{Book, Cover};

pub struct Epub {
    // data
    data: EpubDoc<Cursor<Vec<u8>>>,
}

impl Epub {
    pub fn new(epub_buffer: Vec<u8>) -> Option<Self> {
        // 创建 Cursor 并从中读取 EpubDoc
        let cursor = std::io::Cursor::new(epub_buffer);
        let epub_doc = match EpubDoc::from_reader(cursor) {
            Ok(epub_doc) => epub_doc,
            Err(_) => return None,
        };
        Some(Self { data: epub_doc })
    }

    /// 解析 epub
    pub fn get_title(&mut self) -> Option<String> {
        self.data.mdata("title").clone()
    }

    #[allow(unused)]
    pub fn get_cover(&mut self) -> Option<(Vec<u8>, String)> {
        self.data.get_cover().clone()
    }

    pub fn get_author(&mut self) -> Option<String> {
        self.data.mdata("creator").clone()
    }

    pub fn parse_book(&mut self) -> Option<(Book, Cover)> {
        let uid = Uuid::new_v4();
        let id = format!("{}.epub", uid.to_string());
        let cover_id = format!("{}.jpg", uid.to_string());
        let title = match self.get_title() {
            Some(title) => title,
            None => "".to_string(),
        };
        let author = match self.get_author() {
            Some(author) => author,
            None => "".to_string(),
        };
        let cover = match self.get_cover() {
            Some(cover) => cover,
            None => Cover::default().data,
        };
        let book = Book {
            id,
            file_url: "".to_string(),
            cover_url: cover_id.clone(),
            title,
            author,
            rating: 0.0,
            status: 0,
            description: "".to_string(),
            added_date: chrono::Local::now().naive_local().date(),
        };

        let cover = Cover {
            id: cover_id,
            data: cover,
        };

        Some((book, cover))
    }
}
