use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetParamReq {
    pub name: ::std::string::String,
    pub default_value: ::std::string::String,
}

impl Default for GetParamReq {
    fn default() -> Self {
        GetParamReq {
            name: ::std::string::String::new(),
            default_value: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for GetParamReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetParamRes {
    pub value: ::std::string::String,
}

impl Default for GetParamRes {
    fn default() -> Self {
        GetParamRes {
            value: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for GetParamRes {}


pub struct GetParam;
impl ros2_client::Service for GetParam {
    type Request = GetParamReq;
    type Response = GetParamRes;

    fn request_type_name(&self) -> &str { "GetParamReq" }
    fn response_type_name(&self) -> &str { "GetParamRes" }
}
