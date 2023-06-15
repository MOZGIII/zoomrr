#[async_trait::async_trait]
pub trait Uploader {
    type Error;

    async fn upload(
        &self,
        filename: String,
        mime_type: String,
        data: hyper::Body,
    ) -> Result<(), Self::Error>;
}
