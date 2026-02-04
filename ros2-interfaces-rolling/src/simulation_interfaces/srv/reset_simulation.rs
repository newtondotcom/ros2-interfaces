use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResetSimulationRequest {
    pub scope: u8,
}

impl ResetSimulationRequest {
    pub const SCOPE_DEFAULT: u8 = 0;
    pub const SCOPE_TIME: u8 = 1;
    pub const SCOPE_STATE: u8 = 2;
    pub const SCOPE_SPAWNED: u8 = 4;
    pub const SCOPE_ALL: u8 = 255;
}

impl Default for ResetSimulationRequest {
    fn default() -> Self {
        ResetSimulationRequest {
            scope: 0,
        }
    }
}

impl ros2_client::Message for ResetSimulationRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResetSimulationResponse {
    pub result: crate::simulation_interfaces::msg::Result,
}

impl Default for ResetSimulationResponse {
    fn default() -> Self {
        ResetSimulationResponse {
            result: crate::simulation_interfaces::msg::Result::default(),
        }
    }
}

impl ros2_client::Message for ResetSimulationResponse {}


pub struct ResetSimulation;
impl ros2_client::Service for ResetSimulation {
    type Request = ResetSimulationRequest;
    type Response = ResetSimulationResponse;

    fn request_type_name(&self) -> &str { "ResetSimulationRequest" }
    fn response_type_name(&self) -> &str { "ResetSimulationResponse" }
}
