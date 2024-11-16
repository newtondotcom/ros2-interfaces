use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MuxListReq {

}

impl Default for MuxListReq {
    fn default() -> Self {
        MuxListReq {

        }
    }
}

impl ros2_client::Message for MuxListReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MuxListRes {
    pub topics: Vec<::std::string::String>,
}

impl Default for MuxListRes {
    fn default() -> Self {
        MuxListRes {
            topics: Vec::new(),
        }
    }
}

impl ros2_client::Message for MuxListRes {}


pub struct MuxList;
impl ros2_client::Service for MuxList {
    type Request = MuxListReq;
    type Response = MuxListRes;

    fn request_type_name(&self) -> &str { "MuxListReq" }
    fn response_type_name(&self) -> &str { "MuxListRes" }
}
