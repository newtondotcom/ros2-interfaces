use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetCurrentWorldRequest {

}

impl Default for GetCurrentWorldRequest {
    fn default() -> Self {
        GetCurrentWorldRequest {

        }
    }
}

impl ros2_client::Message for GetCurrentWorldRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetCurrentWorldResponse {
    pub result: crate::simulation_interfaces::msg::Result,
    pub world: crate::simulation_interfaces::msg::WorldResource,
}

impl GetCurrentWorldResponse {
    pub const NO_WORLD_LOADED: u8 = 101;
}

impl Default for GetCurrentWorldResponse {
    fn default() -> Self {
        GetCurrentWorldResponse {
            result: crate::simulation_interfaces::msg::Result::default(),
            world: crate::simulation_interfaces::msg::WorldResource::default(),
        }
    }
}

impl ros2_client::Message for GetCurrentWorldResponse {}


pub struct GetCurrentWorld;
impl ros2_client::Service for GetCurrentWorld {
    type Request = GetCurrentWorldRequest;
    type Response = GetCurrentWorldResponse;

    fn request_type_name(&self) -> &str { "GetCurrentWorldRequest" }
    fn response_type_name(&self) -> &str { "GetCurrentWorldResponse" }
}
