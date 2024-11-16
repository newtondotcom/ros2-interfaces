use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WaypointPushReq {
    pub start_index: u16,
    pub waypoints: Vec<crate::mavros_msgs::msg::Waypoint>,
}

impl Default for WaypointPushReq {
    fn default() -> Self {
        WaypointPushReq {
            start_index: 0,
            waypoints: Vec::new(),
        }
    }
}

impl ros2_client::Message for WaypointPushReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WaypointPushRes {
    pub success: bool,
    pub wp_transfered: u32,
}

impl Default for WaypointPushRes {
    fn default() -> Self {
        WaypointPushRes {
            success: false,
            wp_transfered: 0,
        }
    }
}

impl ros2_client::Message for WaypointPushRes {}


pub struct WaypointPush;
impl ros2_client::Service for WaypointPush {
    type Request = WaypointPushReq;
    type Response = WaypointPushRes;

    fn request_type_name(&self) -> &str { "WaypointPushReq" }
    fn response_type_name(&self) -> &str { "WaypointPushRes" }
}
