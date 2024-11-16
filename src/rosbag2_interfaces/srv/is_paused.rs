use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IsPausedReq {

}

impl Default for IsPausedReq {
    fn default() -> Self {
        IsPausedReq {

        }
    }
}

impl ros2_client::Message for IsPausedReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IsPausedRes {
    pub paused: bool,
}

impl Default for IsPausedRes {
    fn default() -> Self {
        IsPausedRes {
            paused: false,
        }
    }
}

impl ros2_client::Message for IsPausedRes {}


pub struct IsPaused;
impl ros2_client::Service for IsPaused {
    type Request = IsPausedReq;
    type Response = IsPausedRes;

    fn request_type_name(&self) -> &str { "IsPausedReq" }
    fn response_type_name(&self) -> &str { "IsPausedRes" }
}
