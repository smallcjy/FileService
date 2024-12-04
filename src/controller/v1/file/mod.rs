use crate::controller::s3;
use crate::{models::epub::Epub, utils::http_util::post_book_info};
use std::io::Read;
use actix_multipart::form::MultipartForm;
use actix_web::{web, HttpResponse};
use file::UploadForm;
pub mod file;

#[inline]
pub fn service_config(cfg: &mut web::ServiceConfig) {
    let middleware =
        actix_web_httpauth::middleware::HttpAuthentication::bearer(crate::casdoor::validator);

    cfg.service(
        web::scope("/v1")
        .wrap(middleware)
        .route("/oss-temp-credential", web::get().to(oss_temp_credential))
        .route("/upload", web::post().to(upload))
        .route("/download", web::get().to(download))
    );
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
        .map_err(|err| HttpResponse::BadRequest().json(err.to_string())).unwrap();

    let mut epub = Epub::new(epub_buffer.clone());
    let book =  match epub.parse_book() {
        Some(book) => book,
        None => return HttpResponse::BadRequest().json("parse epub failed")
    };

    // save file to oss
    // Output doc: https://docs.rs/aws-sdk-s3/latest/aws_sdk_s3/operation/put_object/struct.PutObjectOutput.html
    // Error doc: https://docs.rs/aws-sdk-s3/latest/aws_sdk_s3/operation/put_object/enum.PutObjectError.html
    // TODO: handle error
    s3::upload(
        &book.id.to_string(),
        "tempfile",
        // https://docs.rs/aws-sdk-s3/latest/aws_sdk_s3/?search=ByteStream
        aws_sdk_s3::primitives::ByteStream::from(epub_buffer)
    ).await.ok(); 

    // post book info to libre
    match post_book_info(book).await {
        Ok(_) => (),
        Err(_) => return HttpResponse::InternalServerError().json("post book info failed")
    }

    HttpResponse::Ok().json("upload")
}

pub async fn download(bookid: web::Path<i32>) -> HttpResponse {
    let file_stream = match s3::download(&bookid.to_string(), "tempfile").await {
            Ok(output) => output.body,
            Err(_) => return HttpResponse::InternalServerError().json("Failed to download file!"),
        };

    let temp_file = file_stream.bytes().unwrap().to_vec();

    
    return HttpResponse::Ok()
        .content_type("application/epub+zip")
        .append_header(("Content-Disposition", format!("attachment; filename=\"{}.epub\"", bookid)))
        .body(temp_file)
}