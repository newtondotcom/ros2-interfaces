use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApplyPlanningSceneReq {
    pub scene: crate::moveit_msgs::msg::PlanningScene,
}

impl Default for ApplyPlanningSceneReq {
    fn default() -> Self {
        ApplyPlanningSceneReq {
            scene: crate::moveit_msgs::msg::PlanningScene::default(),
        }
    }
}

impl ros2_client::Message for ApplyPlanningSceneReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApplyPlanningSceneRes {
    pub success: bool,
}

impl Default for ApplyPlanningSceneRes {
    fn default() -> Self {
        ApplyPlanningSceneRes {
            success: false,
        }
    }
}

impl ros2_client::Message for ApplyPlanningSceneRes {}


pub struct ApplyPlanningScene;
impl ros2_client::Service for ApplyPlanningScene {
    type Request = ApplyPlanningSceneReq;
    type Response = ApplyPlanningSceneRes;

    fn request_type_name(&self) -> &str { "ApplyPlanningSceneReq" }
    fn response_type_name(&self) -> &str { "ApplyPlanningSceneRes" }
}
