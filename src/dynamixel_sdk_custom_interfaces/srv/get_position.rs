use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetPositionReq {
    pub id: u8,
}

impl Default for GetPositionReq {
    fn default() -> Self {
        GetPositionReq {
            id: 0,
        }
    }
}

impl ros2_client::Message for GetPositionReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetPositionRes {
    pub position: i32,
}

impl Default for GetPositionRes {
    fn default() -> Self {
        GetPositionRes {
            position: 0,
        }
    }
}

impl ros2_client::Message for GetPositionRes {}


pub struct GetPosition;
impl ros2_client::Service for GetPosition {
    type Request = GetPositionReq;
    type Response = GetPositionRes;

    fn request_type_name(&self) -> &str { "GetPositionReq" }
    fn response_type_name(&self) -> &str { "GetPositionRes" }
}
