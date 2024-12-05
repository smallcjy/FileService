mod presigned;
pub use presigned::*;
mod local;
pub use local::*;

/// Generate an S3 client configured with the environment variables
/// 
/// TODO:
/// - [ ] 支持热重载
pub async fn s3_client() -> aws_sdk_s3::Client {
    let s3_endpoint = std::env::var("S3_ENDPOINT").expect("S3_ENDPOINT");
    let s3_region_var = std::env::var("S3_REGION").expect("S3_REGION");

    // AWS_ACCESS_KEY_ID and AWS_SECRET_ACCESS_KEY are used by the AWS SDK for Rust
    std::env::var("AWS_ACCESS_KEY_ID").expect("AWS_ACCESS_KEY_ID");
    std::env::var("AWS_SECRET_ACCESS_KEY").expect("AWS_SECRET_ACCESS_KEY");

    let shared_config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
    let s3_region = aws_sdk_s3::config::Region::new(s3_region_var);
    let conf = aws_sdk_s3::config::Builder::from(&shared_config)
        .endpoint_url(s3_endpoint)
        .region(s3_region)
        .build();

    aws_sdk_s3::Client::from_conf(conf)
}
