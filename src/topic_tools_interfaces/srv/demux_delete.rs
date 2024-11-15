use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DemuxDeleteReq {
    pub topic: ::std::string::String,
}

impl Default for DemuxDeleteReq {
    fn default() -> Self {
        DemuxDeleteReq {
            topic: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for DemuxDeleteReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DemuxDeleteRes {
    pub success: bool,
}

impl Default for DemuxDeleteRes {
    fn default() -> Self {
        DemuxDeleteRes {
            success: false,
        }
    }
}

impl ros2_client::Message for DemuxDeleteRes {}


pub struct DemuxDelete;
impl ros2_client::Service for DemuxDelete {
    type Request = DemuxDeleteReq;
    type Response = DemuxDeleteRes;

    fn request_type_name(&self) -> &str { "DemuxDeleteReq" }
    fn response_type_name(&self) -> &str { "DemuxDeleteRes" }
}
