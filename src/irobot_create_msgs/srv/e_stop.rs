use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EStopReq {
    pub e_stop_on: bool,
}

impl Default for EStopReq {
    fn default() -> Self {
        EStopReq {
            e_stop_on: false,
        }
    }
}

impl ros2_client::Message for EStopReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EStopRes {
    pub success: bool,
    pub message: ::std::string::String,
}

impl Default for EStopRes {
    fn default() -> Self {
        EStopRes {
            success: false,
            message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for EStopRes {}


pub struct EStop;
impl ros2_client::Service for EStop {
    type Request = EStopReq;
    type Response = EStopRes;

    fn request_type_name(&self) -> &str { "EStopReq" }
    fn response_type_name(&self) -> &str { "EStopRes" }
}
