use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WaypointSetCurrentReq {
    pub wp_seq: u16,
}

impl Default for WaypointSetCurrentReq {
    fn default() -> Self {
        WaypointSetCurrentReq {
            wp_seq: 0,
        }
    }
}

impl ros2_client::Message for WaypointSetCurrentReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WaypointSetCurrentRes {
    pub success: bool,
}

impl Default for WaypointSetCurrentRes {
    fn default() -> Self {
        WaypointSetCurrentRes {
            success: false,
        }
    }
}

impl ros2_client::Message for WaypointSetCurrentRes {}


pub struct WaypointSetCurrent;
impl ros2_client::Service for WaypointSetCurrent {
    type Request = WaypointSetCurrentReq;
    type Response = WaypointSetCurrentRes;

    fn request_type_name(&self) -> &str { "WaypointSetCurrentReq" }
    fn response_type_name(&self) -> &str { "WaypointSetCurrentRes" }
}
