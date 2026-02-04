use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetSimulationStateRequest {

}

impl Default for GetSimulationStateRequest {
    fn default() -> Self {
        GetSimulationStateRequest {

        }
    }
}

impl ros2_client::Message for GetSimulationStateRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetSimulationStateResponse {
    pub state: crate::simulation_interfaces::msg::SimulationState,
    pub result: crate::simulation_interfaces::msg::Result,
}

impl Default for GetSimulationStateResponse {
    fn default() -> Self {
        GetSimulationStateResponse {
            state: crate::simulation_interfaces::msg::SimulationState::default(),
            result: crate::simulation_interfaces::msg::Result::default(),
        }
    }
}

impl ros2_client::Message for GetSimulationStateResponse {}


pub struct GetSimulationState;
impl ros2_client::Service for GetSimulationState {
    type Request = GetSimulationStateRequest;
    type Response = GetSimulationStateResponse;

    fn request_type_name(&self) -> &str { "GetSimulationStateRequest" }
    fn response_type_name(&self) -> &str { "GetSimulationStateResponse" }
}
