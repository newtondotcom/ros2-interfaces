use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PauseReq {

}

impl Default for PauseReq {
    fn default() -> Self {
        PauseReq {

        }
    }
}

impl ros2_client::Message for PauseReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PauseRes {

}

impl Default for PauseRes {
    fn default() -> Self {
        PauseRes {

        }
    }
}

impl ros2_client::Message for PauseRes {}


pub struct Pause;
impl ros2_client::Service for Pause {
    type Request = PauseReq;
    type Response = PauseRes;

    fn request_type_name(&self) -> &str { "PauseReq" }
    fn response_type_name(&self) -> &str { "PauseRes" }
}
