use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DemuxAddReq {
    pub topic: ::std::string::String,
}

impl Default for DemuxAddReq {
    fn default() -> Self {
        DemuxAddReq {
            topic: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for DemuxAddReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DemuxAddRes {
    pub success: bool,
}

impl Default for DemuxAddRes {
    fn default() -> Self {
        DemuxAddRes {
            success: false,
        }
    }
}

impl ros2_client::Message for DemuxAddRes {}


pub struct DemuxAdd;
impl ros2_client::Service for DemuxAdd {
    type Request = DemuxAddReq;
    type Response = DemuxAddRes;

    fn request_type_name(&self) -> &str { "DemuxAddReq" }
    fn response_type_name(&self) -> &str { "DemuxAddRes" }
}
