use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetEntityStateRequest {
    pub entity: ::std::string::String,
}

impl Default for GetEntityStateRequest {
    fn default() -> Self {
        GetEntityStateRequest {
            entity: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for GetEntityStateRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetEntityStateResponse {
    pub result: crate::simulation_interfaces::msg::Result,
    pub state: crate::simulation_interfaces::msg::EntityState,
}

impl Default for GetEntityStateResponse {
    fn default() -> Self {
        GetEntityStateResponse {
            result: crate::simulation_interfaces::msg::Result::default(),
            state: crate::simulation_interfaces::msg::EntityState::default(),
        }
    }
}

impl ros2_client::Message for GetEntityStateResponse {}


pub struct GetEntityState;
impl ros2_client::Service for GetEntityState {
    type Request = GetEntityStateRequest;
    type Response = GetEntityStateResponse;

    fn request_type_name(&self) -> &str { "GetEntityStateRequest" }
    fn response_type_name(&self) -> &str { "GetEntityStateResponse" }
}
