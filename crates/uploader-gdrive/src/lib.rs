pub struct GdriveUploader<Connect, AuthClient> {
    pub client: hyper::Client<Connect>,
    pub scopes: Vec<String>,
    pub authenticator: yup_oauth2::authenticator::Authenticator<AuthClient>,
    pub folder_id: String,
}

#[derive(Debug, thiserror::Error)]
pub enum UploadInitRequestBuildError {
    #[error(transparent)]
    Json(serde_json::Error),
    #[error(transparent)]
    Http(hyper::http::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum UploadError {
    #[error(transparent)]
    Auth(yup_oauth2::Error),
    #[error("unable to obtain token")]
    NoToken,
    #[error(transparent)]
    UploadInitRequestBuild(UploadInitRequestBuildError),
    #[error(transparent)]
    UploadInitRequest(hyper::Error),
    #[error("server rejected upload initialization with status {0}: {1}")]
    UploadInitServer(hyper::StatusCode, String),
    #[error("no upload URL in the resumable upload request")]
    NoUploadUrl,
    #[error("the upload URL was not a valid string: {0}")]
    UploadUrlNonUtf8(hyper::header::ToStrError),
    #[error(transparent)]
    UploadRequestBuild(hyper::http::Error),
    #[error(transparent)]
    UploadRequest(hyper::Error),
    #[error("server rejected upload with status {0}: {1}")]
    UploadServer(hyper::StatusCode, String),
}

impl<Connection, AuthClient> GdriveUploader<Connection, AuthClient> {
    fn build_upload_init_request(
        &self,
        token: &str,
        info: &RequestFile<'_>,
    ) -> Result<hyper::Request<hyper::Body>, UploadInitRequestBuildError> {
        let body = serde_json::to_vec(&info).map_err(UploadInitRequestBuildError::Json)?;
        let body = hyper::Body::from(body);

        hyper::Request::post(
            "https://www.googleapis.com/upload/drive/v3/files?uploadType=resumable&supportsAllDrives=true",
        )
        .header(
            hyper::header::CONTENT_TYPE,
            "application/json; charset=UTF-8",
        )
        .header(hyper::header::AUTHORIZATION, format!("Bearer {token}"))
        .body(body)
        .map_err(UploadInitRequestBuildError::Http)
    }
}

impl<Connect, AuthClient> GdriveUploader<Connect, AuthClient>
where
    AuthClient: hyper::service::Service<hyper::Uri> + Clone + Send + Sync + 'static,
    AuthClient::Response: hyper::client::connect::Connection
        + tokio::io::AsyncRead
        + tokio::io::AsyncWrite
        + Send
        + Sync
        + Unpin
        + 'static,
    AuthClient::Future: Send + Unpin + 'static,
    AuthClient::Error: Into<Box<dyn std::error::Error + Send + Sync>>,
{
    async fn obtain_access_token(&self) -> Result<yup_oauth2::AccessToken, yup_oauth2::Error> {
        self.authenticator.token(&self.scopes).await
    }
}

#[async_trait::async_trait]
impl<Connect, AuthClient> uploader::Uploader for GdriveUploader<Connect, AuthClient>
where
    Connect: hyper::client::connect::Connect + Send + Sync + Clone + 'static,

    AuthClient: hyper::service::Service<hyper::Uri> + Clone + Send + Sync + 'static,
    AuthClient::Response: hyper::client::connect::Connection
        + tokio::io::AsyncRead
        + tokio::io::AsyncWrite
        + Send
        + Sync
        + Unpin
        + 'static,
    AuthClient::Future: Send + Unpin + 'static,
    AuthClient::Error: Into<Box<dyn std::error::Error + Send + Sync>>,
{
    type Error = UploadError;

    async fn upload(
        &self,
        filename: String,
        mime_type: String,
        data: hyper::Body,
    ) -> Result<(), Self::Error> {
        let access_token = self
            .obtain_access_token()
            .await
            .map_err(Self::Error::Auth)?;
        let token = access_token.token().ok_or(Self::Error::NoToken)?;

        let req_file = RequestFile {
            name: &filename,
            mime_type: &mime_type,
            parents: &[&self.folder_id],
        };

        let req = self
            .build_upload_init_request(token, &req_file)
            .map_err(UploadError::UploadInitRequestBuild)?;

        let res = self
            .client
            .request(req)
            .await
            .map_err(UploadError::UploadInitRequest)?;

        let status = res.status();
        if !status.is_success() {
            let body = parse_error_body(res.into_body()).await;
            tracing::debug!(message = "upload init request failed", %status, %body);
            return Err(UploadError::UploadInitServer(status, body));
        }

        let upload_url = res
            .headers()
            .get(hyper::header::LOCATION)
            .ok_or(UploadError::NoUploadUrl)?;
        let upload_url = upload_url.to_str().map_err(UploadError::UploadUrlNonUtf8)?;
        let upload_url = upload_url.to_owned();

        // No need for the response anymore.
        drop(res);

        let req = hyper::Request::put(upload_url)
            .body(data)
            .map_err(UploadError::UploadRequestBuild)?;

        let res = self
            .client
            .request(req)
            .await
            .map_err(UploadError::UploadRequest)?;

        let status = res.status();
        if !status.is_success() {
            let body = parse_error_body(res.into_body()).await;
            tracing::debug!(message = "upload request failed", %status, %body);
            return Err(UploadError::UploadServer(status, body));
        }

        Ok(())
    }
}

async fn parse_error_body(body: hyper::Body) -> String {
    let buf = hyper::body::to_bytes(body).await.unwrap_or_default();
    String::from_utf8_lossy(&buf).into_owned()
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct RequestFile<'a> {
    pub name: &'a str,
    pub parents: &'a [&'a str],
    pub mime_type: &'a str,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct ResponseFile {}
