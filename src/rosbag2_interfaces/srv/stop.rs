use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StopReq {

}

impl Default for StopReq {
    fn default() -> Self {
        StopReq {

        }
    }
}

impl ros2_client::Message for StopReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StopRes {

}

impl Default for StopRes {
    fn default() -> Self {
        StopRes {

        }
    }
}

impl ros2_client::Message for StopRes {}


pub struct Stop;
impl ros2_client::Service for Stop {
    type Request = StopReq;
    type Response = StopRes;

    fn request_type_name(&self) -> &str { "StopReq" }
    fn response_type_name(&self) -> &str { "StopRes" }
}
