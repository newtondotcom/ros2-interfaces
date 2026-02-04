use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlanPathRequest {
    pub start: crate::geometry_msgs::msg::Pose,
    pub goal: crate::geometry_msgs::msg::Pose,
    pub from_robot_pose: bool,
}

impl Default for PlanPathRequest {
    fn default() -> Self {
        PlanPathRequest {
            start: crate::geometry_msgs::msg::Pose::default(),
            goal: crate::geometry_msgs::msg::Pose::default(),
            from_robot_pose: false,
        }
    }
}

impl ros2_client::Message for PlanPathRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlanPathResponse {
    pub success: bool,
    pub plan: crate::nav_msgs::msg::Path,
}

impl Default for PlanPathResponse {
    fn default() -> Self {
        PlanPathResponse {
            success: false,
            plan: crate::nav_msgs::msg::Path::default(),
        }
    }
}

impl ros2_client::Message for PlanPathResponse {}


pub struct PlanPath;
impl ros2_client::Service for PlanPath {
    type Request = PlanPathRequest;
    type Response = PlanPathResponse;

    fn request_type_name(&self) -> &str { "PlanPathRequest" }
    fn response_type_name(&self) -> &str { "PlanPathResponse" }
}
