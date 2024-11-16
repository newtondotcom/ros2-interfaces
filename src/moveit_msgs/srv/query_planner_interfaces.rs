use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QueryPlannerInterfacesReq {

}

impl Default for QueryPlannerInterfacesReq {
    fn default() -> Self {
        QueryPlannerInterfacesReq {

        }
    }
}

impl ros2_client::Message for QueryPlannerInterfacesReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QueryPlannerInterfacesRes {
    pub planner_interfaces: Vec<crate::moveit_msgs::msg::PlannerInterfaceDescription>,
}

impl Default for QueryPlannerInterfacesRes {
    fn default() -> Self {
        QueryPlannerInterfacesRes {
            planner_interfaces: Vec::new(),
        }
    }
}

impl ros2_client::Message for QueryPlannerInterfacesRes {}


pub struct QueryPlannerInterfaces;
impl ros2_client::Service for QueryPlannerInterfaces {
    type Request = QueryPlannerInterfacesReq;
    type Response = QueryPlannerInterfacesRes;

    fn request_type_name(&self) -> &str { "QueryPlannerInterfacesReq" }
    fn response_type_name(&self) -> &str { "QueryPlannerInterfacesRes" }
}
