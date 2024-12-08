use std::io::{Read, Write};

use actix_multipart::form::MultipartForm;
use actix_web::{web, HttpResponse};
use aws_sdk_s3::Client;

use crate::{
    controller::{
        s3,
        v1::parse::{book::Book, parse, parse_filetype},
    },
    utils::http_util::post_book_info,
};

use super::file::UploadForm;

pub async fn upload_epub(
    mut payload: MultipartForm<UploadForm>,
    client: web::Data<Client>,
) -> HttpResponse {
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
            Ok(_) => HttpResponse::Ok().json("upload"),
            Err(_) => return HttpResponse::InternalServerError().json("post book info failed"),
        }
    } else {
        return HttpResponse::InternalServerError().json("upload failed!");
    }
}

pub async fn upload_pdf(
    mut payload: MultipartForm<UploadForm>,
    client: web::Data<Client>,
) -> HttpResponse {
    let mut pdf_buffer = Vec::new();
    payload
        .file
        .file
        .read_to_end(&mut pdf_buffer)
        .map_err(|err| HttpResponse::BadRequest().json(err.to_string()))
        .unwrap();

    let fuuid = uuid::Uuid::new_v4().to_string();
    let temp_path = format!("temp/files/{}.pdf", fuuid);
    let cover_path = format!("temp/covers/{}.jpg", fuuid);

    let mut file = match std::fs::File::create(&temp_path) {
        Ok(file) => file,
        Err(_) => return HttpResponse::InternalServerError().json("Failed to create file!"),
    };

    match file
        .write_all(&pdf_buffer)
        .map_err(|err| HttpResponse::InternalServerError().json(err.to_string()))
    {
        Ok(_) => drop(pdf_buffer),
        Err(_) => return HttpResponse::InternalServerError().json("Failed to write file!"),
    }

    //call get pdf cover
    match crate::utils::file_util::get_pdf_cover(uuid::Uuid::parse_str(&fuuid).unwrap()) {
        Ok(_) => (),
        Err(err) => return HttpResponse::InternalServerError().json(err.to_string()),
    }

    let mut book = Book::default();
    book.id = fuuid;
    book.title = payload.file.file_name.clone().unwrap().to_string();

    // upload pdf and its cover
    s3::upload(
        &client,
        &format!("{}.pdf", book.id),
        "librebooks",
        aws_sdk_s3::primitives::ByteStream::from_path(temp_path.clone())
            .await
            .unwrap(),
    )
    .await
    .ok();

    s3::upload(
        &client,
        &format!("{}.jpg", book.id),
        "librebooks",
        aws_sdk_s3::primitives::ByteStream::from_path(cover_path.clone())
            .await
            .unwrap(),
    )
    .await
    .ok();

    // delete temp files
    let _ = std::fs::remove_file(temp_path.clone());
    let _ = std::fs::remove_file(cover_path.clone());

    // post book info to libre
    match post_book_info(book).await {
        Ok(_) => HttpResponse::Ok().json("upload pdf"),
        Err(_) => return HttpResponse::InternalServerError().json("post book info failed"),
    }
}
