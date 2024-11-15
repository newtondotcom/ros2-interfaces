use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MuxAddReq {
    pub topic: ::std::string::String,
}

impl Default for MuxAddReq {
    fn default() -> Self {
        MuxAddReq {
            topic: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for MuxAddReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MuxAddRes {
    pub success: bool,
}

impl Default for MuxAddRes {
    fn default() -> Self {
        MuxAddRes {
            success: false,
        }
    }
}

impl ros2_client::Message for MuxAddRes {}


pub struct MuxAdd;
impl ros2_client::Service for MuxAdd {
    type Request = MuxAddReq;
    type Response = MuxAddRes;

    fn request_type_name(&self) -> &str { "MuxAddReq" }
    fn response_type_name(&self) -> &str { "MuxAddRes" }
}
