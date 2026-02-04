use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoadWorldRequest {
    pub world_resource: crate::simulation_interfaces::msg::Resource,
    pub fail_on_unsupported_element: bool,
    pub ignore_missing_or_unsupported_assets: bool,
}

impl Default for LoadWorldRequest {
    fn default() -> Self {
        LoadWorldRequest {
            world_resource: crate::simulation_interfaces::msg::Resource::default(),
            fail_on_unsupported_element: false,
            ignore_missing_or_unsupported_assets: false,
        }
    }
}

impl ros2_client::Message for LoadWorldRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoadWorldResponse {
    pub result: crate::simulation_interfaces::msg::Result,
    pub world: crate::simulation_interfaces::msg::WorldResource,
}

impl LoadWorldResponse {
    pub const UNSUPPORTED_FORMAT: u8 = 101;
    pub const NO_RESOURCE: u8 = 102;
    pub const RESOURCE_PARSE_ERROR: u8 = 103;
    pub const MISSING_ASSETS: u8 = 104;
    pub const UNSUPPORTED_ASSETS: u8 = 105;
    pub const UNSUPPORTED_ELEMENTS: u8 = 106;
}

impl Default for LoadWorldResponse {
    fn default() -> Self {
        LoadWorldResponse {
            result: crate::simulation_interfaces::msg::Result::default(),
            world: crate::simulation_interfaces::msg::WorldResource::default(),
        }
    }
}

impl ros2_client::Message for LoadWorldResponse {}


pub struct LoadWorld;
impl ros2_client::Service for LoadWorld {
    type Request = LoadWorldRequest;
    type Response = LoadWorldResponse;

    fn request_type_name(&self) -> &str { "LoadWorldRequest" }
    fn response_type_name(&self) -> &str { "LoadWorldResponse" }
}
