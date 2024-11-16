use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WaypointPullReq {

}

impl Default for WaypointPullReq {
    fn default() -> Self {
        WaypointPullReq {

        }
    }
}

impl ros2_client::Message for WaypointPullReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WaypointPullRes {
    pub success: bool,
    pub wp_received: u32,
}

impl Default for WaypointPullRes {
    fn default() -> Self {
        WaypointPullRes {
            success: false,
            wp_received: 0,
        }
    }
}

impl ros2_client::Message for WaypointPullRes {}


pub struct WaypointPull;
impl ros2_client::Service for WaypointPull {
    type Request = WaypointPullReq;
    type Response = WaypointPullRes;

    fn request_type_name(&self) -> &str { "WaypointPullReq" }
    fn response_type_name(&self) -> &str { "WaypointPullRes" }
}
