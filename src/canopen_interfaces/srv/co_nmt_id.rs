use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CONmtIDReq {
    pub nmtcommand: u8,
    pub nodeid: u8,
}

impl Default for CONmtIDReq {
    fn default() -> Self {
        CONmtIDReq {
            nmtcommand: 0,
            nodeid: 0,
        }
    }
}

impl ros2_client::Message for CONmtIDReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CONmtIDRes {
    pub success: bool,
}

impl Default for CONmtIDRes {
    fn default() -> Self {
        CONmtIDRes {
            success: false,
        }
    }
}

impl ros2_client::Message for CONmtIDRes {}


pub struct CONmtID;
impl ros2_client::Service for CONmtID {
    type Request = CONmtIDReq;
    type Response = CONmtIDRes;

    fn request_type_name(&self) -> &str { "CONmtIDReq" }
    fn response_type_name(&self) -> &str { "CONmtIDRes" }
}
