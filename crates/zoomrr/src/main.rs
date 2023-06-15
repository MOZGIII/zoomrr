//! Zoom Recordings Replicator.

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    tracing_subscriber::fmt::init();

    let addr = envfury::must("ADDR")?;
    let zoom_webhook_secret_token: String = envfury::must("ZOOM_WEBHOOK_SECRET_TOKEN")?;

    let gdrive_credentials_file: std::path::PathBuf = envfury::must("GDRIVE_CREDENTIALS_FILE")?;
    let gdrive_scopes: String = envfury::must("GDRIVE_SCOPES")?;
    let gdrive_folder_id: String = envfury::must("GDRIVE_FOLDER_ID")?;

    let gdrive_scopes: Vec<_> = gdrive_scopes
        .split(|c| c == ' ' || c == ',')
        .map(ToOwned::to_owned)
        .collect();

    let gdrive_credentials = yup_oauth2::read_service_account_key(gdrive_credentials_file).await?;

    let gdrive_client = hyper_client::new();

    let gdrive_auth = yup_oauth2::ServiceAccountAuthenticator::builder(gdrive_credentials)
        .build()
        .await?;

    let uploader = uploader_gdrive::GdriveUploader {
        client: gdrive_client,
        authenticator: gdrive_auth,
        scopes: gdrive_scopes,
        folder_id: gdrive_folder_id,
    };

    let recording_client = hyper_client::new();

    let (tx, rx) = tokio::sync::mpsc::channel(100);

    let upload_params = core_upload::Params {
        rx,
        recording_client,
        uploader,
    };

    let validator =
        zoom_webhook_validator::ZoomSignedWebhookValidator::new(&zoom_webhook_secret_token);

    let context = core_routes::Context { tx, validator };
    let context = std::sync::Arc::new(context);

    let routes = core_routes::routes(context);

    let token = tokio_util::sync::CancellationToken::new();

    {
        let token = token.clone();
        tokio::spawn(async move {
            core_upload::run(upload_params).await;
            token.cancel();
        });
    }

    axum::Server::bind(&addr)
        .serve(routes.into_make_service())
        .with_graceful_shutdown(token.cancelled())
        .await?;

    Ok(())
}
