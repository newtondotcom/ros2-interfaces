use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CloseBlackboardStreamReq {
    pub topic_name: ::std::string::String,
}

impl Default for CloseBlackboardStreamReq {
    fn default() -> Self {
        CloseBlackboardStreamReq {
            topic_name: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for CloseBlackboardStreamReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CloseBlackboardStreamRes {
    pub result: bool,
}

impl Default for CloseBlackboardStreamRes {
    fn default() -> Self {
        CloseBlackboardStreamRes {
            result: false,
        }
    }
}

impl ros2_client::Message for CloseBlackboardStreamRes {}


pub struct CloseBlackboardStream;
impl ros2_client::Service for CloseBlackboardStream {
    type Request = CloseBlackboardStreamReq;
    type Response = CloseBlackboardStreamRes;

    fn request_type_name(&self) -> &str { "CloseBlackboardStreamReq" }
    fn response_type_name(&self) -> &str { "CloseBlackboardStreamRes" }
}
