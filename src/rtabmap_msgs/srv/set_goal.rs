use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetGoalReq {
    pub node_id: i32,
    pub node_label: ::std::string::String,
    pub frame_id: ::std::string::String,
}

impl Default for SetGoalReq {
    fn default() -> Self {
        SetGoalReq {
            node_id: 0,
            node_label: ::std::string::String::new(),
            frame_id: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for SetGoalReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetGoalRes {
    pub path_ids: Vec<i32>,
    pub path_poses: Vec<crate::geometry_msgs::msg::Pose>,
    pub planning_time: f32,
}

impl Default for SetGoalRes {
    fn default() -> Self {
        SetGoalRes {
            path_ids: Vec::new(),
            path_poses: Vec::new(),
            planning_time: 0.0,
        }
    }
}

impl ros2_client::Message for SetGoalRes {}


pub struct SetGoal;
impl ros2_client::Service for SetGoal {
    type Request = SetGoalReq;
    type Response = SetGoalRes;

    fn request_type_name(&self) -> &str { "SetGoalReq" }
    fn response_type_name(&self) -> &str { "SetGoalRes" }
}
