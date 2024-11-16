use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct COReadReq {
    pub index: u16,
    pub subindex: u8,
}

impl Default for COReadReq {
    fn default() -> Self {
        COReadReq {
            index: 0,
            subindex: 0,
        }
    }
}

impl ros2_client::Message for COReadReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct COReadRes {
    pub success: bool,
    pub data: u32,
}

impl Default for COReadRes {
    fn default() -> Self {
        COReadRes {
            success: false,
            data: 0,
        }
    }
}

impl ros2_client::Message for COReadRes {}


pub struct CORead;
impl ros2_client::Service for CORead {
    type Request = COReadReq;
    type Response = COReadRes;

    fn request_type_name(&self) -> &str { "COReadReq" }
    fn response_type_name(&self) -> &str { "COReadRes" }
}
