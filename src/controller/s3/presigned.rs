use aws_sdk_s3::{operation::{get_object::GetObjectError, put_object::PutObjectError}, presigning::{PresignedRequest, PresigningConfig}};

use super::S3_CLIENT;

/// Get a presigned URL for uploading a file to S3
/// 
/// you can get the uri by calling [`uri()`] on the returned [`PresignedRequest`]
/// 
/// [`uri()`]: https://docs.rs/aws-sdk-s3/latest/aws_sdk_s3/presigning/struct.PresignedRequest.html#method.uri
pub async fn get_presigned_download_url(
    object: &str, 
    bucket: &str, 
    expire_in: std::time::Duration,
) -> Result<PresignedRequest, GetObjectError> {
    S3_CLIENT
        .get_object()
        .bucket(bucket)
        .key(object)
        .presigned(
            PresigningConfig::builder()
                .expires_in(expire_in)
                .build()
                .expect("Valid presigning config")
        )
        .await
        .map_err(|e| {
            e.into_service_error()
        })
}

/// Get a presigned URL for uploading a file to S3
/// 
/// you can get the request by calling [`into_http_1x_request()`] on the returned [`PresignedRequest`]
/// 
/// [`into_http_1x_request()`]: https://docs.rs/aws-sdk-s3/latest/aws_sdk_s3/presigning/struct.PresignedRequest.html#method.into_http_1x_request
#[allow(dead_code)]
pub async fn get_presigned_upload_url(
    object: &str, 
    bucket: &str, 
    expire_in: std::time::Duration,
) -> Result<PresignedRequest, PutObjectError> {
    S3_CLIENT
        .put_object()
        .bucket(bucket)
        .key(object)
        .presigned(
            PresigningConfig::builder()
                .expires_in(expire_in)
                .build()
                .expect("Valid presigning config")
        )
        .await
        .map_err(|e| {
            e.into_service_error()
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[actix_rt::test]
    async fn test_get_presigned_download_url() {
        let object = "test.txt";
        let bucket = "test-bucket";
        let expire_in = std::time::Duration::from_secs(60);

        let presigned_request = get_presigned_download_url(object, bucket, expire_in)
            .await
            .expect("Get presigned download url");

        let _ = presigned_request.uri();
        // assert!(uri.contains(object));
        // assert!(uri.contains(bucket));
    }

    #[actix_rt::test]
    async fn test_get_presigned_upload_url() {
        let object = "test.txt";
        let bucket = "test-bucket";
        let expire_in = std::time::Duration::from_secs(60);

        let presigned_request = get_presigned_upload_url(object, bucket, expire_in)
            .await
            .expect("Get presigned upload url");

        let _ = presigned_request.into_http_1x_request(
            aws_sdk_s3::primitives::ByteStream::from(vec![])
        );
    }
}