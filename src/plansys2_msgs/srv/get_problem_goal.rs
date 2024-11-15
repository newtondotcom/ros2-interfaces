use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetProblemGoalReq {
    pub request: crate::std_msgs::msg::Empty,
}

impl Default for GetProblemGoalReq {
    fn default() -> Self {
        GetProblemGoalReq {
            request: crate::std_msgs::msg::Empty::default(),
        }
    }
}

impl ros2_client::Message for GetProblemGoalReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetProblemGoalRes {
    pub success: bool,
    pub tree: crate::plansys2_msgs::msg::Tree,
    pub error_info: ::std::string::String,
}

impl Default for GetProblemGoalRes {
    fn default() -> Self {
        GetProblemGoalRes {
            success: false,
            tree: crate::plansys2_msgs::msg::Tree::default(),
            error_info: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for GetProblemGoalRes {}


pub struct GetProblemGoal;
impl ros2_client::Service for GetProblemGoal {
    type Request = GetProblemGoalReq;
    type Response = GetProblemGoalRes;

    fn request_type_name(&self) -> &str { "GetProblemGoalReq" }
    fn response_type_name(&self) -> &str { "GetProblemGoalRes" }
}
