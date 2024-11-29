use serde::{Deserialize, Serialize};
use chrono::NaiveDate;

#[derive(Debug, Serialize, Deserialize)]
pub struct Book {
    pub id: i32,
    pub file_url: String,
    pub cover_url: String,
    pub title: String,
    pub author: String,
    pub rating: f64,
    pub status: i32,
    pub description: String,
    pub added_date: NaiveDate,
}