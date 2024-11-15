use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TeleportAbsoluteReq {
    pub x: f32,
    pub y: f32,
    pub theta: f32,
}

impl Default for TeleportAbsoluteReq {
    fn default() -> Self {
        TeleportAbsoluteReq {
            x: 0.0,
            y: 0.0,
            theta: 0.0,
        }
    }
}

impl ros2_client::Message for TeleportAbsoluteReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TeleportAbsoluteRes {

}

impl Default for TeleportAbsoluteRes {
    fn default() -> Self {
        TeleportAbsoluteRes {

        }
    }
}

impl ros2_client::Message for TeleportAbsoluteRes {}


pub struct TeleportAbsolute;
impl ros2_client::Service for TeleportAbsolute {
    type Request = TeleportAbsoluteReq;
    type Response = TeleportAbsoluteRes;

    fn request_type_name(&self) -> &str { "TeleportAbsoluteReq" }
    fn response_type_name(&self) -> &str { "TeleportAbsoluteRes" }
}
