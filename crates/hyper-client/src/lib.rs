pub type Client =
    hyper::Client<hyper_tls::HttpsConnector<hyper::client::HttpConnector>, hyper::Body>;

pub fn new() -> Client {
    let https = hyper_tls::HttpsConnector::new();
    hyper::Client::builder().build(https)
}

pub use hyper;
pub use hyper_tls;
