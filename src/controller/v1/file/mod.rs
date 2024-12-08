use crate::{controller::s3, utils::http_util::post_book_info};
use crate::models::epub::Epub;
use std::io::Read;
use actix_multipart::form::MultipartForm;
use actix_web::{web, HttpResponse};
use aws_sdk_s3::Client;
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
        .route("/download/{bookid}", web::get().to(download))
    );
}


pub async fn oss_temp_credential(client: web::Data<Client>) -> HttpResponse {
    HttpResponse::Ok().json(
        s3::get_presigned_download_url(
            &client,
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

pub async fn upload(mut payload: MultipartForm<UploadForm>, client: web::Data<Client>) -> HttpResponse {
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
        &client,
        &book.id.to_string(),
        "librebooks",
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

pub async fn download(bookid: web::Path<i32>, client: web::Data<Client>) -> HttpResponse {
    // info!("download book: {}", bookid);
    let mut file_stream = match s3::download(
        &client,
        &bookid.to_string(),
        "librebooks").await {
            Ok(output) => output.body,
            Err(_) => return HttpResponse::InternalServerError().json("Failed to download file!"),
        };
    
    let mut temp_stream = Vec::new();
    while let Ok(Some(bytes)) = file_stream.try_next().await.map_err(|err| {
        HttpResponse::InternalServerError().json(err.to_string())
    }) {
        temp_stream.extend_from_slice(&bytes);
    }

    
    return HttpResponse::Ok()
        .content_type("application/epub+zip")
        .append_header(("Content-Disposition", format!("attachment; filename=\"{}.epub\"", bookid)))
        .body(temp_stream)
}