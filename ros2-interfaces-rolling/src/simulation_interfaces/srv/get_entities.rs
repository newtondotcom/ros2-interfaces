use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetEntitiesRequest {
    pub filters: crate::simulation_interfaces::msg::EntityFilters,
}

impl Default for GetEntitiesRequest {
    fn default() -> Self {
        GetEntitiesRequest {
            filters: crate::simulation_interfaces::msg::EntityFilters::default(),
        }
    }
}

impl ros2_client::Message for GetEntitiesRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetEntitiesResponse {
    pub result: crate::simulation_interfaces::msg::Result,
    pub entities: Vec<::std::string::String>,
}

impl Default for GetEntitiesResponse {
    fn default() -> Self {
        GetEntitiesResponse {
            result: crate::simulation_interfaces::msg::Result::default(),
            entities: Vec::new(),
        }
    }
}

impl ros2_client::Message for GetEntitiesResponse {}


pub struct GetEntities;
impl ros2_client::Service for GetEntities {
    type Request = GetEntitiesRequest;
    type Response = GetEntitiesResponse;

    fn request_type_name(&self) -> &str { "GetEntitiesRequest" }
    fn response_type_name(&self) -> &str { "GetEntitiesResponse" }
}
