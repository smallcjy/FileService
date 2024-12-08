use crate::{
    controller::{s3, v1::parse::parse},
    utils::{filename_parse::parse_filetype, http_util::post_book_info},
};
use actix_multipart::form::MultipartForm;
use actix_web::{web, HttpResponse};
use aws_sdk_s3::Client;
use file::UploadForm;
use log::info;
use std::io::Read;

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
            .route("/download/{bookid}", web::get().to(download)),
    );
}

pub async fn oss_temp_credential(client: web::Data<Client>) -> HttpResponse {
    HttpResponse::Ok().json(
        s3::get_presigned_download_url(
            &client,
            "test",                             // TODO
            "test",                             // TODO
            std::time::Duration::from_secs(60), // TODO
        )
        .await
        .unwrap()
        // PresignedRequest doc: https://docs.rs/aws-sdk-s3/latest/aws_sdk_s3/presigning/struct.PresignedRequest.html
        // Error doc: https://docs.rs/aws-sdk-s3/latest/aws_sdk_s3/operation/get_object/enum.GetObjectError.html
        // TODO
        // - [ ] handle error
        .uri(),
    )
}

pub async fn upload(
    mut payload: MultipartForm<UploadForm>,
    client: web::Data<Client>,
) -> HttpResponse {
    info!(
        "upload file:{:?}",
        parse_filetype(&payload.file.file_name.clone().unwrap())
    );
    // parse epub
    let mut epub_buffer = Vec::new();
    payload
        .file
        .file
        .read_to_end(&mut epub_buffer)
        .map_err(|err| HttpResponse::BadRequest().json(err.to_string()))
        .unwrap();

    if let Some(res) = parse(
        epub_buffer.clone(),
        parse_filetype(&payload.file.file_name.clone().unwrap()),
    )
    .await
    {
        // upload epub

        let book = res.0;
        let cover = res.1;
        // Output doc: https://docs.rs/aws-sdk-s3/latest/aws_sdk_s3/operation/put_object/struct.PutObjectOutput.html
        // Error doc: https://docs.rs/aws-sdk-s3/latest/aws_sdk_s3/operation/put_object/enum.PutObjectError.html
        // TODO: handle error
        s3::upload(
            &client,
            &book.id.to_string(),
            "librebooks",
            // https://docs.rs/aws-sdk-s3/latest/aws_sdk_s3/?search=ByteStream
            aws_sdk_s3::primitives::ByteStream::from(epub_buffer),
        )
        .await
        .ok();

        // save cover to oss
        s3::upload(
            &client,
            &cover.id,
            "librebooks",
            aws_sdk_s3::primitives::ByteStream::from(cover.data.0),
        )
        .await
        .ok();

        // post book info to libre
        match post_book_info(book).await {
            Ok(_) => (),
            Err(_) => return HttpResponse::InternalServerError().json("post book info failed"),
        }

        HttpResponse::Ok().json("upload")
    } else {
        return HttpResponse::InternalServerError().json("upload failed!");
    }
}

pub async fn download(bookid: web::Path<i32>, client: web::Data<Client>) -> HttpResponse {
    // info!("download book: {}", bookid);
    let mut file_stream = match s3::download(&client, &bookid.to_string(), "librebooks").await {
        Ok(output) => output.body,
        Err(_) => return HttpResponse::InternalServerError().json("Failed to download file!"),
    };

    let mut temp_stream = Vec::new();
    while let Ok(Some(bytes)) = file_stream
        .try_next()
        .await
        .map_err(|err| HttpResponse::InternalServerError().json(err.to_string()))
    {
        temp_stream.extend_from_slice(&bytes);
    }

    return HttpResponse::Ok()
        .content_type("application/epub+zip")
        .append_header((
            "Content-Disposition",
            format!("attachment; filename=\"{}.epub\"", bookid),
        ))
        .body(temp_stream);
}
