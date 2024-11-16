use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetParametersReq {
    pub parameters: Vec<crate::rcl_interfaces::msg::Parameter>,
}

impl Default for SetParametersReq {
    fn default() -> Self {
        SetParametersReq {
            parameters: Vec::new(),
        }
    }
}

impl ros2_client::Message for SetParametersReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetParametersRes {
    pub results: Vec<crate::rcl_interfaces::msg::SetParametersResult>,
}

impl Default for SetParametersRes {
    fn default() -> Self {
        SetParametersRes {
            results: Vec::new(),
        }
    }
}

impl ros2_client::Message for SetParametersRes {}


pub struct SetParameters;
impl ros2_client::Service for SetParameters {
    type Request = SetParametersReq;
    type Response = SetParametersRes;

    fn request_type_name(&self) -> &str { "SetParametersReq" }
    fn response_type_name(&self) -> &str { "SetParametersRes" }
}
