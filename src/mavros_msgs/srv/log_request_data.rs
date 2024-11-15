use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LogRequestDataReq {
    pub id: u16,
    pub offset: u32,
    pub count: u32,
}

impl Default for LogRequestDataReq {
    fn default() -> Self {
        LogRequestDataReq {
            id: 0,
            offset: 0,
            count: 0,
        }
    }
}

impl ros2_client::Message for LogRequestDataReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LogRequestDataRes {
    pub success: bool,
}

impl Default for LogRequestDataRes {
    fn default() -> Self {
        LogRequestDataRes {
            success: false,
        }
    }
}

impl ros2_client::Message for LogRequestDataRes {}


pub struct LogRequestData;
impl ros2_client::Service for LogRequestData {
    type Request = LogRequestDataReq;
    type Response = LogRequestDataRes;

    fn request_type_name(&self) -> &str { "LogRequestDataReq" }
    fn response_type_name(&self) -> &str { "LogRequestDataRes" }
}
