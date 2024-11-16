use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MessageIntervalReq {
    pub message_id: u32,
    pub message_rate: f32,
}

impl Default for MessageIntervalReq {
    fn default() -> Self {
        MessageIntervalReq {
            message_id: 0,
            message_rate: 0.0,
        }
    }
}

impl ros2_client::Message for MessageIntervalReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MessageIntervalRes {
    pub success: bool,
}

impl Default for MessageIntervalRes {
    fn default() -> Self {
        MessageIntervalRes {
            success: false,
        }
    }
}

impl ros2_client::Message for MessageIntervalRes {}


pub struct MessageInterval;
impl ros2_client::Service for MessageInterval {
    type Request = MessageIntervalReq;
    type Response = MessageIntervalRes;

    fn request_type_name(&self) -> &str { "MessageIntervalReq" }
    fn response_type_name(&self) -> &str { "MessageIntervalRes" }
}
