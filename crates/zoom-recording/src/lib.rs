pub fn build_request(
    download_url: &str,
    download_token: &str,
) -> Result<hyper::Request<hyper::Body>, hyper::http::Error> {
    hyper::Request::builder()
        .uri(download_url)
        .header(
            hyper::http::header::AUTHORIZATION,
            format!("Bearer {download_token}"),
        )
        .header(hyper::http::header::CONTENT_TYPE, "application/json")
        .body(hyper::Body::empty())
}

#[derive(Debug)]
pub enum RequestError {
    Build(hyper::http::Error),
    Execution(hyper::Error),
}

pub async fn request<Connect>(
    client: &hyper::Client<Connect>,
    download_url: &str,
    download_token: &str,
) -> Result<hyper::Body, RequestError>
where
    Connect: hyper::client::connect::Connect + Clone + Send + Sync + 'static,
{
    let req = build_request(download_url, download_token).map_err(RequestError::Build)?;
    let res = client.request(req).await.map_err(RequestError::Execution)?;
    Ok(res.into_body())
}
