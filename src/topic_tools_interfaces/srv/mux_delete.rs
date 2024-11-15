use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MuxDeleteReq {
    pub topic: ::std::string::String,
}

impl Default for MuxDeleteReq {
    fn default() -> Self {
        MuxDeleteReq {
            topic: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for MuxDeleteReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MuxDeleteRes {
    pub success: bool,
}

impl Default for MuxDeleteRes {
    fn default() -> Self {
        MuxDeleteRes {
            success: false,
        }
    }
}

impl ros2_client::Message for MuxDeleteRes {}


pub struct MuxDelete;
impl ros2_client::Service for MuxDelete {
    type Request = MuxDeleteReq;
    type Response = MuxDeleteRes;

    fn request_type_name(&self) -> &str { "MuxDeleteReq" }
    fn response_type_name(&self) -> &str { "MuxDeleteRes" }
}
