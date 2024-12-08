use serde::{Deserialize, Serialize};
use chrono::NaiveDate;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Book {
    pub id: String,
    pub file_url: String,
    pub cover_url: String,
    pub title: String,
    pub author: String,
    pub rating: f64,
    pub status: i32,
    pub description: String,
    pub added_date: NaiveDate,
}

impl Default for Book {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            file_url: "".to_string(),
            cover_url: "".to_string(),
            title: "".to_string(),
            author: "".to_string(),
            rating: 0.0,
            status: 0,
            description: "".to_string(),
            added_date: chrono::Local::now().naive_local().date()
        }
    }
}

pub struct Cover {
    pub id: String,
    pub data: (Vec<u8>, String),
}

impl Default for Cover {
    fn default() -> Self {
        Self {
            id: "".to_string(),
            data: (vec![], "".to_string()),
        }
    }
    
}