use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParamSetV2Req {
    pub force_set: bool,
    pub param_id: ::std::string::String,
    pub value: crate::rcl_interfaces::msg::ParameterValue,
}

impl Default for ParamSetV2Req {
    fn default() -> Self {
        ParamSetV2Req {
            force_set: false,
            param_id: ::std::string::String::new(),
            value: crate::rcl_interfaces::msg::ParameterValue::default(),
        }
    }
}

impl ros2_client::Message for ParamSetV2Req {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParamSetV2Res {
    pub success: bool,
    pub value: crate::rcl_interfaces::msg::ParameterValue,
}

impl Default for ParamSetV2Res {
    fn default() -> Self {
        ParamSetV2Res {
            success: false,
            value: crate::rcl_interfaces::msg::ParameterValue::default(),
        }
    }
}

impl ros2_client::Message for ParamSetV2Res {}


pub struct ParamSetV2;
impl ros2_client::Service for ParamSetV2 {
    type Request = ParamSetV2Req;
    type Response = ParamSetV2Res;

    fn request_type_name(&self) -> &str { "ParamSetV2Req" }
    fn response_type_name(&self) -> &str { "ParamSetV2Res" }
}
