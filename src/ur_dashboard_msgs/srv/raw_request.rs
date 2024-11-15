use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RawRequestReq {
    pub query: ::std::string::String,
}

impl Default for RawRequestReq {
    fn default() -> Self {
        RawRequestReq {
            query: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for RawRequestReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RawRequestRes {
    pub answer: ::std::string::String,
}

impl Default for RawRequestRes {
    fn default() -> Self {
        RawRequestRes {
            answer: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for RawRequestRes {}


pub struct RawRequest;
impl ros2_client::Service for RawRequest {
    type Request = RawRequestReq;
    type Response = RawRequestRes;

    fn request_type_name(&self) -> &str { "RawRequestReq" }
    fn response_type_name(&self) -> &str { "RawRequestRes" }
}
