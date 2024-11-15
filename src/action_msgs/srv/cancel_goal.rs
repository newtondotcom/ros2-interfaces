use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CancelGoalReq {
    pub goal_info: crate::action_msgs::msg::GoalInfo,
}

impl Default for CancelGoalReq {
    fn default() -> Self {
        CancelGoalReq {
            goal_info: crate::action_msgs::msg::GoalInfo::default(),
        }
    }
}

impl ros2_client::Message for CancelGoalReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CancelGoalRes {
    pub return_code: i8,
    pub goals_canceling: Vec<crate::action_msgs::msg::GoalInfo>,
}

impl CancelGoalRes {
    pub const ERROR_NONE: i8 = 0;
    pub const ERROR_REJECTED: i8 = 1;
    pub const ERROR_UNKNOWN_GOAL_ID: i8 = 2;
    pub const ERROR_GOAL_TERMINATED: i8 = 3;
}

impl Default for CancelGoalRes {
    fn default() -> Self {
        CancelGoalRes {
            return_code: 0,
            goals_canceling: Vec::new(),
        }
    }
}

impl ros2_client::Message for CancelGoalRes {}


pub struct CancelGoal;
impl ros2_client::Service for CancelGoal {
    type Request = CancelGoalReq;
    type Response = CancelGoalRes;

    fn request_type_name(&self) -> &str { "CancelGoalReq" }
    fn response_type_name(&self) -> &str { "CancelGoalRes" }
}
