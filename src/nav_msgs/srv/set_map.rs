use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetMapReq {
    pub map: crate::nav_msgs::msg::OccupancyGrid,
    pub initial_pose: crate::geometry_msgs::msg::PoseWithCovarianceStamped,
}

impl Default for SetMapReq {
    fn default() -> Self {
        SetMapReq {
            map: crate::nav_msgs::msg::OccupancyGrid::default(),
            initial_pose: crate::geometry_msgs::msg::PoseWithCovarianceStamped::default(),
        }
    }
}

impl ros2_client::Message for SetMapReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetMapRes {
    pub success: bool,
}

impl Default for SetMapRes {
    fn default() -> Self {
        SetMapRes {
            success: false,
        }
    }
}

impl ros2_client::Message for SetMapRes {}


pub struct SetMap;
impl ros2_client::Service for SetMap {
    type Request = SetMapReq;
    type Response = SetMapRes;

    fn request_type_name(&self) -> &str { "SetMapReq" }
    fn response_type_name(&self) -> &str { "SetMapRes" }
}
