use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetSimulatorFeaturesRequest {

}

impl Default for GetSimulatorFeaturesRequest {
    fn default() -> Self {
        GetSimulatorFeaturesRequest {

        }
    }
}

impl ros2_client::Message for GetSimulatorFeaturesRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetSimulatorFeaturesResponse {
    pub features: crate::simulation_interfaces::msg::SimulatorFeatures,
}

impl Default for GetSimulatorFeaturesResponse {
    fn default() -> Self {
        GetSimulatorFeaturesResponse {
            features: crate::simulation_interfaces::msg::SimulatorFeatures::default(),
        }
    }
}

impl ros2_client::Message for GetSimulatorFeaturesResponse {}


pub struct GetSimulatorFeatures;
impl ros2_client::Service for GetSimulatorFeatures {
    type Request = GetSimulatorFeaturesRequest;
    type Response = GetSimulatorFeaturesResponse;

    fn request_type_name(&self) -> &str { "GetSimulatorFeaturesRequest" }
    fn response_type_name(&self) -> &str { "GetSimulatorFeaturesResponse" }
}
