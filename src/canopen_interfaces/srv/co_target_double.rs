use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct COTargetDoubleReq {
    pub target: f64,
}

impl Default for COTargetDoubleReq {
    fn default() -> Self {
        COTargetDoubleReq {
            target: 0.0,
        }
    }
}

impl ros2_client::Message for COTargetDoubleReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct COTargetDoubleRes {
    pub success: bool,
}

impl Default for COTargetDoubleRes {
    fn default() -> Self {
        COTargetDoubleRes {
            success: false,
        }
    }
}

impl ros2_client::Message for COTargetDoubleRes {}


pub struct COTargetDouble;
impl ros2_client::Service for COTargetDouble {
    type Request = COTargetDoubleReq;
    type Response = COTargetDoubleRes;

    fn request_type_name(&self) -> &str { "COTargetDoubleReq" }
    fn response_type_name(&self) -> &str { "COTargetDoubleRes" }
}
