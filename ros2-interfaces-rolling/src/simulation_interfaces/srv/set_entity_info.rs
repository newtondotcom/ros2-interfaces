use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetEntityInfoRequest {
    pub entity: ::std::string::String,
    pub info: crate::simulation_interfaces::msg::EntityInfo,
}

impl Default for SetEntityInfoRequest {
    fn default() -> Self {
        SetEntityInfoRequest {
            entity: ::std::string::String::new(),
            info: crate::simulation_interfaces::msg::EntityInfo::default(),
        }
    }
}

impl ros2_client::Message for SetEntityInfoRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetEntityInfoResponse {
    pub result: crate::simulation_interfaces::msg::Result,
}

impl Default for SetEntityInfoResponse {
    fn default() -> Self {
        SetEntityInfoResponse {
            result: crate::simulation_interfaces::msg::Result::default(),
        }
    }
}

impl ros2_client::Message for SetEntityInfoResponse {}


pub struct SetEntityInfo;
impl ros2_client::Service for SetEntityInfo {
    type Request = SetEntityInfoRequest;
    type Response = SetEntityInfoResponse;

    fn request_type_name(&self) -> &str { "SetEntityInfoRequest" }
    fn response_type_name(&self) -> &str { "SetEntityInfoResponse" }
}
