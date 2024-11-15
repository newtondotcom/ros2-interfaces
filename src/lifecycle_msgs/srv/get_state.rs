use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetStateReq {

}

impl Default for GetStateReq {
    fn default() -> Self {
        GetStateReq {

        }
    }
}

impl ros2_client::Message for GetStateReq {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetStateRes {
    pub current_state: crate::lifecycle_msgs::msg::State,
}

impl Default for GetStateRes {
    fn default() -> Self {
        GetStateRes {
            current_state: crate::lifecycle_msgs::msg::State::default(),
        }
    }
}

impl ros2_client::Message for GetStateRes {}


pub struct GetState;
impl ros2_client::Service for GetState {
    type Request = GetStateReq;
    type Response = GetStateRes;

    fn request_type_name(&self) -> &str { "GetStateReq" }
    fn response_type_name(&self) -> &str { "GetStateRes" }
}
