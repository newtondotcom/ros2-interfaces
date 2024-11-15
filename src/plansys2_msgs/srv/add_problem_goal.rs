use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddProblemGoalReq {
    pub tree: crate::plansys2_msgs::msg::Tree,
}

impl Default for AddProblemGoalReq {
    fn default() -> Self {
        AddProblemGoalReq {
            tree: crate::plansys2_msgs::msg::Tree::default(),
        }
    }
}

impl ros2_client::Message for AddProblemGoalReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddProblemGoalRes {
    pub success: bool,
    pub error_info: ::std::string::String,
}

impl Default for AddProblemGoalRes {
    fn default() -> Self {
        AddProblemGoalRes {
            success: false,
            error_info: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for AddProblemGoalRes {}


pub struct AddProblemGoal;
impl ros2_client::Service for AddProblemGoal {
    type Request = AddProblemGoalReq;
    type Response = AddProblemGoalRes;

    fn request_type_name(&self) -> &str { "AddProblemGoalReq" }
    fn response_type_name(&self) -> &str { "AddProblemGoalRes" }
}
