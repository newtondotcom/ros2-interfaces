use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMotionPlanReq {
    pub motion_plan_request: crate::moveit_msgs::msg::MotionPlanRequest,
}

impl Default for GetMotionPlanReq {
    fn default() -> Self {
        GetMotionPlanReq {
            motion_plan_request: crate::moveit_msgs::msg::MotionPlanRequest::default(),
        }
    }
}

impl ros2_client::Message for GetMotionPlanReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMotionPlanRes {
    pub motion_plan_response: crate::moveit_msgs::msg::MotionPlanResponse,
}

impl Default for GetMotionPlanRes {
    fn default() -> Self {
        GetMotionPlanRes {
            motion_plan_response: crate::moveit_msgs::msg::MotionPlanResponse::default(),
        }
    }
}

impl ros2_client::Message for GetMotionPlanRes {}


pub struct GetMotionPlan;
impl ros2_client::Service for GetMotionPlan {
    type Request = GetMotionPlanReq;
    type Response = GetMotionPlanRes;

    fn request_type_name(&self) -> &str { "GetMotionPlanReq" }
    fn response_type_name(&self) -> &str { "GetMotionPlanRes" }
}
