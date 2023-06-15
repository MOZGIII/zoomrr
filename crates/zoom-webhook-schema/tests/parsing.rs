use zoom_webhook_schema::*;

#[test]
fn webhook() {
    let sample = include_bytes!("../../../testdata/samples/recording.completed.json");
    let value: Webhook = serde_json::from_slice(sample).unwrap();
    insta::assert_yaml_snapshot!(value);
}

#[test]
fn recording_completed() {
    let sample = include_bytes!("../../../testdata/samples/recording.completed.json");
    let value: Root<recording::completed::Payload> = serde_json::from_slice(sample).unwrap();
    insta::assert_yaml_snapshot!(value);
}

#[test]
fn endpoint_url_validation() {
    let sample = include_bytes!("../../../testdata/samples/endpoint.url_validation.json");
    let value: Root<endpoint::url_validation::Payload> = serde_json::from_slice(sample).unwrap();
    insta::assert_yaml_snapshot!(value);
}
