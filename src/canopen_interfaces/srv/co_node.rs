use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CONodeReq {
    pub nodeid: u8,
}

impl Default for CONodeReq {
    fn default() -> Self {
        CONodeReq {
            nodeid: 0,
        }
    }
}

impl ros2_client::Message for CONodeReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CONodeRes {
    pub success: bool,
}

impl Default for CONodeRes {
    fn default() -> Self {
        CONodeRes {
            success: false,
        }
    }
}

impl ros2_client::Message for CONodeRes {}


pub struct CONode;
impl ros2_client::Service for CONode {
    type Request = CONodeReq;
    type Response = CONodeRes;

    fn request_type_name(&self) -> &str { "CONodeReq" }
    fn response_type_name(&self) -> &str { "CONodeRes" }
}
