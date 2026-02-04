use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestroyClipsEnvRequest {
    pub env_name: ::std::string::String,
}

impl Default for DestroyClipsEnvRequest {
    fn default() -> Self {
        DestroyClipsEnvRequest {
            env_name: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for DestroyClipsEnvRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestroyClipsEnvResponse {
    pub success: bool,
    pub error: ::std::string::String,
}

impl Default for DestroyClipsEnvResponse {
    fn default() -> Self {
        DestroyClipsEnvResponse {
            success: false,
            error: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for DestroyClipsEnvResponse {}


pub struct DestroyClipsEnv;
impl ros2_client::Service for DestroyClipsEnv {
    type Request = DestroyClipsEnvRequest;
    type Response = DestroyClipsEnvResponse;

    fn request_type_name(&self) -> &str { "DestroyClipsEnvRequest" }
    fn response_type_name(&self) -> &str { "DestroyClipsEnvResponse" }
}
