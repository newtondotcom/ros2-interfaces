use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RemoveProblemGoalReq {
    pub request: crate::std_msgs::msg::Empty,
}

impl Default for RemoveProblemGoalReq {
    fn default() -> Self {
        RemoveProblemGoalReq {
            request: crate::std_msgs::msg::Empty::default(),
        }
    }
}

impl ros2_client::Message for RemoveProblemGoalReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RemoveProblemGoalRes {
    pub success: bool,
    pub error_info: ::std::string::String,
}

impl Default for RemoveProblemGoalRes {
    fn default() -> Self {
        RemoveProblemGoalRes {
            success: false,
            error_info: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for RemoveProblemGoalRes {}


pub struct RemoveProblemGoal;
impl ros2_client::Service for RemoveProblemGoal {
    type Request = RemoveProblemGoalReq;
    type Response = RemoveProblemGoalRes;

    fn request_type_name(&self) -> &str { "RemoveProblemGoalReq" }
    fn response_type_name(&self) -> &str { "RemoveProblemGoalRes" }
}
