use uploader::Uploader;

#[tokio::test]
async fn upload() -> Result<(), Box<dyn std::error::Error>> {
    let run_test: bool = envfury::or_else("GDRIVE_TEST_UPLOAD", || false)?;
    if !run_test {
        eprintln!("Skipping google drive uploader test");
        return Ok(());
    }

    let shell_workdir: std::path::PathBuf = envfury::or_parse("PWD", ".")?;
    let gdrive_credentials_file: std::path::PathBuf = envfury::must("GDRIVE_CREDENTIALS_FILE")?;
    let gdrive_scopes: String = envfury::must("GDRIVE_SCOPES")?;
    let gdrive_folder_id: String = envfury::must("GDRIVE_FOLDER_ID")?;

    let gdrive_scopes: Vec<_> = gdrive_scopes
        .split(|c| c == ' ' || c == ',')
        .map(ToOwned::to_owned)
        .collect();

    let gdrive_credentials =
        yup_oauth2::read_service_account_key(shell_workdir.join(gdrive_credentials_file)).await?;

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

    let data = hyper::Body::from("test");

    uploader
        .upload("test".into(), "text/plain".into(), data)
        .await?;

    Ok(())
}
