use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoadClipsPluginRequest {
    pub env_name: ::std::string::String,
    pub plugin_name: ::std::string::String,
}

impl Default for LoadClipsPluginRequest {
    fn default() -> Self {
        LoadClipsPluginRequest {
            env_name: ::std::string::String::new(),
            plugin_name: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for LoadClipsPluginRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoadClipsPluginResponse {
    pub success: bool,
    pub error: ::std::string::String,
}

impl Default for LoadClipsPluginResponse {
    fn default() -> Self {
        LoadClipsPluginResponse {
            success: false,
            error: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for LoadClipsPluginResponse {}


pub struct LoadClipsPlugin;
impl ros2_client::Service for LoadClipsPlugin {
    type Request = LoadClipsPluginRequest;
    type Response = LoadClipsPluginResponse;

    fn request_type_name(&self) -> &str { "LoadClipsPluginRequest" }
    fn response_type_name(&self) -> &str { "LoadClipsPluginResponse" }
}
