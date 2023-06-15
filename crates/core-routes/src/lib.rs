use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::post, Json};

pub fn routes(context: Arc<Context>) -> axum::Router {
    axum::Router::new()
        .route("/", post(handle_webhook))
        .with_state(context)
}

pub struct Context {
    pub tx: tokio::sync::mpsc::Sender<core_interface::UploadRequest>,
    pub validator: zoom_webhook_validator::ZoomSignedWebhookValidator,
}

async fn handle_webhook(
    State(context): State<Arc<Context>>,
    Json(body): Json<zoom_webhook_schema::Webhook>,
) -> impl IntoResponse {
    tracing::debug!(message = "got an incoming request", ?body);

    match body {
        zoom_webhook_schema::Webhook::Validation(val) => {
            handle_validation(context, val).await.into_response()
        }
        zoom_webhook_schema::Webhook::Recordings(val) => {
            handle_recordings(context, val).await.into_response()
        }
    }
}

async fn handle_validation(
    context: Arc<Context>,
    body: Box<zoom_webhook_schema::Variant<zoom_webhook_schema::endpoint::url_validation::Payload>>,
) -> Json<zoom_webhook_schema::endpoint::url_validation::Response> {
    let zoom_webhook_schema::Variant {
        payload: zoom_webhook_schema::endpoint::url_validation::Payload { plain_token },
        ..
    } = *body;

    let encrypted_token = context.validator.encrypt(&plain_token);

    let res = zoom_webhook_schema::endpoint::url_validation::Response {
        plain_token,
        encrypted_token,
    };
    Json(res)
}

async fn handle_recordings(
    context: Arc<Context>,
    body: Box<zoom_webhook_schema::Variant<zoom_webhook_schema::recording::completed::Payload>>,
) -> StatusCode {
    let zoom_webhook_schema::Variant {
        payload:
            zoom_webhook_schema::recording::completed::Payload {
                object:
                    zoom_webhook_schema::recording::completed::Object {
                        recording_files, ..
                    },
                ..
            },
        extra_data: zoom_webhook_schema::recording::completed::ExtraData { download_token },
        ..
    } = *body;

    for recording_file in recording_files {
        let zoom_webhook_schema::recording::completed::RecordingFile {
            download_url,
            meeting_id,
            recording_start,
            recording_end,
            file_extension,
            file_type,
            ..
        } = recording_file;

        let filename = format!("{recording_start}-{recording_end}-{meeting_id}.{file_extension}");

        let send_result = context
            .tx
            .send(core_interface::UploadRequest {
                download_url,
                download_token: download_token.clone(),
                filename,
                file_type,
            })
            .await;

        if let Err(error) = send_result {
            tracing::error!(
                message = "unable to submit an upload request into the tx",
                ?error
            );
            return StatusCode::INTERNAL_SERVER_ERROR;
        }
    }

    StatusCode::OK
}
