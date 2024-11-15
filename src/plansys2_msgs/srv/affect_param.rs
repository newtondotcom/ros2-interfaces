use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AffectParamReq {
    pub param: crate::plansys2_msgs::msg::Param,
}

impl Default for AffectParamReq {
    fn default() -> Self {
        AffectParamReq {
            param: crate::plansys2_msgs::msg::Param::default(),
        }
    }
}

impl ros2_client::Message for AffectParamReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AffectParamRes {
    pub success: bool,
    pub error_info: ::std::string::String,
}

impl Default for AffectParamRes {
    fn default() -> Self {
        AffectParamRes {
            success: false,
            error_info: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for AffectParamRes {}


pub struct AffectParam;
impl ros2_client::Service for AffectParam {
    type Request = AffectParamReq;
    type Response = AffectParamRes;

    fn request_type_name(&self) -> &str { "AffectParamReq" }
    fn response_type_name(&self) -> &str { "AffectParamRes" }
}
