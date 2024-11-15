use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetPlanReq {
    pub domain: ::std::string::String,
    pub problem: ::std::string::String,
}

impl Default for GetPlanReq {
    fn default() -> Self {
        GetPlanReq {
            domain: ::std::string::String::new(),
            problem: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for GetPlanReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetPlanRes {
    pub success: bool,
    pub plan: crate::plansys2_msgs::msg::Plan,
    pub error_info: ::std::string::String,
}

impl Default for GetPlanRes {
    fn default() -> Self {
        GetPlanRes {
            success: false,
            plan: crate::plansys2_msgs::msg::Plan::default(),
            error_info: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for GetPlanRes {}


pub struct GetPlan;
impl ros2_client::Service for GetPlan {
    type Request = GetPlanReq;
    type Response = GetPlanRes;

    fn request_type_name(&self) -> &str { "GetPlanReq" }
    fn response_type_name(&self) -> &str { "GetPlanRes" }
}
