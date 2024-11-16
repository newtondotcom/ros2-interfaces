use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WaypointClearReq {

}

impl Default for WaypointClearReq {
    fn default() -> Self {
        WaypointClearReq {

        }
    }
}

impl ros2_client::Message for WaypointClearReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WaypointClearRes {
    pub success: bool,
}

impl Default for WaypointClearRes {
    fn default() -> Self {
        WaypointClearRes {
            success: false,
        }
    }
}

impl ros2_client::Message for WaypointClearRes {}


pub struct WaypointClear;
impl ros2_client::Service for WaypointClear {
    type Request = WaypointClearReq;
    type Response = WaypointClearRes;

    fn request_type_name(&self) -> &str { "WaypointClearReq" }
    fn response_type_name(&self) -> &str { "WaypointClearRes" }
}
