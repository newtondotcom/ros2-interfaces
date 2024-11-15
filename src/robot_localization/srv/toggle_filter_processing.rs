use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ToggleFilterProcessingReq {
    pub on: bool,
}

impl Default for ToggleFilterProcessingReq {
    fn default() -> Self {
        ToggleFilterProcessingReq {
            on: false,
        }
    }
}

impl ros2_client::Message for ToggleFilterProcessingReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ToggleFilterProcessingRes {
    pub status: bool,
}

impl Default for ToggleFilterProcessingRes {
    fn default() -> Self {
        ToggleFilterProcessingRes {
            status: false,
        }
    }
}

impl ros2_client::Message for ToggleFilterProcessingRes {}


pub struct ToggleFilterProcessing;
impl ros2_client::Service for ToggleFilterProcessing {
    type Request = ToggleFilterProcessingReq;
    type Response = ToggleFilterProcessingRes;

    fn request_type_name(&self) -> &str { "ToggleFilterProcessingReq" }
    fn response_type_name(&self) -> &str { "ToggleFilterProcessingRes" }
}
