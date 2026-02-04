use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IsJointListReadyRequest {
    pub joints: Vec<::std::string::String>,
}

impl Default for IsJointListReadyRequest {
    fn default() -> Self {
        IsJointListReadyRequest {
            joints: Vec::new(),
        }
    }
}

impl ros2_client::Message for IsJointListReadyRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IsJointListReadyResponse {
    pub is_ready: bool,
}

impl Default for IsJointListReadyResponse {
    fn default() -> Self {
        IsJointListReadyResponse {
            is_ready: false,
        }
    }
}

impl ros2_client::Message for IsJointListReadyResponse {}


pub struct IsJointListReady;
impl ros2_client::Service for IsJointListReady {
    type Request = IsJointListReadyRequest;
    type Response = IsJointListReadyResponse;

    fn request_type_name(&self) -> &str { "IsJointListReadyRequest" }
    fn response_type_name(&self) -> &str { "IsJointListReadyResponse" }
}
