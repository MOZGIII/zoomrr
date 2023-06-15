pub mod recording {
    pub mod completed;
}
pub mod endpoint {
    pub mod url_validation;
}

use serde::{Deserialize, Serialize};

/// Extra data to put at the toplevel.
pub trait WithExtraData {
    type ExtraData: Serialize + for<'de> Deserialize<'de>;
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Root<T: WithExtraData> {
    pub event: String,
    pub event_ts: i64,
    pub payload: T,
    #[serde(flatten)]
    pub extra_data: T::ExtraData,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Variant<T: WithExtraData> {
    pub event_ts: i64,
    pub payload: T,
    #[serde(flatten)]
    pub extra_data: T::ExtraData,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "event")]
pub enum Webhook {
    #[serde(rename = "endpoint.url_validation")]
    Validation(Box<Variant<endpoint::url_validation::Payload>>),
    #[serde(rename = "recording.completed")]
    Recordings(Box<Variant<recording::completed::Payload>>),
}
