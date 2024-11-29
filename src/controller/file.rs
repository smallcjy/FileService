use std::io::Read;

use actix_web::HttpResponse;
use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use crate::{models::epub::Epub, utils::http_util::post_book_info};

use super::s3;
#[derive(Debug, MultipartForm)]
pub struct UploadForm {
    pub file: TempFile,
}


pub async fn oss_temp_credential() -> HttpResponse {
    HttpResponse::Ok().json(
        s3::get_presigned_download_url(
            "test", // TODO
            "test", // TODO
            std::time::Duration::from_secs(60) // TODO
        ).await
        .unwrap() 
        // PresignedRequest doc: https://docs.rs/aws-sdk-s3/latest/aws_sdk_s3/presigning/struct.PresignedRequest.html
        // Error doc: https://docs.rs/aws-sdk-s3/latest/aws_sdk_s3/operation/get_object/enum.GetObjectError.html
        // TODO
        // - [ ] handle error
        .uri()
    )
}

pub async fn upload(mut payload: MultipartForm<UploadForm>) -> HttpResponse {
    // parse epub
    let mut epub_buffer = Vec::new();
    payload.file.file.read_to_end(&mut epub_buffer)
        .map_err(|err| Box::new(err)).unwrap();

    let mut epub = Epub::new(epub_buffer.clone());
    let book =  match epub.parse_book() {
        Some(book) => book,
        None => return HttpResponse::BadRequest().json("parse epub failed")
    };

    // save file to oss

    s3::upload(
        &book.id.to_string(),
        "test", // TODO
        // https://docs.rs/aws-sdk-s3/latest/aws_sdk_s3/?search=ByteStream
        aws_sdk_s3::primitives::ByteStream::from(epub_buffer) // TODO
    ).await.ok(); 

    // Output doc: https://docs.rs/aws-sdk-s3/latest/aws_sdk_s3/operation/put_object/struct.PutObjectOutput.html
    // Error doc: https://docs.rs/aws-sdk-s3/latest/aws_sdk_s3/operation/put_object/enum.PutObjectError.html
    // TODO: 
    // - [ ] do some checksum with output
    // - [ ] handle error

    // post book info to libre
    match post_book_info(book).await {
        Ok(_) => (),
        Err(_) => return HttpResponse::InternalServerError().json("post book info failed")
    }

    HttpResponse::Ok().json("upload")
}

pub async fn download() -> HttpResponse {
    HttpResponse::Ok().json("download")
}