use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetPlannerParamsReq {
    pub pipeline_id: ::std::string::String,
    pub planner_config: ::std::string::String,
    pub group: ::std::string::String,
}

impl Default for GetPlannerParamsReq {
    fn default() -> Self {
        GetPlannerParamsReq {
            pipeline_id: ::std::string::String::new(),
            planner_config: ::std::string::String::new(),
            group: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for GetPlannerParamsReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetPlannerParamsRes {
    pub params: crate::moveit_msgs::msg::PlannerParams,
}

impl Default for GetPlannerParamsRes {
    fn default() -> Self {
        GetPlannerParamsRes {
            params: crate::moveit_msgs::msg::PlannerParams::default(),
        }
    }
}

impl ros2_client::Message for GetPlannerParamsRes {}


pub struct GetPlannerParams;
impl ros2_client::Service for GetPlannerParams {
    type Request = GetPlannerParamsReq;
    type Response = GetPlannerParamsRes;

    fn request_type_name(&self) -> &str { "GetPlannerParamsReq" }
    fn response_type_name(&self) -> &str { "GetPlannerParamsRes" }
}
