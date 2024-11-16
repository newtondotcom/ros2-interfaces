use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetRateReq {
    pub rate: f64,
}

impl Default for SetRateReq {
    fn default() -> Self {
        SetRateReq {
            rate: 0.0,
        }
    }
}

impl ros2_client::Message for SetRateReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetRateRes {
    pub success: bool,
}

impl Default for SetRateRes {
    fn default() -> Self {
        SetRateRes {
            success: false,
        }
    }
}

impl ros2_client::Message for SetRateRes {}


pub struct SetRate;
impl ros2_client::Service for SetRate {
    type Request = SetRateReq;
    type Response = SetRateRes;

    fn request_type_name(&self) -> &str { "SetRateReq" }
    fn response_type_name(&self) -> &str { "SetRateRes" }
}
