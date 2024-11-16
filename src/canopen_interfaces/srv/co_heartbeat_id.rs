use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct COHeartbeatIDReq {
    pub nodeid: u8,
    pub heartbeat: u16,
}

impl Default for COHeartbeatIDReq {
    fn default() -> Self {
        COHeartbeatIDReq {
            nodeid: 0,
            heartbeat: 0,
        }
    }
}

impl ros2_client::Message for COHeartbeatIDReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct COHeartbeatIDRes {
    pub success: bool,
}

impl Default for COHeartbeatIDRes {
    fn default() -> Self {
        COHeartbeatIDRes {
            success: false,
        }
    }
}

impl ros2_client::Message for COHeartbeatIDRes {}


pub struct COHeartbeatID;
impl ros2_client::Service for COHeartbeatID {
    type Request = COHeartbeatIDReq;
    type Response = COHeartbeatIDRes;

    fn request_type_name(&self) -> &str { "COHeartbeatIDReq" }
    fn response_type_name(&self) -> &str { "COHeartbeatIDRes" }
}
