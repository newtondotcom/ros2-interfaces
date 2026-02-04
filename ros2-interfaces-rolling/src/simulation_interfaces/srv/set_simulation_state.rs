use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetSimulationStateRequest {
    pub state: crate::simulation_interfaces::msg::SimulationState,
}

impl Default for SetSimulationStateRequest {
    fn default() -> Self {
        SetSimulationStateRequest {
            state: crate::simulation_interfaces::msg::SimulationState::default(),
        }
    }
}

impl ros2_client::Message for SetSimulationStateRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetSimulationStateResponse {
    pub result: crate::simulation_interfaces::msg::Result,
}

impl SetSimulationStateResponse {
    pub const ALREADY_IN_TARGET_STATE: u8 = 101;
    pub const STATE_TRANSITION_ERROR: u8 = 102;
    pub const INCORRECT_TRANSITION: u8 = 103;
}

impl Default for SetSimulationStateResponse {
    fn default() -> Self {
        SetSimulationStateResponse {
            result: crate::simulation_interfaces::msg::Result::default(),
        }
    }
}

impl ros2_client::Message for SetSimulationStateResponse {}


pub struct SetSimulationState;
impl ros2_client::Service for SetSimulationState {
    type Request = SetSimulationStateRequest;
    type Response = SetSimulationStateResponse;

    fn request_type_name(&self) -> &str { "SetSimulationStateRequest" }
    fn response_type_name(&self) -> &str { "SetSimulationStateResponse" }
}
