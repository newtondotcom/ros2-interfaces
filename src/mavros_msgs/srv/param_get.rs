use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParamGetReq {
    pub param_id: ::std::string::String,
}

impl Default for ParamGetReq {
    fn default() -> Self {
        ParamGetReq {
            param_id: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for ParamGetReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParamGetRes {
    pub success: bool,
    pub value: crate::mavros_msgs::msg::ParamValue,
}

impl Default for ParamGetRes {
    fn default() -> Self {
        ParamGetRes {
            success: false,
            value: crate::mavros_msgs::msg::ParamValue::default(),
        }
    }
}

impl ros2_client::Message for ParamGetRes {}


pub struct ParamGet;
impl ros2_client::Service for ParamGet {
    type Request = ParamGetReq;
    type Response = ParamGetRes;

    fn request_type_name(&self) -> &str { "ParamGetReq" }
    fn response_type_name(&self) -> &str { "ParamGetRes" }
}
