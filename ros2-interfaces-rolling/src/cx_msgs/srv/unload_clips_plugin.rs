use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnloadClipsPluginRequest {
    pub env_name: ::std::string::String,
    pub plugin_name: ::std::string::String,
}

impl Default for UnloadClipsPluginRequest {
    fn default() -> Self {
        UnloadClipsPluginRequest {
            env_name: ::std::string::String::new(),
            plugin_name: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for UnloadClipsPluginRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnloadClipsPluginResponse {
    pub success: bool,
    pub error: ::std::string::String,
}

impl Default for UnloadClipsPluginResponse {
    fn default() -> Self {
        UnloadClipsPluginResponse {
            success: false,
            error: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for UnloadClipsPluginResponse {}


pub struct UnloadClipsPlugin;
impl ros2_client::Service for UnloadClipsPlugin {
    type Request = UnloadClipsPluginRequest;
    type Response = UnloadClipsPluginResponse;

    fn request_type_name(&self) -> &str { "UnloadClipsPluginRequest" }
    fn response_type_name(&self) -> &str { "UnloadClipsPluginResponse" }
}
