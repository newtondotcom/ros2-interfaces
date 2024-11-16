use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetPlanningSceneReq {
    pub components: crate::moveit_msgs::msg::PlanningSceneComponents,
}

impl Default for GetPlanningSceneReq {
    fn default() -> Self {
        GetPlanningSceneReq {
            components: crate::moveit_msgs::msg::PlanningSceneComponents::default(),
        }
    }
}

impl ros2_client::Message for GetPlanningSceneReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetPlanningSceneRes {
    pub scene: crate::moveit_msgs::msg::PlanningScene,
}

impl Default for GetPlanningSceneRes {
    fn default() -> Self {
        GetPlanningSceneRes {
            scene: crate::moveit_msgs::msg::PlanningScene::default(),
        }
    }
}

impl ros2_client::Message for GetPlanningSceneRes {}


pub struct GetPlanningScene;
impl ros2_client::Service for GetPlanningScene {
    type Request = GetPlanningSceneReq;
    type Response = GetPlanningSceneRes;

    fn request_type_name(&self) -> &str { "GetPlanningSceneReq" }
    fn response_type_name(&self) -> &str { "GetPlanningSceneRes" }
}
