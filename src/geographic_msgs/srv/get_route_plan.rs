use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetRoutePlanReq {
    pub network: crate::unique_identifier_msgs::msg::UUID,
    pub start: crate::unique_identifier_msgs::msg::UUID,
    pub goal: crate::unique_identifier_msgs::msg::UUID,
}

impl Default for GetRoutePlanReq {
    fn default() -> Self {
        GetRoutePlanReq {
            network: crate::unique_identifier_msgs::msg::UUID::default(),
            start: crate::unique_identifier_msgs::msg::UUID::default(),
            goal: crate::unique_identifier_msgs::msg::UUID::default(),
        }
    }
}

impl ros2_client::Message for GetRoutePlanReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetRoutePlanRes {
    pub success: bool,
    pub status: ::std::string::String,
    pub plan: crate::geographic_msgs::msg::RoutePath,
}

impl Default for GetRoutePlanRes {
    fn default() -> Self {
        GetRoutePlanRes {
            success: false,
            status: ::std::string::String::new(),
            plan: crate::geographic_msgs::msg::RoutePath::default(),
        }
    }
}

impl ros2_client::Message for GetRoutePlanRes {}


pub struct GetRoutePlan;
impl ros2_client::Service for GetRoutePlan {
    type Request = GetRoutePlanReq;
    type Response = GetRoutePlanRes;

    fn request_type_name(&self) -> &str { "GetRoutePlanReq" }
    fn response_type_name(&self) -> &str { "GetRoutePlanRes" }
}
