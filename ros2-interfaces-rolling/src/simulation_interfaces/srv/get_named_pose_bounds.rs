use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetNamedPoseBoundsRequest {
    pub name: ::std::string::String,
}

impl Default for GetNamedPoseBoundsRequest {
    fn default() -> Self {
        GetNamedPoseBoundsRequest {
            name: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for GetNamedPoseBoundsRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetNamedPoseBoundsResponse {
    pub result: crate::simulation_interfaces::msg::Result,
    pub bounds: crate::simulation_interfaces::msg::Bounds,
}

impl Default for GetNamedPoseBoundsResponse {
    fn default() -> Self {
        GetNamedPoseBoundsResponse {
            result: crate::simulation_interfaces::msg::Result::default(),
            bounds: crate::simulation_interfaces::msg::Bounds::default(),
        }
    }
}

impl ros2_client::Message for GetNamedPoseBoundsResponse {}


pub struct GetNamedPoseBounds;
impl ros2_client::Service for GetNamedPoseBounds {
    type Request = GetNamedPoseBoundsRequest;
    type Response = GetNamedPoseBoundsResponse;

    fn request_type_name(&self) -> &str { "GetNamedPoseBoundsRequest" }
    fn response_type_name(&self) -> &str { "GetNamedPoseBoundsResponse" }
}
