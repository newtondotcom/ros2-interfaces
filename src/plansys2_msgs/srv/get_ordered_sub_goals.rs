use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetOrderedSubGoalsReq {
    pub request: crate::std_msgs::msg::Empty,
}

impl Default for GetOrderedSubGoalsReq {
    fn default() -> Self {
        GetOrderedSubGoalsReq {
            request: crate::std_msgs::msg::Empty::default(),
        }
    }
}

impl ros2_client::Message for GetOrderedSubGoalsReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetOrderedSubGoalsRes {
    pub success: bool,
    pub sub_goals: Vec<crate::plansys2_msgs::msg::Tree>,
    pub error_info: ::std::string::String,
}

impl Default for GetOrderedSubGoalsRes {
    fn default() -> Self {
        GetOrderedSubGoalsRes {
            success: false,
            sub_goals: Vec::new(),
            error_info: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for GetOrderedSubGoalsRes {}


pub struct GetOrderedSubGoals;
impl ros2_client::Service for GetOrderedSubGoals {
    type Request = GetOrderedSubGoalsReq;
    type Response = GetOrderedSubGoalsRes;

    fn request_type_name(&self) -> &str { "GetOrderedSubGoalsReq" }
    fn response_type_name(&self) -> &str { "GetOrderedSubGoalsRes" }
}
