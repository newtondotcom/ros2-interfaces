use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetStringReq {
    pub value: ::std::string::String,
}

impl Default for SetStringReq {
    fn default() -> Self {
        SetStringReq {
            value: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for SetStringReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetStringRes {
    pub success: bool,
}

impl Default for SetStringRes {
    fn default() -> Self {
        SetStringRes {
            success: false,
        }
    }
}

impl ros2_client::Message for SetStringRes {}


pub struct SetString;
impl ros2_client::Service for SetString {
    type Request = SetStringReq;
    type Response = SetStringRes;

    fn request_type_name(&self) -> &str { "SetStringReq" }
    fn response_type_name(&self) -> &str { "SetStringRes" }
}
