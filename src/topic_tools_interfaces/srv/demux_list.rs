use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DemuxListReq {

}

impl Default for DemuxListReq {
    fn default() -> Self {
        DemuxListReq {

        }
    }
}

impl ros2_client::Message for DemuxListReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DemuxListRes {
    pub topics: Vec<::std::string::String>,
}

impl Default for DemuxListRes {
    fn default() -> Self {
        DemuxListRes {
            topics: Vec::new(),
        }
    }
}

impl ros2_client::Message for DemuxListRes {}


pub struct DemuxList;
impl ros2_client::Service for DemuxList {
    type Request = DemuxListReq;
    type Response = DemuxListRes;

    fn request_type_name(&self) -> &str { "DemuxListReq" }
    fn response_type_name(&self) -> &str { "DemuxListRes" }
}
