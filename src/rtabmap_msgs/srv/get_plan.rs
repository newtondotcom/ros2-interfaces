use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetPlanReq {
    pub goal_node: i32,
    pub goal: crate::geometry_msgs::msg::PoseStamped,
    pub tolerance: f32,
}

impl Default for GetPlanReq {
    fn default() -> Self {
        GetPlanReq {
            goal_node: 0,
            goal: crate::geometry_msgs::msg::PoseStamped::default(),
            tolerance: 0.0,
        }
    }
}

impl ros2_client::Message for GetPlanReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetPlanRes {
    pub plan: crate::rtabmap_msgs::msg::Path,
}

impl Default for GetPlanRes {
    fn default() -> Self {
        GetPlanRes {
            plan: crate::rtabmap_msgs::msg::Path::default(),
        }
    }
}

impl ros2_client::Message for GetPlanRes {}


pub struct GetPlan;
impl ros2_client::Service for GetPlan {
    type Request = GetPlanReq;
    type Response = GetPlanRes;

    fn request_type_name(&self) -> &str { "GetPlanReq" }
    fn response_type_name(&self) -> &str { "GetPlanRes" }
}
