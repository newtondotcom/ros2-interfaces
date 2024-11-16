use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TeleportRelativeReq {
    pub linear: f32,
    pub angular: f32,
}

impl Default for TeleportRelativeReq {
    fn default() -> Self {
        TeleportRelativeReq {
            linear: 0.0,
            angular: 0.0,
        }
    }
}

impl ros2_client::Message for TeleportRelativeReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TeleportRelativeRes {

}

impl Default for TeleportRelativeRes {
    fn default() -> Self {
        TeleportRelativeRes {

        }
    }
}

impl ros2_client::Message for TeleportRelativeRes {}


pub struct TeleportRelative;
impl ros2_client::Service for TeleportRelative {
    type Request = TeleportRelativeReq;
    type Response = TeleportRelativeRes;

    fn request_type_name(&self) -> &str { "TeleportRelativeReq" }
    fn response_type_name(&self) -> &str { "TeleportRelativeRes" }
}
