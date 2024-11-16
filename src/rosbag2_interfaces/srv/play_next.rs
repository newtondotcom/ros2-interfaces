use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayNextReq {

}

impl Default for PlayNextReq {
    fn default() -> Self {
        PlayNextReq {

        }
    }
}

impl ros2_client::Message for PlayNextReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayNextRes {
    pub success: bool,
}

impl Default for PlayNextRes {
    fn default() -> Self {
        PlayNextRes {
            success: false,
        }
    }
}

impl ros2_client::Message for PlayNextRes {}


pub struct PlayNext;
impl ros2_client::Service for PlayNext {
    type Request = PlayNextReq;
    type Response = PlayNextRes;

    fn request_type_name(&self) -> &str { "PlayNextReq" }
    fn response_type_name(&self) -> &str { "PlayNextRes" }
}
