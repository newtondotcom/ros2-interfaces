use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SendBytesReq {
    pub count: i64,
}

impl Default for SendBytesReq {
    fn default() -> Self {
        SendBytesReq {
            count: 0,
        }
    }
}

impl ros2_client::Message for SendBytesReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SendBytesRes {
    pub data: ::std::string::String,
}

impl Default for SendBytesRes {
    fn default() -> Self {
        SendBytesRes {
            data: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for SendBytesRes {}


pub struct SendBytes;
impl ros2_client::Service for SendBytes {
    type Request = SendBytesReq;
    type Response = SendBytesRes;

    fn request_type_name(&self) -> &str { "SendBytesReq" }
    fn response_type_name(&self) -> &str { "SendBytesRes" }
}
