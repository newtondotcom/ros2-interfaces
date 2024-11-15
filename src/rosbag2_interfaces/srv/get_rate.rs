use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetRateReq {

}

impl Default for GetRateReq {
    fn default() -> Self {
        GetRateReq {

        }
    }
}

impl ros2_client::Message for GetRateReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetRateRes {
    pub rate: f64,
}

impl Default for GetRateRes {
    fn default() -> Self {
        GetRateRes {
            rate: 0.0,
        }
    }
}

impl ros2_client::Message for GetRateRes {}


pub struct GetRate;
impl ros2_client::Service for GetRate {
    type Request = GetRateReq;
    type Response = GetRateRes;

    fn request_type_name(&self) -> &str { "GetRateReq" }
    fn response_type_name(&self) -> &str { "GetRateRes" }
}
