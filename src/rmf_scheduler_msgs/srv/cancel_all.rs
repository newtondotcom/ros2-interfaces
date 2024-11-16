use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CancelAllReq {
    pub group: ::std::string::String,
}

impl Default for CancelAllReq {
    fn default() -> Self {
        CancelAllReq {
            group: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for CancelAllReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CancelAllRes {
    pub success: bool,
    pub message: ::std::string::String,
}

impl Default for CancelAllRes {
    fn default() -> Self {
        CancelAllRes {
            success: false,
            message: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for CancelAllRes {}


pub struct CancelAll;
impl ros2_client::Service for CancelAll {
    type Request = CancelAllReq;
    type Response = CancelAllRes;

    fn request_type_name(&self) -> &str { "CancelAllReq" }
    fn response_type_name(&self) -> &str { "CancelAllRes" }
}
