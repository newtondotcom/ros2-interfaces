use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GraspPlanningReq {
    pub group_name: ::std::string::String,
    pub target: crate::moveit_msgs::msg::CollisionObject,
    pub support_surfaces: Vec<::std::string::String>,
    pub candidate_grasps: Vec<crate::moveit_msgs::msg::Grasp>,
    pub movable_obstacles: Vec<crate::moveit_msgs::msg::CollisionObject>,
}

impl Default for GraspPlanningReq {
    fn default() -> Self {
        GraspPlanningReq {
            group_name: ::std::string::String::new(),
            target: crate::moveit_msgs::msg::CollisionObject::default(),
            support_surfaces: Vec::new(),
            candidate_grasps: Vec::new(),
            movable_obstacles: Vec::new(),
        }
    }
}

impl ros2_client::Message for GraspPlanningReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GraspPlanningRes {
    pub grasps: Vec<crate::moveit_msgs::msg::Grasp>,
    pub error_code: crate::moveit_msgs::msg::MoveItErrorCodes,
}

impl Default for GraspPlanningRes {
    fn default() -> Self {
        GraspPlanningRes {
            grasps: Vec::new(),
            error_code: crate::moveit_msgs::msg::MoveItErrorCodes::default(),
        }
    }
}

impl ros2_client::Message for GraspPlanningRes {}


pub struct GraspPlanning;
impl ros2_client::Service for GraspPlanning {
    type Request = GraspPlanningReq;
    type Response = GraspPlanningRes;

    fn request_type_name(&self) -> &str { "GraspPlanningReq" }
    fn response_type_name(&self) -> &str { "GraspPlanningRes" }
}
