use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetEntityInfoRequest {
    pub entity: ::std::string::String,
}

impl Default for GetEntityInfoRequest {
    fn default() -> Self {
        GetEntityInfoRequest {
            entity: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for GetEntityInfoRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetEntityInfoResponse {
    pub result: crate::simulation_interfaces::msg::Result,
    pub info: crate::simulation_interfaces::msg::EntityInfo,
}

impl Default for GetEntityInfoResponse {
    fn default() -> Self {
        GetEntityInfoResponse {
            result: crate::simulation_interfaces::msg::Result::default(),
            info: crate::simulation_interfaces::msg::EntityInfo::default(),
        }
    }
}

impl ros2_client::Message for GetEntityInfoResponse {}


pub struct GetEntityInfo;
impl ros2_client::Service for GetEntityInfo {
    type Request = GetEntityInfoRequest;
    type Response = GetEntityInfoResponse;

    fn request_type_name(&self) -> &str { "GetEntityInfoRequest" }
    fn response_type_name(&self) -> &str { "GetEntityInfoResponse" }
}
