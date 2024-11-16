use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DemuxSelectReq {
    pub topic: ::std::string::String,
}

impl Default for DemuxSelectReq {
    fn default() -> Self {
        DemuxSelectReq {
            topic: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for DemuxSelectReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DemuxSelectRes {
    pub prev_topic: ::std::string::String,
    pub success: bool,
}

impl Default for DemuxSelectRes {
    fn default() -> Self {
        DemuxSelectRes {
            prev_topic: ::std::string::String::new(),
            success: false,
        }
    }
}

impl ros2_client::Message for DemuxSelectRes {}


pub struct DemuxSelect;
impl ros2_client::Service for DemuxSelect {
    type Request = DemuxSelectReq;
    type Response = DemuxSelectRes;

    fn request_type_name(&self) -> &str { "DemuxSelectReq" }
    fn response_type_name(&self) -> &str { "DemuxSelectRes" }
}
