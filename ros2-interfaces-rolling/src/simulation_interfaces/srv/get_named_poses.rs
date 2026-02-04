use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetNamedPosesRequest {
    pub tags: crate::simulation_interfaces::msg::TagsFilter,
}

impl Default for GetNamedPosesRequest {
    fn default() -> Self {
        GetNamedPosesRequest {
            tags: crate::simulation_interfaces::msg::TagsFilter::default(),
        }
    }
}

impl ros2_client::Message for GetNamedPosesRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetNamedPosesResponse {
    pub result: crate::simulation_interfaces::msg::Result,
    pub poses: Vec<crate::simulation_interfaces::msg::NamedPose>,
}

impl Default for GetNamedPosesResponse {
    fn default() -> Self {
        GetNamedPosesResponse {
            result: crate::simulation_interfaces::msg::Result::default(),
            poses: Vec::new(),
        }
    }
}

impl ros2_client::Message for GetNamedPosesResponse {}


pub struct GetNamedPoses;
impl ros2_client::Service for GetNamedPoses {
    type Request = GetNamedPosesRequest;
    type Response = GetNamedPosesResponse;

    fn request_type_name(&self) -> &str { "GetNamedPosesRequest" }
    fn response_type_name(&self) -> &str { "GetNamedPosesResponse" }
}
