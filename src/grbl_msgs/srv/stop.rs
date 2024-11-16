use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StopReq {
    pub machine_id: ::std::string::String,
    pub command: ::std::string::String,
}

impl Default for StopReq {
    fn default() -> Self {
        StopReq {
            machine_id: ::std::string::String::new(),
            command: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for StopReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StopRes {
    pub success: bool,
}

impl Default for StopRes {
    fn default() -> Self {
        StopRes {
            success: false,
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
