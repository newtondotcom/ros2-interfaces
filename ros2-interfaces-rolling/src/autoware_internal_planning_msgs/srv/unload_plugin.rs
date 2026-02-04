use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnloadPluginRequest {
    pub plugin_name: ::std::string::String,
}

impl Default for UnloadPluginRequest {
    fn default() -> Self {
        UnloadPluginRequest {
            plugin_name: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for UnloadPluginRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnloadPluginResponse {
    pub success: bool,
}

impl Default for UnloadPluginResponse {
    fn default() -> Self {
        UnloadPluginResponse {
            success: false,
        }
    }
}

impl ros2_client::Message for UnloadPluginResponse {}


pub struct UnloadPlugin;
impl ros2_client::Service for UnloadPlugin {
    type Request = UnloadPluginRequest;
    type Response = UnloadPluginResponse;

    fn request_type_name(&self) -> &str { "UnloadPluginRequest" }
    fn response_type_name(&self) -> &str { "UnloadPluginResponse" }
}
