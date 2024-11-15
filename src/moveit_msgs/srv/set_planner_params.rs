use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetPlannerParamsReq {
    pub pipeline_id: ::std::string::String,
    pub planner_config: ::std::string::String,
    pub group: ::std::string::String,
    pub params: crate::moveit_msgs::msg::PlannerParams,
    pub replace: bool,
}

impl Default for SetPlannerParamsReq {
    fn default() -> Self {
        SetPlannerParamsReq {
            pipeline_id: ::std::string::String::new(),
            planner_config: ::std::string::String::new(),
            group: ::std::string::String::new(),
            params: crate::moveit_msgs::msg::PlannerParams::default(),
            replace: false,
        }
    }
}

impl ros2_client::Message for SetPlannerParamsReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetPlannerParamsRes {

}

impl Default for SetPlannerParamsRes {
    fn default() -> Self {
        SetPlannerParamsRes {

        }
    }
}

impl ros2_client::Message for SetPlannerParamsRes {}


pub struct SetPlannerParams;
impl ros2_client::Service for SetPlannerParams {
    type Request = SetPlannerParamsReq;
    type Response = SetPlannerParamsRes;

    fn request_type_name(&self) -> &str { "SetPlannerParamsReq" }
    fn response_type_name(&self) -> &str { "SetPlannerParamsRes" }
}
