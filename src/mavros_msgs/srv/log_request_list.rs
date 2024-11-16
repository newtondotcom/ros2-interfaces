use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LogRequestListReq {
    pub start: u16,
    pub end: u16,
}

impl Default for LogRequestListReq {
    fn default() -> Self {
        LogRequestListReq {
            start: 0,
            end: 0,
        }
    }
}

impl ros2_client::Message for LogRequestListReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LogRequestListRes {
    pub success: bool,
}

impl Default for LogRequestListRes {
    fn default() -> Self {
        LogRequestListRes {
            success: false,
        }
    }
}

impl ros2_client::Message for LogRequestListRes {}


pub struct LogRequestList;
impl ros2_client::Service for LogRequestList {
    type Request = LogRequestListReq;
    type Response = LogRequestListRes;

    fn request_type_name(&self) -> &str { "LogRequestListReq" }
    fn response_type_name(&self) -> &str { "LogRequestListRes" }
}
