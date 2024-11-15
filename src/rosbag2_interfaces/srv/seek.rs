use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SeekReq {
    pub time: crate::builtin_interfaces::msg::Time,
}

impl Default for SeekReq {
    fn default() -> Self {
        SeekReq {
            time: crate::builtin_interfaces::msg::Time::default(),
        }
    }
}

impl ros2_client::Message for SeekReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SeekRes {
    pub success: bool,
}

impl Default for SeekRes {
    fn default() -> Self {
        SeekRes {
            success: false,
        }
    }
}

impl ros2_client::Message for SeekRes {}


pub struct Seek;
impl ros2_client::Service for Seek {
    type Request = SeekReq;
    type Response = SeekRes;

    fn request_type_name(&self) -> &str { "SeekReq" }
    fn response_type_name(&self) -> &str { "SeekRes" }
}
