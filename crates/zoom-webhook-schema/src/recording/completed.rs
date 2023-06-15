use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename = "recording.completed")]
pub struct Payload {
    pub account_id: String,
    pub object: Object,
}

impl crate::WithExtraData for Payload {
    type ExtraData = ExtraData;
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExtraData {
    pub download_token: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Object {
    pub id: i64,
    pub uuid: String,
    pub host_id: String,
    pub account_id: String,
    pub topic: String,
    #[serde(rename = "type")]
    pub type_field: i64,
    pub start_time: String,
    pub password: String,
    pub timezone: String,
    pub host_email: String,
    pub duration: i64,
    pub share_url: String,
    pub total_size: i64,
    pub recording_count: i64,
    pub on_prem: bool,
    pub recording_play_passcode: String,
    pub recording_files: Vec<RecordingFile>,
    pub participant_audio_files: Vec<ParticipantAudioFile>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RecordingFile {
    pub id: String,
    pub meeting_id: String,
    pub recording_start: String,
    pub recording_end: String,
    pub recording_type: String,
    pub file_type: String,
    pub file_size: i64,
    pub file_extension: String,
    pub play_url: String,
    pub download_url: String,
    pub status: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParticipantAudioFile {
    pub id: String,
    pub recording_start: String,
    pub recording_end: String,
    pub file_type: String,
    pub file_name: String,
    pub file_size: i64,
    pub file_extension: String,
    pub play_url: String,
    pub download_url: String,
    pub status: String,
}
