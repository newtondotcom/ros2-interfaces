use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LogRequestEndReq {

}

impl Default for LogRequestEndReq {
    fn default() -> Self {
        LogRequestEndReq {

        }
    }
}

impl ros2_client::Message for LogRequestEndReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LogRequestEndRes {
    pub success: bool,
}

impl Default for LogRequestEndRes {
    fn default() -> Self {
        LogRequestEndRes {
            success: false,
        }
    }
}

impl ros2_client::Message for LogRequestEndRes {}


pub struct LogRequestEnd;
impl ros2_client::Service for LogRequestEnd {
    type Request = LogRequestEndReq;
    type Response = LogRequestEndRes;

    fn request_type_name(&self) -> &str { "LogRequestEndReq" }
    fn response_type_name(&self) -> &str { "LogRequestEndRes" }
}
