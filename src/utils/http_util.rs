use std::error::Error;
use reqwest::Client;
use std::env;

use crate::models::model::Book;

pub async fn post_book_info(book: Book) -> Result<(), Box<dyn Error>> {
    let api_url = env::var("POST_BOOK_INFO_URL").expect("POST_BOOK_INFO must be set");
    log::debug!("Posting to URL: {}", api_url);
    log::debug!("Book info: {:?}", book);
    let client = Client::new();
    let res = client.post(api_url)
    .json(&book)
    .send()
    .await?;

    match res.status() {
        reqwest::StatusCode::OK => Ok(()),
        _ => Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Failed to post book info")))
    }
}