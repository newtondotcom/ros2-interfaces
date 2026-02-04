use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnloadWorldRequest {

}

impl Default for UnloadWorldRequest {
    fn default() -> Self {
        UnloadWorldRequest {

        }
    }
}

impl ros2_client::Message for UnloadWorldRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnloadWorldResponse {
    pub result: crate::simulation_interfaces::msg::Result,
}

impl UnloadWorldResponse {
    pub const NO_WORLD_LOADED: u8 = 101;
}

impl Default for UnloadWorldResponse {
    fn default() -> Self {
        UnloadWorldResponse {
            result: crate::simulation_interfaces::msg::Result::default(),
        }
    }
}

impl ros2_client::Message for UnloadWorldResponse {}


pub struct UnloadWorld;
impl ros2_client::Service for UnloadWorld {
    type Request = UnloadWorldRequest;
    type Response = UnloadWorldResponse;

    fn request_type_name(&self) -> &str { "UnloadWorldRequest" }
    fn response_type_name(&self) -> &str { "UnloadWorldResponse" }
}
