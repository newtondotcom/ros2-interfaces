use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMapROIReq {
    pub x: f64,
    pub y: f64,
    pub l_x: f64,
    pub l_y: f64,
}

impl Default for GetMapROIReq {
    fn default() -> Self {
        GetMapROIReq {
            x: 0.0,
            y: 0.0,
            l_x: 0.0,
            l_y: 0.0,
        }
    }
}

impl ros2_client::Message for GetMapROIReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMapROIRes {
    pub sub_map: crate::nav_msgs::msg::OccupancyGrid,
}

impl Default for GetMapROIRes {
    fn default() -> Self {
        GetMapROIRes {
            sub_map: crate::nav_msgs::msg::OccupancyGrid::default(),
        }
    }
}

impl ros2_client::Message for GetMapROIRes {}


pub struct GetMapROI;
impl ros2_client::Service for GetMapROI {
    type Request = GetMapROIReq;
    type Response = GetMapROIRes;

    fn request_type_name(&self) -> &str { "GetMapROIReq" }
    fn response_type_name(&self) -> &str { "GetMapROIRes" }
}
