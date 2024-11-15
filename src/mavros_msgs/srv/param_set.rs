use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParamSetReq {
    pub param_id: ::std::string::String,
    pub value: crate::mavros_msgs::msg::ParamValue,
}

impl Default for ParamSetReq {
    fn default() -> Self {
        ParamSetReq {
            param_id: ::std::string::String::new(),
            value: crate::mavros_msgs::msg::ParamValue::default(),
        }
    }
}

impl ros2_client::Message for ParamSetReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParamSetRes {
    pub success: bool,
    pub value: crate::mavros_msgs::msg::ParamValue,
}

impl Default for ParamSetRes {
    fn default() -> Self {
        ParamSetRes {
            success: false,
            value: crate::mavros_msgs::msg::ParamValue::default(),
        }
    }
}

impl ros2_client::Message for ParamSetRes {}


pub struct ParamSet;
impl ros2_client::Service for ParamSet {
    type Request = ParamSetReq;
    type Response = ParamSetRes;

    fn request_type_name(&self) -> &str { "ParamSetReq" }
    fn response_type_name(&self) -> &str { "ParamSetRes" }
}
