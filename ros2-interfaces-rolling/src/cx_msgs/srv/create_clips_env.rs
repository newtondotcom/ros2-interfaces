use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateClipsEnvRequest {
    pub env_name: ::std::string::String,
}

impl Default for CreateClipsEnvRequest {
    fn default() -> Self {
        CreateClipsEnvRequest {
            env_name: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for CreateClipsEnvRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateClipsEnvResponse {
    pub success: bool,
    pub error: ::std::string::String,
}

impl Default for CreateClipsEnvResponse {
    fn default() -> Self {
        CreateClipsEnvResponse {
            success: false,
            error: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for CreateClipsEnvResponse {}


pub struct CreateClipsEnv;
impl ros2_client::Service for CreateClipsEnv {
    type Request = CreateClipsEnvRequest;
    type Response = CreateClipsEnvResponse;

    fn request_type_name(&self) -> &str { "CreateClipsEnvRequest" }
    fn response_type_name(&self) -> &str { "CreateClipsEnvResponse" }
}
