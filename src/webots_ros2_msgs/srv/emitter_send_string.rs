use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EmitterSendStringReq {
    pub value: ::std::string::String,
}

impl Default for EmitterSendStringReq {
    fn default() -> Self {
        EmitterSendStringReq {
            value: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for EmitterSendStringReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EmitterSendStringRes {
    pub result: i64,
}

impl Default for EmitterSendStringRes {
    fn default() -> Self {
        EmitterSendStringRes {
            result: 0,
        }
    }
}

impl ros2_client::Message for EmitterSendStringRes {}


pub struct EmitterSendString;
impl ros2_client::Service for EmitterSendString {
    type Request = EmitterSendStringReq;
    type Response = EmitterSendStringRes;

    fn request_type_name(&self) -> &str { "EmitterSendStringReq" }
    fn response_type_name(&self) -> &str { "EmitterSendStringRes" }
}
