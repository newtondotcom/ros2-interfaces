use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetSpawnablesRequest {
    pub sources: Vec<::std::string::String>,
}

impl Default for GetSpawnablesRequest {
    fn default() -> Self {
        GetSpawnablesRequest {
            sources: Vec::new(),
        }
    }
}

impl ros2_client::Message for GetSpawnablesRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetSpawnablesResponse {
    pub result: crate::simulation_interfaces::msg::Result,
    pub spawnables: Vec<crate::simulation_interfaces::msg::Spawnable>,
}

impl Default for GetSpawnablesResponse {
    fn default() -> Self {
        GetSpawnablesResponse {
            result: crate::simulation_interfaces::msg::Result::default(),
            spawnables: Vec::new(),
        }
    }
}

impl ros2_client::Message for GetSpawnablesResponse {}


pub struct GetSpawnables;
impl ros2_client::Service for GetSpawnables {
    type Request = GetSpawnablesRequest;
    type Response = GetSpawnablesResponse;

    fn request_type_name(&self) -> &str { "GetSpawnablesRequest" }
    fn response_type_name(&self) -> &str { "GetSpawnablesResponse" }
}
