pub struct S3Uploader {
    pub client: aws_sdk_s3::Client,
    pub bucket_name: String,
}

#[async_trait::async_trait]
impl uploader::Uploader for S3Uploader {
    type Error = aws_sdk_s3::Error;

    async fn upload(
        &self,
        filename: String,
        mime_type: String,
        data: hyper::Body,
    ) -> Result<(), Self::Error> {
        let response = self
            .client
            .put_object()
            .bucket(&self.bucket_name)
            .key(filename)
            .content_type(mime_type)
            .body(aws_sdk_s3::primitives::ByteStream::from(data))
            .send()
            .await?;
        tracing::debug!(message = "got response from s3", ?response);
        Ok(())
    }
}
