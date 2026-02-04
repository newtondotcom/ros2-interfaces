use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StopVisionTrackingRequest {
    pub label: ::std::string::String,
}

impl Default for StopVisionTrackingRequest {
    fn default() -> Self {
        StopVisionTrackingRequest {
            label: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for StopVisionTrackingRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StopVisionTrackingResponse {
    pub success: bool,
}

impl Default for StopVisionTrackingResponse {
    fn default() -> Self {
        StopVisionTrackingResponse {
            success: false,
        }
    }
}

impl ros2_client::Message for StopVisionTrackingResponse {}


pub struct StopVisionTracking;
impl ros2_client::Service for StopVisionTracking {
    type Request = StopVisionTrackingRequest;
    type Response = StopVisionTrackingResponse;

    fn request_type_name(&self) -> &str { "StopVisionTrackingRequest" }
    fn response_type_name(&self) -> &str { "StopVisionTrackingResponse" }
}
