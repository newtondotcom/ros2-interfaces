use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetAvailableWorldsRequest {
    pub additional_sources: Vec<::std::string::String>,
    pub filter: crate::simulation_interfaces::msg::TagsFilter,
    pub offline_only: bool,
    pub continue_on_error: bool,
}

impl Default for GetAvailableWorldsRequest {
    fn default() -> Self {
        GetAvailableWorldsRequest {
            additional_sources: Vec::new(),
            filter: crate::simulation_interfaces::msg::TagsFilter::default(),
            offline_only: false,
            continue_on_error: false,
        }
    }
}

impl ros2_client::Message for GetAvailableWorldsRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetAvailableWorldsResponse {
    pub result: crate::simulation_interfaces::msg::Result,
    pub worlds: Vec<crate::simulation_interfaces::msg::WorldResource>,
}

impl GetAvailableWorldsResponse {
    pub const DEFAULT_SOURCES_FAILED: u8 = 101;
}

impl Default for GetAvailableWorldsResponse {
    fn default() -> Self {
        GetAvailableWorldsResponse {
            result: crate::simulation_interfaces::msg::Result::default(),
            worlds: Vec::new(),
        }
    }
}

impl ros2_client::Message for GetAvailableWorldsResponse {}


pub struct GetAvailableWorlds;
impl ros2_client::Service for GetAvailableWorlds {
    type Request = GetAvailableWorldsRequest;
    type Response = GetAvailableWorldsResponse;

    fn request_type_name(&self) -> &str { "GetAvailableWorldsRequest" }
    fn response_type_name(&self) -> &str { "GetAvailableWorldsResponse" }
}
