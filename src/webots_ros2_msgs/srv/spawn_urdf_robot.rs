use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpawnUrdfRobotReq {
    pub robot: crate::webots_ros2_msgs::msg::UrdfRobot,
}

impl Default for SpawnUrdfRobotReq {
    fn default() -> Self {
        SpawnUrdfRobotReq {
            robot: crate::webots_ros2_msgs::msg::UrdfRobot::default(),
        }
    }
}

impl ros2_client::Message for SpawnUrdfRobotReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpawnUrdfRobotRes {
    pub success: bool,
}

impl Default for SpawnUrdfRobotRes {
    fn default() -> Self {
        SpawnUrdfRobotRes {
            success: false,
        }
    }
}

impl ros2_client::Message for SpawnUrdfRobotRes {}


pub struct SpawnUrdfRobot;
impl ros2_client::Service for SpawnUrdfRobot {
    type Request = SpawnUrdfRobotReq;
    type Response = SpawnUrdfRobotRes;

    fn request_type_name(&self) -> &str { "SpawnUrdfRobotReq" }
    fn response_type_name(&self) -> &str { "SpawnUrdfRobotRes" }
}
