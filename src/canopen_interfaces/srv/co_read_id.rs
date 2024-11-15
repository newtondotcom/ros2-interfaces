use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct COReadIDReq {
    pub nodeid: u8,
    pub index: u16,
    pub subindex: u8,
    pub canopen_datatype: u8,
}

impl COReadIDReq {
    pub const CANOPEN_DATATYPE_INT8: u8 = 0x02;
    pub const CANOPEN_DATATYPE_INT16: u8 = 0x03;
    pub const CANOPEN_DATATYPE_INT32: u8 = 0x04;
    pub const CANOPEN_DATATYPE_UINT8: u8 = 0x05;
    pub const CANOPEN_DATATYPE_UINT16: u8 = 0x06;
    pub const CANOPEN_DATATYPE_UINT32: u8 = 0x07;
}

impl Default for COReadIDReq {
    fn default() -> Self {
        COReadIDReq {
            nodeid: 0,
            index: 0,
            subindex: 0,
            canopen_datatype: 0,
        }
    }
}

impl ros2_client::Message for COReadIDReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct COReadIDRes {
    pub success: bool,
    pub data: u32,
}

impl Default for COReadIDRes {
    fn default() -> Self {
        COReadIDRes {
            success: false,
            data: 0,
        }
    }
}

impl ros2_client::Message for COReadIDRes {}


pub struct COReadID;
impl ros2_client::Service for COReadID {
    type Request = COReadIDReq;
    type Response = COReadIDRes;

    fn request_type_name(&self) -> &str { "COReadIDReq" }
    fn response_type_name(&self) -> &str { "COReadIDRes" }
}
