use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParamPullReq {
    pub force_pull: bool,
}

impl Default for ParamPullReq {
    fn default() -> Self {
        ParamPullReq {
            force_pull: false,
        }
    }
}

impl ros2_client::Message for ParamPullReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParamPullRes {
    pub success: bool,
    pub param_received: u32,
}

impl Default for ParamPullRes {
    fn default() -> Self {
        ParamPullRes {
            success: false,
            param_received: 0,
        }
    }
}

impl ros2_client::Message for ParamPullRes {}


pub struct ParamPull;
impl ros2_client::Service for ParamPull {
    type Request = ParamPullReq;
    type Response = ParamPullRes;

    fn request_type_name(&self) -> &str { "ParamPullReq" }
    fn response_type_name(&self) -> &str { "ParamPullRes" }
}
