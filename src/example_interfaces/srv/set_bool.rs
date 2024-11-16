use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetBoolReq {
    pub data: bool,
}

impl Default for SetBoolReq {
    fn default() -> Self {
        SetBoolReq {
            data: false,
        }
    }
}

impl ros2_client::Message for SetBoolReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetBoolRes {
    pub success: bool,
    pub message: ::std::string::String,
}

impl Default for SetBoolRes {
    fn default() -> Self {
        SetBoolRes {
            success: false,
            message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for SetBoolRes {}


pub struct SetBool;
impl ros2_client::Service for SetBool {
    type Request = SetBoolReq;
    type Response = SetBoolRes;

    fn request_type_name(&self) -> &str { "SetBoolReq" }
    fn response_type_name(&self) -> &str { "SetBoolRes" }
}
