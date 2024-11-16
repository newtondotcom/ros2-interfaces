use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetAvailableStatesReq {

}

impl Default for GetAvailableStatesReq {
    fn default() -> Self {
        GetAvailableStatesReq {

        }
    }
}

impl ros2_client::Message for GetAvailableStatesReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetAvailableStatesRes {
    pub available_states: Vec<crate::lifecycle_msgs::msg::State>,
}

impl Default for GetAvailableStatesRes {
    fn default() -> Self {
        GetAvailableStatesRes {
            available_states: Vec::new(),
        }
    }
}

impl ros2_client::Message for GetAvailableStatesRes {}


pub struct GetAvailableStates;
impl ros2_client::Service for GetAvailableStates {
    type Request = GetAvailableStatesReq;
    type Response = GetAvailableStatesRes;

    fn request_type_name(&self) -> &str { "GetAvailableStatesReq" }
    fn response_type_name(&self) -> &str { "GetAvailableStatesRes" }
}
