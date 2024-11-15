use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LIDoutputstateSrvReq {
    pub active: bool,
}

impl Default for LIDoutputstateSrvReq {
    fn default() -> Self {
        LIDoutputstateSrvReq {
            active: false,
        }
    }
}

impl ros2_client::Message for LIDoutputstateSrvReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LIDoutputstateSrvRes {
    pub success: bool,
}

impl Default for LIDoutputstateSrvRes {
    fn default() -> Self {
        LIDoutputstateSrvRes {
            success: false,
        }
    }
}

impl ros2_client::Message for LIDoutputstateSrvRes {}


pub struct LIDoutputstateSrv;
impl ros2_client::Service for LIDoutputstateSrv {
    type Request = LIDoutputstateSrvReq;
    type Response = LIDoutputstateSrvRes;

    fn request_type_name(&self) -> &str { "LIDoutputstateSrvReq" }
    fn response_type_name(&self) -> &str { "LIDoutputstateSrvRes" }
}
