use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParamPushReq {

}

impl Default for ParamPushReq {
    fn default() -> Self {
        ParamPushReq {

        }
    }
}

impl ros2_client::Message for ParamPushReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParamPushRes {
    pub success: bool,
    pub param_transfered: u32,
}

impl Default for ParamPushRes {
    fn default() -> Self {
        ParamPushRes {
            success: false,
            param_transfered: 0,
        }
    }
}

impl ros2_client::Message for ParamPushRes {}


pub struct ParamPush;
impl ros2_client::Service for ParamPush {
    type Request = ParamPushReq;
    type Response = ParamPushRes;

    fn request_type_name(&self) -> &str { "ParamPushReq" }
    fn response_type_name(&self) -> &str { "ParamPushRes" }
}
