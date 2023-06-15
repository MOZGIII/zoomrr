#[derive(Debug)]
pub struct UploadRequest {
    pub download_token: String,
    pub download_url: String,
    pub filename: String,
    pub file_type: String,
}
