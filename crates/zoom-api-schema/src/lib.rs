#[derive(Debug)]
pub enum DeleteMeetingRecordingActionParam {
    Delete,
    Trash,
}

#[derive(Debug)]
pub struct DeleteMeetingRecordingParams<'a> {
    pub meeting_id: &'a str,
    pub recording_id: &'a str,
    pub action: Option<DeleteMeetingRecordingActionParam>,
}

impl<'a> std::fmt::Display for DeleteMeetingRecordingParams<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "/meetings/{}/recordings/{}",
            urlencoding::encode(self.meeting_id),
            urlencoding::encode(self.recording_id),
        )?;
        if let Some(ref action) = self.action {
            write!(f, "?action=")?;
            match action {
                DeleteMeetingRecordingActionParam::Delete => write!(f, "delete")?,
                DeleteMeetingRecordingActionParam::Trash => write!(f, "trash")?,
            }
        };
        Ok(())
    }
}
