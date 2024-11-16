use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetParametersAtomicallyReq {
    pub parameters: Vec<crate::rcl_interfaces::msg::Parameter>,
}

impl Default for SetParametersAtomicallyReq {
    fn default() -> Self {
        SetParametersAtomicallyReq {
            parameters: Vec::new(),
        }
    }
}

impl ros2_client::Message for SetParametersAtomicallyReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetParametersAtomicallyRes {
    pub result: crate::rcl_interfaces::msg::SetParametersResult,
}

impl Default for SetParametersAtomicallyRes {
    fn default() -> Self {
        SetParametersAtomicallyRes {
            result: crate::rcl_interfaces::msg::SetParametersResult::default(),
        }
    }
}

impl ros2_client::Message for SetParametersAtomicallyRes {}


pub struct SetParametersAtomically;
impl ros2_client::Service for SetParametersAtomically {
    type Request = SetParametersAtomicallyReq;
    type Response = SetParametersAtomicallyRes;

    fn request_type_name(&self) -> &str { "SetParametersAtomicallyReq" }
    fn response_type_name(&self) -> &str { "SetParametersAtomicallyRes" }
}
