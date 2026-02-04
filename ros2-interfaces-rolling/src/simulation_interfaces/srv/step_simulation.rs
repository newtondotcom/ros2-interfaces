use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StepSimulationRequest {
    pub steps: u64, // default: 1
}

impl Default for StepSimulationRequest {
    fn default() -> Self {
        StepSimulationRequest {
            steps: 1,
        }
    }
}

impl ros2_client::Message for StepSimulationRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StepSimulationResponse {
    pub result: crate::simulation_interfaces::msg::Result,
}

impl Default for StepSimulationResponse {
    fn default() -> Self {
        StepSimulationResponse {
            result: crate::simulation_interfaces::msg::Result::default(),
        }
    }
}

impl ros2_client::Message for StepSimulationResponse {}


pub struct StepSimulation;
impl ros2_client::Service for StepSimulation {
    type Request = StepSimulationRequest;
    type Response = StepSimulationResponse;

    fn request_type_name(&self) -> &str { "StepSimulationRequest" }
    fn response_type_name(&self) -> &str { "StepSimulationResponse" }
}
