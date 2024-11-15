use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetCostReq {
    pub use_footprint: bool,
    pub x: f32,
    pub y: f32,
    pub theta: f32,
}

impl Default for GetCostReq {
    fn default() -> Self {
        GetCostReq {
            use_footprint: false,
            x: 0.0,
            y: 0.0,
            theta: 0.0,
        }
    }
}

impl ros2_client::Message for GetCostReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetCostRes {
    pub cost: f32,
}

impl Default for GetCostRes {
    fn default() -> Self {
        GetCostRes {
            cost: 0.0,
        }
    }
}

impl ros2_client::Message for GetCostRes {}


pub struct GetCost;
impl ros2_client::Service for GetCost {
    type Request = GetCostReq;
    type Response = GetCostRes;

    fn request_type_name(&self) -> &str { "GetCostReq" }
    fn response_type_name(&self) -> &str { "GetCostRes" }
}
