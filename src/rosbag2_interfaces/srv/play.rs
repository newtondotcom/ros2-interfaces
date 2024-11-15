use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayReq {
    pub start_offset: crate::builtin_interfaces::msg::Time,
    pub playback_duration: crate::builtin_interfaces::msg::Duration,
    pub playback_until_timestamp: crate::builtin_interfaces::msg::Time,
}

impl Default for PlayReq {
    fn default() -> Self {
        PlayReq {
            start_offset: crate::builtin_interfaces::msg::Time::default(),
            playback_duration: crate::builtin_interfaces::msg::Duration::default(),
            playback_until_timestamp: crate::builtin_interfaces::msg::Time::default(),
        }
    }
}

impl ros2_client::Message for PlayReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayRes {
    pub success: bool,
}

impl Default for PlayRes {
    fn default() -> Self {
        PlayRes {
            success: false,
        }
    }
}

impl ros2_client::Message for PlayRes {}


pub struct Play;
impl ros2_client::Service for Play {
    type Request = PlayReq;
    type Response = PlayRes;

    fn request_type_name(&self) -> &str { "PlayReq" }
    fn response_type_name(&self) -> &str { "PlayRes" }
}
