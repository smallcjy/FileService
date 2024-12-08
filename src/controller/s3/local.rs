#![allow(dead_code)]
use aws_sdk_s3::operation::{get_object::{GetObjectError, GetObjectOutput}, put_object::{PutObjectError, PutObjectOutput}};

pub async fn upload(client: &aws_sdk_s3::Client, object: &str, bucket: &str, body: aws_sdk_s3::primitives::ByteStream) -> Result<PutObjectOutput, PutObjectError> {
    client
        .put_object()
        .bucket(bucket)
        .key(object)
        .body(body)
        .send()
        .await
        .map_err(|e| {
            e.into_service_error()
        })
}

pub async fn download(client: &aws_sdk_s3::Client, object: &str, bucket: &str) -> Result<GetObjectOutput, GetObjectError> {
    client
        .get_object()
        .bucket(bucket)
        .key(object)
        .send()
        .await
        .map_err(|e| {
            e.into_service_error()
        })
}