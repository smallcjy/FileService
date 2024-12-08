use std::default;

use serde::{Deserialize, Serialize};
use chrono::NaiveDate;

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