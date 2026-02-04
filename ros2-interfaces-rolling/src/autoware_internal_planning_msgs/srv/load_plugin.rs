use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoadPluginRequest {
    pub plugin_name: ::std::string::String,
}

impl Default for LoadPluginRequest {
    fn default() -> Self {
        LoadPluginRequest {
            plugin_name: ::std::string::String::new(),
        }
    }
}

impl ros2_client::Message for LoadPluginRequest {}



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoadPluginResponse {
    pub success: bool,
}

impl Default for LoadPluginResponse {
    fn default() -> Self {
        LoadPluginResponse {
            success: false,
        }
    }
}

impl ros2_client::Message for LoadPluginResponse {}


pub struct LoadPlugin;
impl ros2_client::Service for LoadPlugin {
    type Request = LoadPluginRequest;
    type Response = LoadPluginResponse;

    fn request_type_name(&self) -> &str { "LoadPluginRequest" }
    fn response_type_name(&self) -> &str { "LoadPluginResponse" }
}
