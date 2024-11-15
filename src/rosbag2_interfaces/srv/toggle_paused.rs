use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TogglePausedReq {

}

impl Default for TogglePausedReq {
    fn default() -> Self {
        TogglePausedReq {

        }
    }
}

impl ros2_client::Message for TogglePausedReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TogglePausedRes {

}

impl Default for TogglePausedRes {
    fn default() -> Self {
        TogglePausedRes {

        }
    }
}

impl ros2_client::Message for TogglePausedRes {}


pub struct TogglePaused;
impl ros2_client::Service for TogglePaused {
    type Request = TogglePausedReq;
    type Response = TogglePausedRes;

    fn request_type_name(&self) -> &str { "TogglePausedReq" }
    fn response_type_name(&self) -> &str { "TogglePausedRes" }
}
