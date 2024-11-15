use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetUserdataReq {
    pub userdata_key: ::std::string::String,
}

impl Default for GetUserdataReq {
    fn default() -> Self {
        GetUserdataReq {
            userdata_key: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for GetUserdataReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetUserdataRes {
    pub success: bool,
    pub message: ::std::string::String,
    pub userdata: Vec<crate::flexbe_msgs::msg::UserdataInfo>,
}

impl Default for GetUserdataRes {
    fn default() -> Self {
        GetUserdataRes {
            success: false,
            message: ::std::string::String::new(),
            userdata: Vec::new(),
        }
    }
}

impl ros2_client::Message for GetUserdataRes {}


pub struct GetUserdata;
impl ros2_client::Service for GetUserdata {
    type Request = GetUserdataReq;
    type Response = GetUserdataRes;

    fn request_type_name(&self) -> &str { "GetUserdataReq" }
    fn response_type_name(&self) -> &str { "GetUserdataRes" }
}
