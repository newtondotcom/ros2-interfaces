use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetEntityBoundsRequest {
    pub entity: ::std::string::String,
}

impl Default for GetEntityBoundsRequest {
    fn default() -> Self {
        GetEntityBoundsRequest {
            entity: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for GetEntityBoundsRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetEntityBoundsResponse {
    pub result: crate::simulation_interfaces::msg::Result,
    pub bounds: crate::simulation_interfaces::msg::Bounds,
}

impl Default for GetEntityBoundsResponse {
    fn default() -> Self {
        GetEntityBoundsResponse {
            result: crate::simulation_interfaces::msg::Result::default(),
            bounds: crate::simulation_interfaces::msg::Bounds::default(),
        }
    }
}

impl ros2_client::Message for GetEntityBoundsResponse {}


pub struct GetEntityBounds;
impl ros2_client::Service for GetEntityBounds {
    type Request = GetEntityBoundsRequest;
    type Response = GetEntityBoundsResponse;

    fn request_type_name(&self) -> &str { "GetEntityBoundsRequest" }
    fn response_type_name(&self) -> &str { "GetEntityBoundsResponse" }
}
