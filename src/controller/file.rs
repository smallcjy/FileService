use actix_web::HttpResponse;
use super::s3;


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

pub async fn upload() -> HttpResponse {
    s3::upload(
        "test", // TODO
        "test", // TODO
        // https://docs.rs/aws-sdk-s3/latest/aws_sdk_s3/?search=ByteStream
        aws_sdk_s3::primitives::ByteStream::from_path("big_file.csv")
            .await
            .unwrap() // TODO
    ).await.ok(); 
    // Output doc: https://docs.rs/aws-sdk-s3/latest/aws_sdk_s3/operation/put_object/struct.PutObjectOutput.html
    // Error doc: https://docs.rs/aws-sdk-s3/latest/aws_sdk_s3/operation/put_object/enum.PutObjectError.html
    // TODO: 
    // - [ ] do some checksum with output
    // - [ ] handle error
    HttpResponse::Ok().json("upload")
}

pub async fn download() -> HttpResponse {
    HttpResponse::Ok().json("download")
}