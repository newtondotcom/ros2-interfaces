use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetAvailableTransitionsReq {

}

impl Default for GetAvailableTransitionsReq {
    fn default() -> Self {
        GetAvailableTransitionsReq {

        }
    }
}

impl ros2_client::Message for GetAvailableTransitionsReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetAvailableTransitionsRes {
    pub available_transitions: Vec<crate::lifecycle_msgs::msg::TransitionDescription>,
}

impl Default for GetAvailableTransitionsRes {
    fn default() -> Self {
        GetAvailableTransitionsRes {
            available_transitions: Vec::new(),
        }
    }
}

impl ros2_client::Message for GetAvailableTransitionsRes {}


pub struct GetAvailableTransitions;
impl ros2_client::Service for GetAvailableTransitions {
    type Request = GetAvailableTransitionsReq;
    type Response = GetAvailableTransitionsRes;

    fn request_type_name(&self) -> &str { "GetAvailableTransitionsReq" }
    fn response_type_name(&self) -> &str { "GetAvailableTransitionsRes" }
}
