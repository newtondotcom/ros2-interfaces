use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClearCostmapAroundRobotReq {
    pub reset_distance: f32,
}

impl Default for ClearCostmapAroundRobotReq {
    fn default() -> Self {
        ClearCostmapAroundRobotReq {
            reset_distance: 0.0,
        }
    }
}

impl ros2_client::Message for ClearCostmapAroundRobotReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClearCostmapAroundRobotRes {
    pub response: crate::std_msgs::msg::Empty,
}

impl Default for ClearCostmapAroundRobotRes {
    fn default() -> Self {
        ClearCostmapAroundRobotRes {
            response: crate::std_msgs::msg::Empty::default(),
        }
    }
}

impl ros2_client::Message for ClearCostmapAroundRobotRes {}


pub struct ClearCostmapAroundRobot;
impl ros2_client::Service for ClearCostmapAroundRobot {
    type Request = ClearCostmapAroundRobotReq;
    type Response = ClearCostmapAroundRobotRes;

    fn request_type_name(&self) -> &str { "ClearCostmapAroundRobotReq" }
    fn response_type_name(&self) -> &str { "ClearCostmapAroundRobotRes" }
}
