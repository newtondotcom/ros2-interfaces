use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetEntitiesStatesRequest {
    pub filters: crate::simulation_interfaces::msg::EntityFilters,
}

impl Default for GetEntitiesStatesRequest {
    fn default() -> Self {
        GetEntitiesStatesRequest {
            filters: crate::simulation_interfaces::msg::EntityFilters::default(),
        }
    }
}

impl ros2_client::Message for GetEntitiesStatesRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetEntitiesStatesResponse {
    pub result: crate::simulation_interfaces::msg::Result,
    pub entities: Vec<::std::string::String>,
    pub states: Vec<crate::simulation_interfaces::msg::EntityState>,
}

impl Default for GetEntitiesStatesResponse {
    fn default() -> Self {
        GetEntitiesStatesResponse {
            result: crate::simulation_interfaces::msg::Result::default(),
            entities: Vec::new(),
            states: Vec::new(),
        }
    }
}

impl ros2_client::Message for GetEntitiesStatesResponse {}


pub struct GetEntitiesStates;
impl ros2_client::Service for GetEntitiesStates {
    type Request = GetEntitiesStatesRequest;
    type Response = GetEntitiesStatesResponse;

    fn request_type_name(&self) -> &str { "GetEntitiesStatesRequest" }
    fn response_type_name(&self) -> &str { "GetEntitiesStatesResponse" }
}
