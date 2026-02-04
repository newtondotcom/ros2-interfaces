use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StartPathRecordingRequest {
    pub recording_time_step: f64,
}

impl Default for StartPathRecordingRequest {
    fn default() -> Self {
        StartPathRecordingRequest {
            recording_time_step: 0.0,
        }
    }
}

impl ros2_client::Message for StartPathRecordingRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StartPathRecordingResponse {
    pub started_recording: bool,
    pub message: ::std::string::String,
}

impl Default for StartPathRecordingResponse {
    fn default() -> Self {
        StartPathRecordingResponse {
            started_recording: false,
            message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for StartPathRecordingResponse {}


pub struct StartPathRecording;
impl ros2_client::Service for StartPathRecording {
    type Request = StartPathRecordingRequest;
    type Response = StartPathRecordingResponse;

    fn request_type_name(&self) -> &str { "StartPathRecordingRequest" }
    fn response_type_name(&self) -> &str { "StartPathRecordingResponse" }
}
