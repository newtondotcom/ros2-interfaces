use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SaveMotionRequest {
    pub motion_key: ::std::string::String,
    pub filepath: ::std::string::String,
}

impl Default for SaveMotionRequest {
    fn default() -> Self {
        SaveMotionRequest {
            motion_key: ::std::string::String::new(),
            filepath: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for SaveMotionRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SaveMotionResponse {
    pub success: bool,
}

impl Default for SaveMotionResponse {
    fn default() -> Self {
        SaveMotionResponse {
            success: false,
        }
    }
}

impl ros2_client::Message for SaveMotionResponse {}


pub struct SaveMotion;
impl ros2_client::Service for SaveMotion {
    type Request = SaveMotionRequest;
    type Response = SaveMotionResponse;

    fn request_type_name(&self) -> &str { "SaveMotionRequest" }
    fn response_type_name(&self) -> &str { "SaveMotionResponse" }
}
