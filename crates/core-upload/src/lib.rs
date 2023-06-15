#[derive(Debug)]
pub struct Params<Uploader> {
    pub rx: tokio::sync::mpsc::Receiver<core_interface::UploadRequest>,
    pub recording_client: hyper_client::Client,
    pub uploader: Uploader,
}

pub async fn run<Uploader>(params: Params<Uploader>)
where
    Uploader: uploader::Uploader,
    <Uploader as uploader::Uploader>::Error: std::fmt::Debug,
{
    let Params {
        mut rx,
        recording_client,
        uploader,
    } = params;

    loop {
        let Some(upload_request) = rx.recv().await else {
            break;
        };

        let core_interface::UploadRequest {
            download_token,
            download_url,
            filename,
            file_type,
        } = upload_request;

        let recording_body =
            zoom_recording::request(&recording_client, &download_url, &download_token)
                .await
                .unwrap();

        let mime_type =
            zoom_mime_types::file_type_to_mime(&file_type).unwrap_or("application/octet-stream");

        uploader
            .upload(filename, mime_type.into(), recording_body)
            .await
            .unwrap();
    }
}
