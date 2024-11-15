use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct COWriteReq {
    pub index: u16,
    pub subindex: u8,
    pub data: u32,
}

impl Default for COWriteReq {
    fn default() -> Self {
        COWriteReq {
            index: 0,
            subindex: 0,
            data: 0,
        }
    }
}

impl ros2_client::Message for COWriteReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct COWriteRes {
    pub success: bool,
}

impl Default for COWriteRes {
    fn default() -> Self {
        COWriteRes {
            success: false,
        }
    }
}

impl ros2_client::Message for COWriteRes {}


pub struct COWrite;
impl ros2_client::Service for COWrite {
    type Request = COWriteReq;
    type Response = COWriteRes;

    fn request_type_name(&self) -> &str { "COWriteReq" }
    fn response_type_name(&self) -> &str { "COWriteRes" }
}
