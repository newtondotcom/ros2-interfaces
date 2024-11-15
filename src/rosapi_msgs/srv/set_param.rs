use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetParamReq {
    pub name: ::std::string::String,
    pub value: ::std::string::String,
}

impl Default for SetParamReq {
    fn default() -> Self {
        SetParamReq {
            name: ::std::string::String::new(),
            value: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for SetParamReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetParamRes {

}

impl Default for SetParamRes {
    fn default() -> Self {
        SetParamRes {

        }
    }
}

impl ros2_client::Message for SetParamRes {}


pub struct SetParam;
impl ros2_client::Service for SetParam {
    type Request = SetParamReq;
    type Response = SetParamRes;

    fn request_type_name(&self) -> &str { "SetParamReq" }
    fn response_type_name(&self) -> &str { "SetParamRes" }
}
