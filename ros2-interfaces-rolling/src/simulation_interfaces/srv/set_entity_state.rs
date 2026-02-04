use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetEntityStateRequest {
    pub entity: ::std::string::String,
    pub state: crate::simulation_interfaces::msg::EntityState,
    pub set_pose: bool,
    pub set_twist: bool,
    pub set_acceleration: bool,
}

impl Default for SetEntityStateRequest {
    fn default() -> Self {
        SetEntityStateRequest {
            entity: ::std::string::String::new(),
            state: crate::simulation_interfaces::msg::EntityState::default(),
            set_pose: false,
            set_twist: false,
            set_acceleration: false,
        }
    }
}

impl ros2_client::Message for SetEntityStateRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetEntityStateResponse {
    pub result: crate::simulation_interfaces::msg::Result,
}

impl SetEntityStateResponse {
    pub const INVALID_POSE: u8 = 101;
}

impl Default for SetEntityStateResponse {
    fn default() -> Self {
        SetEntityStateResponse {
            result: crate::simulation_interfaces::msg::Result::default(),
        }
    }
}

impl ros2_client::Message for SetEntityStateResponse {}


pub struct SetEntityState;
impl ros2_client::Service for SetEntityState {
    type Request = SetEntityStateRequest;
    type Response = SetEntityStateResponse;

    fn request_type_name(&self) -> &str { "SetEntityStateRequest" }
    fn response_type_name(&self) -> &str { "SetEntityStateResponse" }
}
