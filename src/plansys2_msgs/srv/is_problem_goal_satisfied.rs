use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IsProblemGoalSatisfiedReq {
    pub tree: crate::plansys2_msgs::msg::Tree,
}

impl Default for IsProblemGoalSatisfiedReq {
    fn default() -> Self {
        IsProblemGoalSatisfiedReq {
            tree: crate::plansys2_msgs::msg::Tree::default(),
        }
    }
}

impl ros2_client::Message for IsProblemGoalSatisfiedReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IsProblemGoalSatisfiedRes {
    pub success: bool,
    pub satisfied: bool,
    pub error_info: ::std::string::String,
}

impl Default for IsProblemGoalSatisfiedRes {
    fn default() -> Self {
        IsProblemGoalSatisfiedRes {
            success: false,
            satisfied: false,
            error_info: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for IsProblemGoalSatisfiedRes {}


pub struct IsProblemGoalSatisfied;
impl ros2_client::Service for IsProblemGoalSatisfied {
    type Request = IsProblemGoalSatisfiedReq;
    type Response = IsProblemGoalSatisfiedRes;

    fn request_type_name(&self) -> &str { "IsProblemGoalSatisfiedReq" }
    fn response_type_name(&self) -> &str { "IsProblemGoalSatisfiedRes" }
}
