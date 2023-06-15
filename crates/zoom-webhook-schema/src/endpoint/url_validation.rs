use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "endpoint.url_validation")]
#[serde(rename_all = "camelCase")]
pub struct Payload {
    pub plain_token: String,
}

impl crate::WithExtraData for Payload {
    type ExtraData = ();
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub plain_token: String,
    pub encrypted_token: String,
}
