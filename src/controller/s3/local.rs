#![allow(dead_code)]
use super::S3_CLIENT;
use aws_sdk_s3::operation::{get_object::{GetObjectError, GetObjectOutput}, put_object::{PutObjectError, PutObjectOutput}};

pub async fn upload(object: &str, bucket: &str, body: aws_sdk_s3::primitives::ByteStream) -> Result<PutObjectOutput, PutObjectError> {
    S3_CLIENT
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

pub async fn download(object: &str, bucket: &str) -> Result<GetObjectOutput, GetObjectError> {
    S3_CLIENT
        .get_object()
        .bucket(bucket)
        .key(object)
        .send()
        .await
        .map_err(|e| {
            e.into_service_error()
        })
}