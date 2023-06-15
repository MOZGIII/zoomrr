pub fn file_type_to_mime(file_type: &str) -> Option<&'static str> {
    Some(match file_type {
        "M4A" => "audio/mp4",
        "MP4" => "video/mp4",
        "TXT" => "plain/text",
        _ => return None,
    })
}
